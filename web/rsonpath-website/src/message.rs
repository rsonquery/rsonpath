use js_sys::Array;
use std::ops::{Add, AddAssign, Div};
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};

const WORKER_STARTED_CODE: f64 = 10.0;
const LOAD_FILE_CODE: f64 = 20.0;
const LOAD_FILE_PROGRESS_CODE: f64 = 21.0;
const LOAD_FILE_SUCCESS_CODE: f64 = 22.0;
const LOAD_FILE_FAILURE_CODE: f64 = 23.0;
const RUN_RSONPATH_CODE: f64 = 30.0;
const RUN_RSONPATH_SUCCESS_CODE: f64 = 31.0;
const RUN_RSONPATH_FAILURE_CODE: f64 = 32.0;
const DISCARD_FILE_CODE: f64 = 40.0;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageType {
    WorkerStarted,
    LoadFile,
    LoadFileProgress,
    LoadFileSuccess,
    LoadFileFailure,
    DiscardFile,
    RunRsonpath,
    RunRsonpathSuccess,
    RunRsonpathFailure,
    Unknown,
}

fn code_to_type(code: f64) -> MessageType {
    match code {
        WORKER_STARTED_CODE => MessageType::WorkerStarted,
        LOAD_FILE_CODE => MessageType::LoadFile,
        LOAD_FILE_PROGRESS_CODE => MessageType::LoadFileProgress,
        LOAD_FILE_SUCCESS_CODE => MessageType::LoadFileSuccess,
        LOAD_FILE_FAILURE_CODE => MessageType::LoadFileFailure,
        DISCARD_FILE_CODE => MessageType::DiscardFile,
        RUN_RSONPATH_CODE => MessageType::RunRsonpath,
        RUN_RSONPATH_SUCCESS_CODE => MessageType::RunRsonpathSuccess,
        RUN_RSONPATH_FAILURE_CODE => MessageType::RunRsonpathFailure,
        _ => MessageType::Unknown,
    }
}

pub trait Message: Sized {
    fn ty() -> MessageType;

    fn serialize(self) -> JsValue;

    fn deserialize(value: JsValue) -> Result<Self, JsValue>;
}

pub trait MessageChannel {
    fn send(&self, message: impl Message) -> Result<(), JsValue>;
}

pub trait GenericMessage {
    fn ty(&self) -> MessageType;

    fn deserialize<T: Message>(&self) -> Result<T, JsValue>;
}

impl MessageChannel for web_sys::Worker {
    fn send(&self, message: impl Message) -> Result<(), JsValue> {
        let value = message.serialize();
        self.post_message(&value)
    }
}

impl MessageChannel for web_sys::DedicatedWorkerGlobalScope {
    fn send(&self, message: impl Message) -> Result<(), JsValue> {
        let value = message.serialize();
        self.post_message(&value)
    }
}

impl GenericMessage for web_sys::MessageEvent {
    fn ty(&self) -> MessageType {
        let data = Array::from(&self.data());
        if data.length() >= 1
            && let Some(code) = data.get(0).as_f64()
        {
            code_to_type(code)
        } else {
            MessageType::Unknown
        }
    }

    fn deserialize<T: Message>(&self) -> Result<T, JsValue> {
        assert_eq!(T::ty(), self.ty(), "deserializing into invalid message type");
        T::deserialize(self.data())
    }
}

pub struct WorkerStartedMessage {
    status: i32,
}

pub struct LoadFileMessage {
    file: web_sys::File,
}

pub struct LoadFileProgressMessage {
    progress: f32,
}

pub struct LoadFileSuccessMessage {
    preview: String,
}

pub struct LoadFileFailureMessage {
    error: String,
}

pub struct DiscardFileMessage {}

pub struct RunRsonpathMessage {
    query: String,
    input: RunRsonpathInput,
    result_mode: RunRsonpathMode,
    repetitions: usize,
}

pub enum RunRsonpathInput {
    LoadedFile,
    Inline(String),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RunRsonpathMode {
    Count,
    Nodes,
    Indices,
}

pub struct RunRsonpathFailureMessage {
    error: String,
}

pub struct RunRsonpathSuccessMessage {
    runtime: RsonpathRuntime,
    results: String,
}

#[derive(Clone)]
pub struct RsonpathRuntime {
    avg_parse_time_ms: f64,
    avg_compile_time_ms: f64,
    avg_run_time_ms: f64,
}

impl RsonpathRuntime {
    pub fn new(avg_parse_time_ms: f64, avg_compile_time_ms: f64, avg_run_time_ms: f64) -> Self {
        Self {
            avg_parse_time_ms,
            avg_compile_time_ms,
            avg_run_time_ms,
        }
    }

    pub fn avg_parse_time(&self) -> Duration {
        Duration::from_secs_f64(self.avg_parse_time_ms / 1000.0)
    }

    pub fn avg_compile_time(&self) -> Duration {
        Duration::from_secs_f64(self.avg_compile_time_ms / 1000.0)
    }

    pub fn avg_run_time(&self) -> Duration {
        Duration::from_secs_f64(self.avg_run_time_ms / 1000.0)
    }
}

impl Default for RsonpathRuntime {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Add for RsonpathRuntime {
    type Output = RsonpathRuntime;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.avg_parse_time_ms + rhs.avg_parse_time_ms,
            self.avg_compile_time_ms + rhs.avg_compile_time_ms,
            self.avg_run_time_ms + rhs.avg_run_time_ms,
        )
    }
}

impl AddAssign for RsonpathRuntime {
    fn add_assign(&mut self, rhs: Self) {
        self.avg_parse_time_ms += rhs.avg_parse_time_ms;
        self.avg_compile_time_ms += rhs.avg_compile_time_ms;
        self.avg_run_time_ms += rhs.avg_run_time_ms;
    }
}

impl Div<usize> for RsonpathRuntime {
    type Output = RsonpathRuntime;

    fn div(self, rhs: usize) -> Self::Output {
        let rhs = rhs as f64;
        Self::new(
            self.avg_parse_time_ms / rhs,
            self.avg_compile_time_ms / rhs,
            self.avg_run_time_ms / rhs,
        )
    }
}

impl WorkerStartedMessage {
    pub fn new(status: i32) -> Self {
        Self { status }
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}

impl Message for WorkerStartedMessage {
    fn ty() -> MessageType {
        MessageType::WorkerStarted
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&WORKER_STARTED_CODE.into());
        response.push(&self.status.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed WorkerStartedMessage: array length {}", data.length()).into())
        } else {
            let Some(status) = data.get(1).as_f64() else {
                return Err("malformed WorkerStartedMessage: element is not a number"
                    .to_string()
                    .into());
            };
            Ok(Self { status: status as i32 })
        }
    }
}

impl LoadFileMessage {
    pub fn new(file: web_sys::File) -> Self {
        Self { file }
    }

    pub fn file(&self) -> &web_sys::File {
        &self.file
    }
}

impl Message for LoadFileMessage {
    fn ty() -> MessageType {
        MessageType::LoadFile
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&LOAD_FILE_CODE.into());
        response.push(&self.file.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed LoadFileMessage: array length {}", data.length()).into())
        } else {
            match data.get(1).dyn_into::<web_sys::File>() {
                Ok(file) => Ok(Self { file }),
                Err(_err) => Err("malformed LoadFileMessage: element is not a File".to_string().into()),
            }
        }
    }
}

impl LoadFileProgressMessage {
    pub fn new(progress: f32) -> Self {
        Self { progress }
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }
}

impl Message for LoadFileProgressMessage {
    fn ty() -> MessageType {
        MessageType::LoadFileProgress
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&LOAD_FILE_PROGRESS_CODE.into());
        response.push(&self.progress.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed LoadFileProgressMessage: array length {}", data.length()).into())
        } else {
            let Some(progress) = data.get(1).as_f64() else {
                return Err("malformed LoadFileProgressMessage: element is not a number"
                    .to_string()
                    .into());
            };
            Ok(Self {
                progress: progress as f32,
            })
        }
    }
}

impl LoadFileSuccessMessage {
    pub fn new(preview: String) -> Self {
        Self { preview }
    }

    pub fn preview(&self) -> &str {
        &self.preview
    }

    pub fn into_preview(self) -> String {
        self.preview
    }
}

impl Message for LoadFileSuccessMessage {
    fn ty() -> MessageType {
        MessageType::LoadFileSuccess
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&LOAD_FILE_SUCCESS_CODE.into());
        response.push(&self.preview.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed LoadFileSuccessMessage: array length {}", data.length()).into())
        } else {
            let Some(preview) = data.get(1).as_string() else {
                return Err("malformed LoadFileSuccessMessage: element is not a string"
                    .to_string()
                    .into());
            };
            Ok(Self { preview })
        }
    }
}

impl LoadFileFailureMessage {
    pub fn new(error: String) -> Self {
        Self { error }
    }

    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn into_error(self) -> String {
        self.error
    }
}

impl Message for LoadFileFailureMessage {
    fn ty() -> MessageType {
        MessageType::LoadFileFailure
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&LOAD_FILE_FAILURE_CODE.into());
        response.push(&self.error.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed LoadFileFailureMessage: array length {}", data.length()).into())
        } else {
            let Some(error) = data.get(1).as_string() else {
                return Err("malformed LoadFileFailureMessage: element is not a string"
                    .to_string()
                    .into());
            };
            Ok(Self { error })
        }
    }
}

impl Default for DiscardFileMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscardFileMessage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Message for DiscardFileMessage {
    fn ty() -> MessageType {
        MessageType::DiscardFile
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&DISCARD_FILE_CODE.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 1 {
            Err(format!("malformed DiscardFileMessage: array length {}", data.length()).into())
        } else {
            Ok(Self {})
        }
    }
}

pub struct RunRsonpathMessageBuilder {
    query: String,
    input: RunRsonpathInput,
    result_mode: RunRsonpathMode,
    repetitions: usize,
}

impl RunRsonpathMessageBuilder {
    pub fn new_inline(query: String, input: String) -> Self {
        Self {
            query,
            input: RunRsonpathInput::Inline(input),
            result_mode: RunRsonpathMode::Nodes,
            repetitions: 1,
        }
    }

    pub fn new_file(query: String) -> Self {
        Self {
            query,
            input: RunRsonpathInput::LoadedFile,
            result_mode: RunRsonpathMode::Nodes,
            repetitions: 1,
        }
    }

    pub fn benchmark(&mut self, repetitions: usize) -> &mut Self {
        self.repetitions = repetitions;
        self
    }

    pub fn result_mode(&mut self, mode: RunRsonpathMode) -> &mut Self {
        self.result_mode = mode;
        self
    }

    pub fn into_message(self) -> RunRsonpathMessage {
        RunRsonpathMessage {
            query: self.query,
            input: self.input,
            result_mode: self.result_mode,
            repetitions: self.repetitions,
        }
    }
}

impl From<RunRsonpathMessageBuilder> for RunRsonpathMessage {
    fn from(value: RunRsonpathMessageBuilder) -> Self {
        value.into_message()
    }
}

impl RunRsonpathMessage {
    const INLINE_INPUT_CODE: f64 = 1.0;
    const FILE_INPUT_CODE: f64 = 2.0;
    const RESULT_MODE_COUNT_CODE: f64 = 1.0;
    const RESULT_MODE_NODES_CODE: f64 = 2.0;
    const RESULT_MODE_INDICES_CODE: f64 = 3.0;

    pub fn new_inline(query: String, input: String, result_mode: RunRsonpathMode) -> Self {
        Self {
            query,
            input: RunRsonpathInput::Inline(input),
            result_mode,
            repetitions: 1,
        }
    }

    pub fn new_file(query: String, result_mode: RunRsonpathMode) -> Self {
        Self {
            query,
            input: RunRsonpathInput::LoadedFile,
            result_mode,
            repetitions: 1,
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn input(&self) -> &RunRsonpathInput {
        &self.input
    }

    pub fn result_mode(&self) -> RunRsonpathMode {
        self.result_mode
    }

    pub fn repetitions(&self) -> usize {
        self.repetitions
    }

    pub fn is_benchmark_run(&self) -> bool {
        self.repetitions > 1
    }
}

impl Message for RunRsonpathMessage {
    fn ty() -> MessageType {
        MessageType::RunRsonpath
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&RUN_RSONPATH_CODE.into());
        response.push(&self.query.into());
        match self.input {
            RunRsonpathInput::LoadedFile => {
                response.push(&Self::FILE_INPUT_CODE.into());
                response.push(&JsValue::UNDEFINED);
            }
            RunRsonpathInput::Inline(input) => {
                response.push(&Self::INLINE_INPUT_CODE.into());
                response.push(&input.into());
            }
        };
        match self.result_mode {
            RunRsonpathMode::Count => response.push(&Self::RESULT_MODE_COUNT_CODE.into()),
            RunRsonpathMode::Nodes => response.push(&Self::RESULT_MODE_NODES_CODE.into()),
            RunRsonpathMode::Indices => response.push(&Self::RESULT_MODE_INDICES_CODE.into()),
        };
        response.push(&self.repetitions.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 6 {
            Err(format!("malformed RunRsonpathMessage: array length {}", data.length()).into())
        } else {
            let Some(query) = data.get(1).as_string() else {
                return Err("malformed RunRsonpathMessage: element 1 is not a string"
                    .to_string()
                    .into());
            };
            let Some(input_code) = data.get(2).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 2 is not a number"
                    .to_string()
                    .into());
            };
            let input = match input_code {
                Self::INLINE_INPUT_CODE => {
                    let Some(inline_input) = data.get(3).as_string() else {
                        return Err("malformed RunRsonpathMessage: element 3 is not a string"
                            .to_string()
                            .into());
                    };
                    RunRsonpathInput::Inline(inline_input)
                }
                Self::FILE_INPUT_CODE => RunRsonpathInput::LoadedFile,
                _ => return Err("malformed RunRsonpathMessage: unknown input code".into()),
            };
            let Some(result_code) = data.get(4).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 4 is not a number"
                    .to_string()
                    .into());
            };
            let result_mode = match result_code {
                Self::RESULT_MODE_COUNT_CODE => RunRsonpathMode::Count,
                Self::RESULT_MODE_NODES_CODE => RunRsonpathMode::Nodes,
                Self::RESULT_MODE_INDICES_CODE => RunRsonpathMode::Indices,
                _ => {
                    return Err("malformed RunRsonpathMessage: unknown result code".to_string().into());
                }
            };
            let Some(repetitions) = data.get(5).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 4 is not a number"
                    .to_string()
                    .into());
            };
            Ok(Self {
                query,
                input,
                result_mode,
                repetitions: repetitions as usize,
            })
        }
    }
}

impl RunRsonpathSuccessMessage {
    pub fn new(runtime: RsonpathRuntime, results: String) -> Self {
        Self { runtime, results }
    }

    pub fn runtime(&self) -> &RsonpathRuntime {
        &self.runtime
    }

    pub fn results(&self) -> &str {
        &self.results
    }

    pub fn into_results(self) -> String {
        self.results
    }
}

impl Message for RunRsonpathSuccessMessage {
    fn ty() -> MessageType {
        MessageType::RunRsonpathSuccess
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&RUN_RSONPATH_SUCCESS_CODE.into());
        response.push(&self.results.into());
        response.push(&self.runtime.avg_parse_time_ms.into());
        response.push(&self.runtime.avg_compile_time_ms.into());
        response.push(&self.runtime.avg_run_time_ms.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 5 {
            Err(format!("malformed RunRsonpathSuccessMessage: array length {}", data.length()).into())
        } else {
            let Some(results) = data.get(1).as_string() else {
                return Err("malformed RunRsonpathSuccessMessage: element 1 is not a string"
                    .to_string()
                    .into());
            };
            let Some(avg_parse_time_ms) = data.get(2).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 2 is not a number"
                    .to_string()
                    .into());
            };
            let Some(avg_compile_time_ms) = data.get(3).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 3 is not a number"
                    .to_string()
                    .into());
            };
            let Some(avg_run_time_ms) = data.get(4).as_f64() else {
                return Err("malformed RunRsonpathMessage: element 4 is not a number"
                    .to_string()
                    .into());
            };
            Ok(Self {
                results,
                runtime: RsonpathRuntime {
                    avg_parse_time_ms,
                    avg_compile_time_ms,
                    avg_run_time_ms,
                },
            })
        }
    }
}

impl RunRsonpathFailureMessage {
    pub fn new(error: String) -> Self {
        Self { error }
    }

    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn into_error(self) -> String {
        self.error
    }
}

impl Message for RunRsonpathFailureMessage {
    fn ty() -> MessageType {
        MessageType::RunRsonpathFailure
    }

    fn serialize(self) -> JsValue {
        let response = Array::new();
        response.push(&RUN_RSONPATH_FAILURE_CODE.into());
        response.push(&self.error.into());
        response.into()
    }

    fn deserialize(value: JsValue) -> Result<Self, JsValue> {
        let data = Array::from(&value);
        if data.length() != 2 {
            Err(format!("malformed LoadFileFailureMessage: array length {}", data.length()).into())
        } else {
            let Some(error) = data.get(1).as_string() else {
                return Err("malformed LoadFileFailureMessage: element is not a string"
                    .to_string()
                    .into());
            };
            Ok(Self { error })
        }
    }
}
