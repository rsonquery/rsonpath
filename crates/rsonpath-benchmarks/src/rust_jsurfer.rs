use std::num::TryFromIntError;

use crate::framework::implementation::Implementation;
use jni::objects::{JClass, JObject, JValue};
use jni::signature::{JavaType, Primitive, TypeSignature};
use jni::{AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use lazy_static::lazy_static;
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
    TypeSignature {
        args: vec![string_type()],
        ret: json_file_type(),
    }
    .to_string()
}

fn compile_query_sig() -> String {
    TypeSignature {
        args: vec![string_type()],
        ret: compiled_query_type(),
    }
    .to_string()
}

fn overhead_sig() -> String {
    TypeSignature {
        args: vec![],
        ret: compiled_query_type(),
    }
    .to_string()
}

fn run_sig() -> String {
    TypeSignature {
        args: vec![json_file_type()],
        ret: JavaType::Primitive(Primitive::Long),
    }
    .to_string()
}

lazy_static! {
    static ref JVM: Jvm = Jvm::new().unwrap();
}

pub struct Jvm(JavaVM);

pub struct JSurferContext<'j> {
    jvm: AttachGuard<'j>,
    shim: JClass<'j>,
}

pub struct CompiledQuery<'j> {
    query_object: JObject<'j>,
}

pub struct LoadedFile<'j> {
    file_object: JValue<'j>,
}

pub struct Overhead<'j> {
    env: &'j JNIEnv<'j>,
    shim: JObject<'j>,
}

impl Jvm {
    fn new() -> Result<Self, JSurferError> {
        let jar_path = std::env::var("RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH")
            .map_err(JSurferError::NoJarPathEnvVar)?;

        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option(&format!("-Djava.class.path={jar_path}"))
            .build()?;

        let jvm = JavaVM::new(jvm_args)?;

        Ok(Jvm(jvm))
    }

    pub fn attach() -> Result<JSurferContext<'static>, JSurferError> {
        let guard = JVM.0.attach_current_thread()?;
        let shim = guard.find_class(SHIM_CLASS)?;

        Ok(JSurferContext { jvm: guard, shim })
    }
}

impl<'j> JSurferContext<'j> {
    pub fn env(&self) -> &JNIEnv<'j> {
        &self.jvm
    }

    pub fn create_overhead(&'j self) -> Result<Overhead<'j>, JSurferError> {
        let overhead_result =
            self.env()
                .call_static_method(self.shim, OVERHEAD_METHOD, overhead_sig(), &[])?;

        let overhead_object = match overhead_result {
            JValue::Object(obj) => obj,
            _ => {
                return Err(type_error(
                    OVERHEAD_METHOD,
                    "Object",
                    overhead_result.type_name(),
                ))
            }
        };

        Ok(Overhead {
            env: self.env(),
            shim: overhead_object,
        })
    }
}

impl<'j> Overhead<'j> {
    pub fn run(&self, loaded_file: &LoadedFile) -> Result<i64, JSurferError> {
        let result =
            self.env
                .call_method(self.shim, RUN_METHOD, run_sig(), &[loaded_file.file_object])?;

        match result {
            JValue::Long(res) => Ok(res),
            _ => panic!("run returned something else than long"),
        }
    }
}

pub struct JSurfer {
    context: JSurferContext<'static>,
}

impl JSurfer {
    fn env(&self) -> &JNIEnv<'static> {
        self.context.env()
    }

    fn shim(&self) -> JClass<'static> {
        self.context.shim
    }
}

impl Implementation for JSurfer {
    type Query = CompiledQuery<'static>;

    type File = LoadedFile<'static>;

    type Error = JSurferError;

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

        let loaded_file = self.env().call_static_method(
            self.shim(),
            LOAD_METHOD,
            load_file_sig(),
            &[file_string.into()],
        )?;

        Ok(LoadedFile {
            file_object: loaded_file,
        })
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query_string = self.env().new_string(query)?;
        let compile_query_result = self.env().call_static_method(
            self.shim(),
            COMPILE_METHOD,
            compile_query_sig(),
            &[query_string.into()],
        )?;

        let compiled_query_object = match compile_query_result {
            JValue::Object(query_obj) => query_obj,
            _ => {
                return Err(type_error(
                    COMPILE_METHOD,
                    "Object",
                    compile_query_result.type_name(),
                ))
            }
        };

        Ok(CompiledQuery {
            query_object: compiled_query_object,
        })
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        let result = self.env().call_method(
            query.query_object,
            RUN_METHOD,
            run_sig(),
            &[file.file_object],
        )?;

        match result {
            JValue::Long(res) => res
                .try_into()
                .map_err(|err| JSurferError::ResultOutOfRange {
                    value: res,
                    source: err,
                }),
            _ => {
                return Err(type_error(
                    RUN_METHOD,
                    "Long (non-negative)",
                    result.type_name(),
                ))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum JSurferError {
    #[error("could not find JSurfer shim jar path (this should be set by the build script)")]
    NoJarPathEnvVar(std::env::VarError),
    #[error("error while setting up the JVM")]
    JvmError(#[from] jni::JvmError),
    #[error("runtime error in JSurfer code")]
    JavaRuntimeError(#[from] jni::errors::Error),
    #[error("JVM method {method} returned {actual} when {expected} was expected")]
    JavaTypeError {
        method: String,
        expected: String,
        actual: String,
    },
    #[error("received result outside of u64 range: {value}")]
    ResultOutOfRange {
        value: i64,
        #[source]
        source: TryFromIntError,
    },
}

fn type_error(method: &str, expected: &str, actual: &str) -> JSurferError {
    JSurferError::JavaTypeError {
        method: method.to_owned(),
        expected: expected.to_owned(),
        actual: actual.to_owned(),
    }
}
