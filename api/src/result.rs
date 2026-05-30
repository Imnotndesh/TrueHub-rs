#[derive(Debug, Clone)]
pub enum ApiResult<T> {
    Loading,
    Success(T),
    Error { message: String },
}