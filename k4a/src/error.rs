#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Error {
    Failed,
}

impl From<libk4a_sys::k4a_result_t> for Error {
    fn from(result: libk4a_sys::k4a_result_t) -> Self {
        match result {
            libk4a_sys::k4a_result_t::K4A_RESULT_FAILED => Error::Failed,
            _ => unreachable!(),
        }
    }
}

pub fn k4a_result(result: libk4a_sys::k4a_result_t) -> Result<(), Error> {
    match result {
        libk4a_sys::k4a_result_t::K4A_RESULT_SUCCEEDED => Ok(()),
        _ => Err(result.into()),
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum WaitError {
    Failed,
    Timeout,
}

impl From<libk4a_sys::k4a_wait_result_t> for WaitError {
    fn from(result: libk4a_sys::k4a_wait_result_t) -> Self {
        match result {
            libk4a_sys::k4a_wait_result_t::K4A_WAIT_RESULT_FAILED => WaitError::Failed,
            libk4a_sys::k4a_wait_result_t::K4A_WAIT_RESULT_TIMEOUT => WaitError::Timeout,
            _ => unreachable!(),
        }
    }
}

pub fn k4a_wait_result(wait_result: libk4a_sys::k4a_wait_result_t) -> Result<(), WaitError> {
    match wait_result {
        libk4a_sys::k4a_wait_result_t::K4A_WAIT_RESULT_SUCCEEDED => Ok(()),
        _ => Err(wait_result.into()),
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum StreamError {
    Failed,
    Eof,
}

impl From<libk4a_sys::k4a_stream_result_t> for StreamError {
    fn from(result: libk4a_sys::k4a_stream_result_t) -> Self {
        match result {
            libk4a_sys::k4a_stream_result_t::K4A_STREAM_RESULT_FAILED => StreamError::Failed,
            libk4a_sys::k4a_stream_result_t::K4A_STREAM_RESULT_EOF => StreamError::Eof,
            _ => unreachable!(),
        }
    }
}

pub fn k4a_stream_result(wait_result: libk4a_sys::k4a_stream_result_t) -> Result<(), StreamError> {
    match wait_result {
        libk4a_sys::k4a_stream_result_t::K4A_STREAM_RESULT_SUCCEEDED => Ok(()),
        _ => Err(wait_result.into()),
    }
}
