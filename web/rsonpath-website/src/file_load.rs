use crate::message::*;
use crate::util::AtomicF32;
use egui_async::Bind;
use eyre::Result;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::task::Poll;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker, window};
use web_time::Duration;

#[derive(Clone)]
pub struct FileLoad(Rc<Mutex<FileLoadImpl>>);

struct FileLoadImpl {
    file_read_bind: Bind<(), String>,
    state: FileLoadState,
}

pub enum FileLoadState {
    Idle,
    Requested(FileLoadRequested),
    InProgress(FileLoadInProgress),
    Succeeded(FileLoaded),
    Failed(FileLoadFailed),
    None,
}

pub struct FileLoadRequested {
    file: web_sys::File,
}

pub struct FileLoadInProgress {
    file: web_sys::File,
    progress: Rc<AtomicF32>,
    finished: Rc<RefCell<Option<FileLoadFinished>>>,
    start: f64,
}

enum FileLoadFinished {
    Success(LoadFileSuccessMessage),
    Failure(LoadFileFailureMessage),
}

pub struct FileLoaded {
    file: web_sys::File,
    preview: String,
    elapsed: f64,
}

pub struct FileLoadFailed {
    file: web_sys::File,
    error: String,
    elapsed: f64,
}

impl FileLoad {
    pub fn new() -> Self {
        Self(Rc::new(Mutex::new(FileLoadImpl {
            file_read_bind: Bind::new(true),
            state: FileLoadState::Idle,
        })))
    }

    pub fn request_async_load(&self, file: web_sys::File, gui_ctx: egui::Context, worker: Worker) {
        let future = FileLoadFuture::start(self.clone(), file, gui_ctx, worker);
        self.0.lock().unwrap().file_read_bind.request(future);
    }

    pub fn discard(&self, gui_ctx: &egui::Context, worker: &Worker) {
        let mut lock = self.0.lock().unwrap();
        lock.state = FileLoadState::Idle;
        lock.file_read_bind.clear();
        let msg = DiscardFileMessage::new();
        worker.send(msg).expect("send DiscardFileMessage to worker to succeed");
        gui_ctx.request_repaint();
    }

    pub fn with_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&FileLoadState) -> R,
    {
        let state = &self.0.lock().unwrap().state;
        f(state)
    }
}

pub struct FileLoadFuture {
    worker: Worker,
    file_load: FileLoad,
    gui_ctx: egui::Context,
}

impl FileLoadFuture {
    fn start(file_load: FileLoad, file: web_sys::File, gui_ctx: egui::Context, worker: Worker) -> Self {
        {
            let mut inner = file_load.0.lock().unwrap();
            inner.state = FileLoadState::Requested(FileLoadRequested { file });
        }

        Self {
            worker,
            gui_ctx,
            file_load,
        }
    }
}

impl FileLoadInProgress {
    pub fn progress(&self) -> f32 {
        self.progress.get(Ordering::Acquire)
    }
}

impl FileLoadFailed {
    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_secs_f64(self.elapsed * 0.001)
    }

    pub fn file(&self) -> &web_sys::File {
        &self.file
    }
}

impl FileLoaded {
    pub fn preview(&self) -> &str {
        &self.preview
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_secs_f64(self.elapsed * 0.001)
    }

    pub fn file(&self) -> &web_sys::File {
        &self.file
    }
}

impl Future for FileLoadFuture {
    type Output = Result<(), String>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        use web_sys::console;
        let mut file_load = self.file_load.0.lock().unwrap();
        let mut state = FileLoadState::None;
        std::mem::swap(&mut file_load.state, &mut state);
        let state = state;
        let (state, response) = match state {
            FileLoadState::Idle => (FileLoadState::Idle, Poll::Ready(Ok(()))),
            FileLoadState::Requested(request) => {
                console::log_1(&"Requested, starting...".into());
                let start = window().unwrap().performance().unwrap().now();
                let progress = Rc::new(AtomicF32::new(0.0));
                let finished = Rc::new(RefCell::new(None));
                let progress_clone = progress.clone();
                let finished_clone = finished.clone();
                let waker = cx.waker().clone();
                let gui_ctx = self.gui_ctx.clone();
                let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| match msg.ty() {
                    MessageType::LoadFileProgress => {
                        let msg = msg
                            .deserialize::<LoadFileProgressMessage>()
                            .expect("LoadFileProgressMessage to be correct");
                        progress_clone.store(msg.progress(), Ordering::Release);
                        gui_ctx.request_repaint();
                    }
                    MessageType::LoadFileSuccess => {
                        let msg = msg
                            .deserialize::<LoadFileSuccessMessage>()
                            .expect("LoadFileFinishedMessage to be correct");
                        finished_clone.borrow_mut().replace(FileLoadFinished::Success(msg));
                        progress_clone.store(1.0, Ordering::Release);
                        waker.wake_by_ref();
                        gui_ctx.request_repaint();
                    }
                    MessageType::LoadFileFailure => {
                        let msg = msg
                            .deserialize::<LoadFileFailureMessage>()
                            .expect("LoadFileFailureMessage to be correct");
                        finished_clone.borrow_mut().replace(FileLoadFinished::Failure(msg));
                        progress_clone.store(1.0, Ordering::Release);
                        waker.wake_by_ref();
                        gui_ctx.request_repaint();
                    }
                    _ => (),
                }) as Box<dyn Fn(MessageEvent)>);
                self.worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                self.worker
                    .send(LoadFileMessage::new(request.file.clone()))
                    .expect("LoadFileMessage sent to worker");

                (
                    FileLoadState::InProgress(FileLoadInProgress {
                        file: request.file,
                        progress,
                        finished,
                        start,
                    }),
                    Poll::Pending,
                )
            }
            FileLoadState::InProgress(in_progress) => {
                if let Some(finished) = in_progress.finished.take() {
                    console::log_1(&"We are done!".into());
                    let now = window().unwrap().performance().unwrap().now();
                    let elapsed = now - in_progress.start;
                    self.gui_ctx.request_repaint();
                    match finished {
                        FileLoadFinished::Success(success) => (
                            FileLoadState::Succeeded(FileLoaded {
                                file: in_progress.file,
                                preview: success.into_preview(),
                                elapsed,
                            }),
                            Poll::Ready(Ok(())),
                        ),
                        FileLoadFinished::Failure(failure) => (
                            FileLoadState::Failed(FileLoadFailed {
                                file: in_progress.file,
                                error: failure.into_error(),
                                elapsed,
                            }),
                            Poll::Ready(Ok(())),
                        ),
                    }
                } else {
                    console::log_1(&"We are not done.".into());
                    (FileLoadState::InProgress(in_progress), Poll::Pending)
                }
            }
            FileLoadState::Succeeded(loaded) => (FileLoadState::Succeeded(loaded), Poll::Ready(Ok(()))),
            FileLoadState::Failed(loaded) => (FileLoadState::Failed(loaded), Poll::Ready(Ok(()))),
            FileLoadState::None => unreachable!("FileLoadState::None cannot happen"),
        };

        file_load.state = state;
        response
    }
}
