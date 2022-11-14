pub trait Implementation: Sized {
    type Query;
    type File;
    type Error: std::error::Error;

    fn id() -> &'static str;

    fn new() -> Result<Self, Self::Error>;

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error>;

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error>;

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error>;
}

pub struct PreparedQuery<I: Implementation> {
    pub(crate) implementation: I,
    pub(crate) query: I::Query,
    pub(crate) file: I::File,
}

pub(crate) fn prepare<I: Implementation>(
    implementation: I,
    file_path: &str,
    query: &str,
) -> Result<PreparedQuery<I>, I::Error> {
    let query = implementation.compile_query(query)?;
    let file = implementation.load_file(file_path)?;

    Ok(PreparedQuery {
        implementation,
        query,
        file,
    })
}
