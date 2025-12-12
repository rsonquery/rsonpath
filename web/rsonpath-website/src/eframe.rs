use rsonpath_website::constants;
use rsonpath_website::message::*;
use std::pin::Pin;
use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
};
use std::task::{Context, Poll};
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker, console, js_sys::Array};

fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    color_eyre::install().expect("color_eyre install");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document");
    let canvas = document
        .get_element_by_id(constants::CANVAS_ELEMENT_ID)
        .expect("canvas element not found, update CANVAS_ELEMENT_ID")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    wasm_bindgen_futures::spawn_local(async move {
        let runner = eframe::WebRunner::new();
        // The GUI must wait for the Runner to spawn, or else actions such as Run or Open File will not be handled.
        console::log_1(&"⏳ Waiting for rsonpath worker to spawn...".into());
        let worker = create_worker().await.expect("Failed to create worker");
        console::log_1(&"Worker ready.".into());
        // Handle control to the Website struct.
        worker.set_onmessage(None);
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(rsonpath_website::start(cc, worker)))),
            )
            .await
            .expect("Failed to start eframe WebRunner");
    });

    Ok(())
}

fn create_worker() -> impl Future<Output = Result<web_sys::Worker, JsValue>> {
    // This is basically magic, modelled after trunk webworker example:
    // https://github.com/trunk-rs/trunk/blob/f1ee3d4032cb939b8513d1d8fabfcb84ce46d811/examples/webworker/src/bin/app.rs#L5-L27
    let origin = web_sys::window()
        .expect("window to be available")
        .location()
        .href()
        .expect("location href to be available");

    let script = Array::new();
    script.push(
        &format!(
            r#"importScripts("{origin}/{}.js");wasm_bindgen("{origin}/{}_bg.wasm");"#,
            constants::WORKER_BIN_NAME,
            constants::WORKER_BIN_NAME
        )
        .into(),
    );

    let blob_property_bag = BlobPropertyBag::new();
    blob_property_bag.set_type("text/javascript");
    let blob = Blob::new_with_str_sequence_and_options(&script, &blob_property_bag).expect("blob to be created");

    let url = Url::create_object_url_with_blob(&blob).expect("url to be created");

    let worker = Worker::new(&url).expect("worker to be created");
    let worker_clone = worker.clone();
    // Here we handle control over to our custom Future that coordinates with the spawned worker.
    CreateWorkerFuture::new(worker_clone)
}

struct CreateWorkerFuture {
    worker: Worker,
    state: CreateWorkerFutureState,
}

/// Internal state of the [`CreateWorkerFuture`].
enum CreateWorkerFutureState {
    /// Sentinel value, must not be used between polls.
    None,
    /// Future was created and not polled yet, setup of event handlers required.
    Init,
    /// Message channels were initialized, we are waiting for the worker to report back.
    /// The inner value is the status code Arc with a value of -1 while waiting, set to the actual
    /// status after the worker reports back.
    Launched(Arc<AtomicI32>),
}

impl CreateWorkerFuture {
    pub fn new(worker: Worker) -> Self {
        Self {
            worker,
            state: CreateWorkerFutureState::Init,
        }
    }
}

impl Future for CreateWorkerFuture {
    type Output = Result<Worker, JsValue>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = CreateWorkerFutureState::None;
        std::mem::swap(&mut self.state, &mut state);
        let state = state;
        let (state, response) = match state {
            CreateWorkerFutureState::Init => {
                // Set up the callback and the status code Arc to communicate the message back.
                let status_code = Arc::new(AtomicI32::new(-1));
                let status_code_clone = status_code.clone();
                let waker_clone = cx.waker().clone();
                let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
                    assert_eq!(msg.ty(), MessageType::WorkerStarted, "worker reported invalid message");
                    let msg = msg
                        .deserialize::<WorkerStartedMessage>()
                        .expect("correct WorkerStartedMessage");
                    status_code_clone.store(msg.status(), Ordering::Release);
                    waker_clone.wake_by_ref();
                }) as Box<dyn Fn(MessageEvent)>);
                self.worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();
                (CreateWorkerFutureState::Launched(status_code), Poll::Pending)
            }
            CreateWorkerFutureState::Launched(status_code_arc) => {
                let status_code = status_code_arc.load(Ordering::Acquire);
                if status_code == -1 {
                    // It's not the message handler that woke us up, we're not ready yet.
                    (CreateWorkerFutureState::Launched(status_code_arc), Poll::Pending)
                } else if status_code == 0 {
                    // Successful start.
                    let worker = self.worker.clone();
                    (
                        CreateWorkerFutureState::Launched(status_code_arc),
                        Poll::Ready(Ok(worker)),
                    )
                } else {
                    // Some error occurred.
                    let error = format!("worker failed to create: status code {status_code}").into();
                    (
                        CreateWorkerFutureState::Launched(status_code_arc),
                        Poll::Ready(Err(error)),
                    )
                }
            }
            CreateWorkerFutureState::None => unreachable!("CreateWorkerFutureState::None cannot happen"),
        };

        self.state = state;
        response
    }
}
