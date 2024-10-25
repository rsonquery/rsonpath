use crate::framework::implementation::Implementation;
use jni::objects::{JClass, JObject};
use jni::signature::{JavaType, Primitive, ReturnType, TypeSignature};
use jni::{AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use lazy_static::lazy_static;
use std::num::TryFromIntError;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};
use thiserror::Error;

macro_rules! package {
    () => {
        "com/v0ldek/rsonpath/jsurferShim"
    };
}

const SHIM_CLASS: &str = concat!(package!(), "/Shim");
const QUERY_CLASS: &str = concat!(package!(), "/CompiledQuery");
const FILE_CLASS: &str = concat!(package!(), "/JsonFile");
const COMPILE_METHOD: &str = "compileQuery";
const LOAD_METHOD: &str = "loadFile";
const RUN_METHOD: &str = "run";
const OVERHEAD_METHOD: &str = "overheadShim";

fn string_type() -> JavaType {
    JavaType::Object("java/lang/String".to_owned())
}
fn json_file_type() -> JavaType {
    JavaType::Object(FILE_CLASS.to_owned())
}
fn compiled_query_type() -> JavaType {
    JavaType::Object(QUERY_CLASS.to_owned())
}

fn load_file_sig() -> String {
    format!("({}){}", string_type(), json_file_type())
}

fn compile_query_sig() -> String {
    format!("({}){}", string_type(), compiled_query_type())
}

fn overhead_sig() -> String {
    format!("(){}", compiled_query_type())
}

fn run_sig() -> String {
    let sig = TypeSignature {
        args: vec![json_file_type()],
        ret: ReturnType::Primitive(Primitive::Long),
    };

    sig.to_string()
}

lazy_static! {
    static ref JVM: Jvm = Jvm::new().unwrap();
}

pub struct Jvm(JavaVM);

pub struct JSurferContext<'j> {
    jvm: Mutex<AttachGuard<'j>>,
    shim: JClass<'j>,
}

pub struct CompiledQuery<'j> {
    query_object: JObject<'j>,
}

pub struct LoadedFile<'j> {
    file_object: JObject<'j>,
}

pub struct Overhead<'a, 'j> {
    ctx: &'a JSurferContext<'j>,
    shim: JObject<'j>,
}

impl Jvm {
    fn new() -> Result<Self, JSurferError> {
        let jar_path = std::env::var("RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH").map_err(JSurferError::NoJarPathEnvVar)?;

        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option(format!("-Djava.class.path={jar_path}"))
            .build()?;

        let jvm = JavaVM::new(jvm_args)?;

        Ok(Jvm(jvm))
    }

    pub fn attach() -> Result<JSurferContext<'static>, JSurferError> {
        let mut guard = JVM.0.attach_current_thread()?;
        let shim = guard.find_class(SHIM_CLASS)?;
        let jvm = Mutex::new(guard);

        Ok(JSurferContext { jvm, shim })
    }
}

struct EnvWrap<'a, 'j>(MutexGuard<'a, AttachGuard<'j>>);

impl<'a, 'j> Deref for EnvWrap<'a, 'j> {
    type Target = JNIEnv<'j>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'j> DerefMut for EnvWrap<'a, 'j> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'j> JSurferContext<'j> {
    pub fn env(&'_ self) -> impl DerefMut<Target = JNIEnv<'j>> + '_ {
        EnvWrap(self.jvm.lock().unwrap())
    }

    pub fn create_overhead(&'_ self) -> Result<Overhead<'_, 'j>, JSurferError> {
        let overhead_result = self
            .env()
            .call_static_method(&self.shim, OVERHEAD_METHOD, overhead_sig(), &[])?;

        let actual_type = overhead_result.type_name();
        let overhead_object = overhead_result
            .l()
            .map_err(|e| type_error(e, OVERHEAD_METHOD, "Object", actual_type))?;

        Ok(Overhead {
            ctx: self,
            shim: overhead_object,
        })
    }
}

impl<'a, 'j> Overhead<'a, 'j> {
    pub fn run(&self, loaded_file: &LoadedFile) -> Result<i64, JSurferError> {
        let result =
            self.ctx
                .env()
                .call_method(&self.shim, RUN_METHOD, run_sig(), &[(&loaded_file.file_object).into()])?;

        let actual_type = result.type_name();
        result.j().map_err(|e| type_error(e, RUN_METHOD, "Long", actual_type))
    }
}

pub struct JSurfer {
    context: JSurferContext<'static>,
}

impl JSurfer {
    fn env(&self) -> impl DerefMut<Target = JNIEnv<'static>> + '_ {
        self.context.env()
    }

    fn shim(&self) -> &JClass<'static> {
        &self.context.shim
    }
}

impl Implementation for JSurfer {
    type Query = CompiledQuery<'static>;

    type File = LoadedFile<'static>;

    type Error = JSurferError;

    type Result<'a> = u64; // FIXME

    fn id() -> &'static str {
        "jsurfer"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JSurfer {
            context: Jvm::attach()?,
        })
    }

    fn load_file(&self, path: &str) -> Result<Self::File, Self::Error> {
        let file_string = self.env().new_string(path)?;

        let loaded_file =
            self.env()
                .call_static_method(self.shim(), LOAD_METHOD, load_file_sig(), &[(&file_string).into()])?;

        let actual_type = loaded_file.type_name();
        loaded_file
            .l()
            .map_err(|e| type_error(e, LOAD_METHOD, "Object", actual_type))
            .map(|f| LoadedFile { file_object: f })
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query_string = self.env().new_string(query)?;
        let compile_query_result = self.env().call_static_method(
            self.shim(),
            COMPILE_METHOD,
            compile_query_sig(),
            &[(&query_string).into()],
        )?;

        let actual_type = compile_query_result.type_name();
        let compiled_query_object = compile_query_result
            .l()
            .map_err(|e| type_error(e, OVERHEAD_METHOD, "Object", actual_type))?;

        Ok(CompiledQuery {
            query_object: compiled_query_object,
        })
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        let result = self.env().call_method(
            &query.query_object,
            RUN_METHOD,
            run_sig(),
            &[(&file.file_object).into()],
        )?;

        let actual_type = result.type_name();
        result
            .j()
            .map_err(|e| type_error(e, RUN_METHOD, "Long (non-negative)", actual_type))
            .and_then(|l| {
                l.try_into()
                    .map_err(|err| JSurferError::ResultOutOfRange { value: l, source: err })
            })
    }
}

#[derive(Error, Debug)]
pub enum JSurferError {
    #[error("could not find JSurfer shim jar path (this should be set by the build script): {0}")]
    NoJarPathEnvVar(std::env::VarError),
    #[error("error while setting up the JVM: {0}")]
    JvmError(#[from] jni::JvmError),
    #[error("error while starting the JVM: {0}")]
    StartJvmError(#[from] jni::errors::StartJvmError),
    #[error("runtime error in JSurfer code: {0}")]
    JavaRuntimeError(#[from] jni::errors::Error),
    #[error("JVM method {method} returned {actual} when {expected} was expected")]
    JavaTypeError {
        method: String,
        expected: String,
        actual: String,
        #[source]
        source: jni::errors::Error,
    },
    #[error("received result outside of u64 range: {value}")]
    ResultOutOfRange {
        value: i64,
        #[source]
        source: TryFromIntError,
    },
}

fn type_error(source: jni::errors::Error, method: &str, expected: &str, actual: &str) -> JSurferError {
    JSurferError::JavaTypeError {
        method: method.to_owned(),
        expected: expected.to_owned(),
        actual: actual.to_owned(),
        source,
    }
}
