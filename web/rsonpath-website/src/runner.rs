use rsonpath::automaton::Automaton;
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::BorrowedBytes;
use rsonpath::result::MatchWriter;
use rsonpath_syntax::{JsonPathQuery, ParserBuilder};
use rsonpath_website::constants;
use rsonpath_website::message::*;
use rsonpath_website::util::{DisplaySize, error_string};
use std::io::Write;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, FileReader, MessageEvent, console};
use web_time::Instant;

#[derive(Clone)]
struct Runner(Rc<RunnerImpl>);

struct RunnerImpl {
    scope: DedicatedWorkerGlobalScope,
    input: Mutex<RunnerInput>,
}

enum RunnerInput {
    None,
    Loading,
    Loaded(String),
}

fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"worker starting".into());

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));

    Runner::init(scope.clone());

    let msg = WorkerStartedMessage::new(0);
    scope.send(msg).expect("posting ready message succeeds");
}

impl Runner {
    fn init(scope: DedicatedWorkerGlobalScope) {
        let this = Self(Rc::new(RunnerImpl {
            scope: scope.clone(),
            input: Mutex::new(RunnerInput::None),
        }));

        let handler = Closure::wrap(Box::new(move |msg: MessageEvent| {
            this.handle(msg);
        }) as Box<dyn Fn(MessageEvent)>);
        scope.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
    }

    fn handle(&self, event: MessageEvent) {
        match event.ty() {
            MessageType::LoadFile => {
                let msg = event
                    .deserialize::<LoadFileMessage>()
                    .expect("LoadFileMessage to be correct");
                if let Err(err) = self.handle_load_file(msg) {
                    self.0
                        .scope
                        .send(LoadFileFailureMessage::new(error_string(err)))
                        .expect("posting response to LoadFile succeeds")
                }
            }
            MessageType::LoadFileProgress => console::warn_1(&"Runner cannot handle LoadFileProgress".into()),
            MessageType::LoadFileSuccess => console::warn_1(&"Runner cannot handle LoadFileSuccess".into()),
            MessageType::LoadFileFailure => console::warn_1(&"Runner cannot handle LoadFileFailure".into()),
            MessageType::DiscardFile => {
                let _ = event
                    .deserialize::<DiscardFileMessage>()
                    .expect("DiscardFileMessage to be correct");
                self.discard_file();
            }
            MessageType::RunRsonpath => {
                let msg = event
                    .deserialize::<RunRsonpathMessage>()
                    .expect("RunRsonpathMessage to be correct");
                if let Err(err) = self.handle_run(msg) {
                    self.0
                        .scope
                        .send(RunRsonpathFailureMessage::new(error_string(err)))
                        .expect("posting response to RunRsonpath succeeds")
                }
            }
            MessageType::RunRsonpathSuccess => console::warn_1(&"Runner cannot handle RunRsonpathSuccess".into()),
            MessageType::RunRsonpathFailure => console::warn_1(&"Runner cannot handle RunRsonpathFailure".into()),
            MessageType::WorkerStarted => console::warn_1(&"Runner cannot handle WorkerStarted".into()),
            MessageType::Unknown => console::warn_1(&"Runner received unknown message".into()),
        }
    }

    fn handle_load_file(&self, msg: LoadFileMessage) -> Result<(), JsValue> {
        console::log_1(&"Runner handling LoadFile event...".into());
        {
            let mut state = self.0.input.lock().unwrap();

            match &*state {
                RunnerInput::Loading => return Err("Runner received LoadFile event while already loading".into()),
                RunnerInput::None | RunnerInput::Loaded(_) => {
                    *state = RunnerInput::Loading;
                }
            };
        }
        if msg.file().size() > constants::FILE_MAX_BYTES as f64 {
            return Err(format!(
                "The file is too large; max size is {} due to browser limitations.",
                DisplaySize(constants::FILE_MAX_BYTES as f64)
            )
            .into());
        }
        let reader = FileReader::new()?;
        match reader.read_as_text(msg.file()) {
            Ok(_) => {
                let runner = self.0.clone();
                let reader_clone = reader.clone();
                let onloadend_cb = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    assert_eq!(reader_clone.ready_state(), FileReader::DONE);
                    if let Some(err) = reader_clone.error() {
                        runner
                            .scope
                            .send(LoadFileFailureMessage::new(err.message()))
                            .expect("posting LoadFileFailure message succeeds")
                    } else {
                        let text_value = reader_clone.result().expect("result should be Ok when Error is None");
                        let text = text_value.as_string().expect("text value should be String");
                        let mut preview = String::with_capacity(constants::FILE_PREVIEW_CHARS + 3);
                        preview.extend(text.chars().take(constants::FILE_PREVIEW_CHARS));
                        preview += "...";
                        *runner.input.lock().unwrap() = RunnerInput::Loaded(text);
                        runner
                            .scope
                            .send(LoadFileSuccessMessage::new(preview))
                            .expect("posting LoadFileSuccess message succeeds");
                    }
                }) as Box<dyn FnMut(_)>);
                let runner = self.0.clone();
                let onprogress = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    let event = event.dyn_ref::<web_sys::ProgressEvent>().unwrap();
                    let progress = (event.loaded() / event.total()) as f32;
                    runner
                        .scope
                        .send(LoadFileProgressMessage::new(progress))
                        .expect("posting LoadFileProgress message succeeds");
                }) as Box<dyn FnMut(_)>);
                reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                reader.set_onprogress(Some(onprogress.as_ref().unchecked_ref()));
                onloadend_cb.forget();
                onprogress.forget();
                Ok(())
            }
            Err(err) => {
                console::error_1(&"LoadFile: immediate error.".into());
                Err(err)
            }
        }
    }

    fn discard_file(&self) {
        console::log_1(&"Runner handling DiscardFile event...".into());
        let mut input = self.0.input.lock().unwrap();
        *input = RunnerInput::None;
    }

    fn handle_run(&self, msg: RunRsonpathMessage) -> Result<(), JsValue> {
        console::log_1(&"Runner handling RunRsonpath event...".into());
        let input_lock = self.0.input.lock().unwrap();
        let input = match msg.input() {
            RunRsonpathInput::LoadedFile => match &*input_lock {
                RunnerInput::None | RunnerInput::Loading => {
                    return Err("LoadedFile run requested but no file is loaded".into());
                }
                RunnerInput::Loaded(json) => BorrowedBytes::new(json.as_bytes()),
            },
            RunRsonpathInput::Inline(json) => BorrowedBytes::new(json.as_bytes()),
        };

        let msg = if !msg.is_benchmark_run() {
            let (results, stats) = run_once(&msg, &input)?;
            RunRsonpathSuccessMessage::new(stats, results)
        } else {
            let (results, _) = run_once(&msg, &input)?;
            for _ in 1..constants::WARMUP_RUNS {
                let _ = run_once(&msg, &input)?;
            }
            let mut acc_stats = RsonpathRuntime::default();
            for _ in 0..msg.repetitions() {
                let (_, stats) = run_once(&msg, &input)?;
                acc_stats += stats;
            }
            RunRsonpathSuccessMessage::new(acc_stats / msg.repetitions(), results)
        };

        self.0.scope.send(msg)?;
        console::log_1(&"Message sent.".into());

        return Ok(());

        fn run_once(msg: &RunRsonpathMessage, input: &BorrowedBytes) -> Result<(String, RsonpathRuntime), JsValue> {
            let parse_start = Instant::now();
            let query = parse_query(msg.query())?;
            let parse_time = parse_start.elapsed();
            console::log_1(&"Parsed!".into());

            let compile_start = Instant::now();
            let automaton = compile_query(&query)?;
            let compile_time = compile_start.elapsed();
            console::log_1(&"Compiled!".into());

            let mut out = Vec::new();

            let engine = RsonpathEngine::from_compiled_query(automaton);

            let start = Instant::now();
            match msg.result_mode() {
                RunRsonpathMode::Count => engine.count(input).map(|res| write!(&mut out, "{res}").unwrap()),
                RunRsonpathMode::Indices => {
                    let mut sink = MatchWriter::from(&mut out);
                    engine.indices(input, &mut sink)
                }
                RunRsonpathMode::Nodes => {
                    let mut sink = MatchWriter::from(&mut out);
                    engine.matches(input, &mut sink)
                }
            }
            .map_err(|err| err.to_string())?;
            let run_time = start.elapsed();
            console::log_2(&"Finished in".into(), &run_time.as_secs_f64().into());

            let results = String::from_utf8(out).expect("<Invalid UTF-8 in stdout>");
            let stats = RsonpathRuntime::new(
                parse_time.as_secs_f64() * 1000.0,
                compile_time.as_secs_f64() * 1000.0,
                run_time.as_secs_f64() * 1000.0,
            );
            Ok((results, stats))
        }

        fn parse_query(query_string: &str) -> Result<JsonPathQuery, String> {
            let mut parser_builder = ParserBuilder::default();
            parser_builder.allow_surrounding_whitespace(true);
            let parser: rsonpath_syntax::Parser = parser_builder.into();
            parser.parse(query_string).map_err(|err| err.to_string())
        }

        fn compile_query(query: &JsonPathQuery) -> Result<Automaton, String> {
            Automaton::new(query).map_err(|err| err.to_string())
        }
    }
}
