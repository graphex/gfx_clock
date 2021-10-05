use core::fmt::Debug;

pub type DisplayMessageResult<T> = Result<T, DisplayMessageError>;

#[derive(Debug, Copy, Clone)]
pub enum DisplayMessageError {
    TubeIndexOutOfRange,
    UnexpectedCharForTubeType,
}