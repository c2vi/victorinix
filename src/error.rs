
pub type VicResult<T> = Result<T, VicError>;

pub struct VicError {
    msg: String,
} 
