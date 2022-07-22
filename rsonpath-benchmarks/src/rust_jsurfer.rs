use eyre::{eyre, Result};
use jni::objects::{JClass, JObject, JValue};
use jni::signature::{JavaType, Primitive, TypeSignature};
use jni::{AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use lazy_static::lazy_static;

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
    env: &'j JNIEnv<'j>,
    query: JObject<'j>,
}

pub struct LoadedFile<'j> {
    file: JValue<'j>,
}

pub struct Overhead<'j> {
    env: &'j JNIEnv<'j>,
    shim: JObject<'j>,
}

impl Jvm {
    fn new() -> Result<Self> {
        let jar_path = std::env::var("RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH")?;

        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option(&format!("-Djava.class.path={jar_path}"))
            .build()?;

        let jvm = JavaVM::new(jvm_args)?;

        Ok(Jvm(jvm))
    }

    pub fn attach() -> Result<JSurferContext<'static>> {
        let guard = JVM.0.attach_current_thread()?;
        let shim = guard.find_class(SHIM_CLASS)?;

        Ok(JSurferContext { jvm: guard, shim })
    }
}

impl<'j> JSurferContext<'j> {
    pub fn env(&self) -> &JNIEnv {
        &self.jvm
    }

    pub fn compile_query(&'j self, query: &'_ str) -> Result<CompiledQuery<'j>> {
        let query_string = self.env().new_string(query)?;
        let compile_query_result = self.env().call_static_method(
            self.shim,
            COMPILE_METHOD,
            compile_query_sig(),
            &[query_string.into()],
        )?;

        let compiled_query_object = match compile_query_result {
            JValue::Object(query_obj) => query_obj,
            _ => {
                return Err(eyre!(
                    "compileQuery returned something other than an object"
                ))
            }
        };

        Ok(CompiledQuery {
            env: self.env(),
            query: compiled_query_object,
        })
    }

    pub fn load_file(&'j self, file_path: &'_ str) -> Result<LoadedFile<'j>> {
        let file_string = self.env().new_string(file_path)?;

        let loaded_file = self.env().call_static_method(
            self.shim,
            LOAD_METHOD,
            load_file_sig(),
            &[file_string.into()],
        )?;

        Ok(LoadedFile { file: loaded_file })
    }

    pub fn create_overhead(&'j self) -> Result<Overhead<'j>> {
        let overhead_result =
            self.env()
                .call_static_method(self.shim, OVERHEAD_METHOD, overhead_sig(), &[])?;

        let overhead_object = match overhead_result {
            JValue::Object(obj) => obj,
            _ => {
                return Err(eyre!(
                    "overheadShim returned something other than an object"
                ))
            }
        };

        Ok(Overhead {
            env: self.env(),
            shim: overhead_object,
        })
    }
}

impl<'j> CompiledQuery<'j> {
    pub fn run(&self, loaded_file: &LoadedFile) -> Result<i64> {
        let result =
            self.env
                .call_method(self.query, RUN_METHOD, run_sig(), &[loaded_file.file])?;

        match result {
            JValue::Long(res) => Ok(res),
            _ => Err(eyre!("run returned something else than long")),
        }
    }
}

impl<'j> Overhead<'j> {
    pub fn run(&self, loaded_file: &LoadedFile) -> Result<i64> {
        let result = self
            .env
            .call_method(self.shim, RUN_METHOD, run_sig(), &[loaded_file.file])?;

        match result {
            JValue::Long(res) => Ok(res),
            _ => Err(eyre!("run returned something else than long")),
        }
    }
}
