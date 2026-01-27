use crate::message::{
    GenericMessage, MessageChannel, MessageType, RsonpathRuntime, RunRsonpathFailureMessage, RunRsonpathMessage,
    RunRsonpathSuccessMessage,
};
use egui_async::Bind;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{MessageEvent, Worker};

#[derive(Clone)]
pub struct EngineRun(Rc<Mutex<EngineRunImpl>>);

struct EngineRunImpl {
    engine_run_bind: Bind<(), String>,
    state: EngineRunState,
}

pub enum EngineRunState {
    Idle,
    Requested(EngineRunRequested),
    InProgress(EngineRunInProgress),
    Succeeded(EngineRunResult),
    Failed(EngineRunFailure),
    None,
}

enum EngineRunFinished {
    Success(RunRsonpathSuccessMessage),
    Failure(RunRsonpathFailureMessage),
}

pub struct EngineRunRequested {
    msg: RunRsonpathMessage,
}

pub struct EngineRunInProgress {
    finished: Rc<RefCell<Option<EngineRunFinished>>>,
}

pub struct EngineRunResult {
    runtime: RsonpathRuntime,
    results: String,
}

pub struct EngineRunFailure {
    error: String,
}

impl EngineRunResult {
    pub fn runtime(&self) -> &RsonpathRuntime {
        &self.runtime
    }

    pub fn results(&self) -> &str {
        &self.results
    }
}

impl EngineRunFailure {
    pub fn error(&self) -> &str {
        &self.error
    }
}

impl EngineRun {
    pub fn new() -> Self {
        Self(Rc::new(Mutex::new(EngineRunImpl {
            engine_run_bind: Bind::new(true),
            state: EngineRunState::Idle,
        })))
    }

    pub fn request_async_run(&self, msg: RunRsonpathMessage, gui_ctx: egui::Context, worker: Worker) {
        let future = EngineRunFuture::start(msg, self.clone(), gui_ctx, worker);
        self.0.lock().unwrap().engine_run_bind.request(future);
    }

    pub fn with_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&EngineRunState) -> R,
    {
        let state = &self.0.lock().unwrap().state;
        f(state)
    }
}

pub struct EngineRunFuture {
    worker: Worker,
    engine_run: EngineRun,
    gui_ctx: egui::Context,
}

impl EngineRunFuture {
    fn start(msg: RunRsonpathMessage, engine_run: EngineRun, gui_ctx: egui::Context, worker: Worker) -> Self {
        {
            let mut inner = engine_run.0.lock().unwrap();
            inner.state = EngineRunState::Requested(EngineRunRequested { msg });
        }

        Self {
            worker,
            gui_ctx,
            engine_run,
        }
    }
}

impl Future for EngineRunFuture {
    type Output = Result<(), String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use web_sys::console;
        let mut engine_run = self.engine_run.0.lock().unwrap();
        let mut state = EngineRunState::None;
        std::mem::swap(&mut engine_run.state, &mut state);
        let state = state;
        let (state, response) = match state {
            EngineRunState::Idle => (EngineRunState::Idle, Poll::Ready(Ok(()))),
            EngineRunState::Requested(request) => {
                console::log_1(&"Requested, starting...".into());
                let finished = Rc::new(RefCell::new(None));
                let finished_clone = finished.clone();
                let waker = cx.waker().clone();
                let gui_ctx = self.gui_ctx.clone();
                let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| match msg.ty() {
                    MessageType::RunRsonpathSuccess => {
                        let msg = msg
                            .deserialize::<RunRsonpathSuccessMessage>()
                            .expect("RunRsonpathSuccessMessage to be correct");
                        finished_clone.borrow_mut().replace(EngineRunFinished::Success(msg));
                        waker.wake_by_ref();
                        gui_ctx.request_repaint();
                    }
                    MessageType::RunRsonpathFailure => {
                        let msg = msg
                            .deserialize::<RunRsonpathFailureMessage>()
                            .expect("RunRsonpathFailureMessage to be correct");
                        finished_clone.borrow_mut().replace(EngineRunFinished::Failure(msg));
                        waker.wake_by_ref();
                        gui_ctx.request_repaint();
                    }
                    _ => (),
                }) as Box<dyn Fn(MessageEvent)>);
                self.worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                self.worker
                    .send(request.msg)
                    .expect("RunRsonpathMessage sent to worker");

                (
                    EngineRunState::InProgress(EngineRunInProgress { finished }),
                    Poll::Pending,
                )
            }
            EngineRunState::InProgress(in_progress) => {
                if let Some(finished) = in_progress.finished.take() {
                    console::log_1(&"We are done!".into());
                    self.gui_ctx.request_repaint();
                    match finished {
                        EngineRunFinished::Success(success) => (
                            EngineRunState::Succeeded(EngineRunResult {
                                runtime: success.runtime().clone(),
                                results: success.into_results(),
                            }),
                            Poll::Ready(Ok(())),
                        ),
                        EngineRunFinished::Failure(failure) => (
                            EngineRunState::Failed(EngineRunFailure {
                                error: failure.into_error(),
                            }),
                            Poll::Ready(Ok(())),
                        ),
                    }
                } else {
                    console::log_1(&"We are not done.".into());
                    (EngineRunState::InProgress(in_progress), Poll::Pending)
                }
            }
            EngineRunState::Succeeded(loaded) => (EngineRunState::Succeeded(loaded), Poll::Ready(Ok(()))),
            EngineRunState::Failed(loaded) => (EngineRunState::Failed(loaded), Poll::Ready(Ok(()))),
            EngineRunState::None => unreachable!("EngineRunState::None cannot happen"),
        };

        engine_run.state = state;
        response
    }
}
