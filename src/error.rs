pub type Result<T> = std::result::Result<T, PtError>;

#[derive(Debug)]
pub enum PtError {
    ArgNotProvide,
    InternalError,
}
