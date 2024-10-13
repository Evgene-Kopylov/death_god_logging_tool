use flexi_logger::FlexiLoggerError;

#[derive(Debug)]
pub enum Error {
    BaseError,
    FlexiLoggerError,
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(_value: Box<dyn std::error::Error>) -> Self {
        Error::BaseError
    }
}

impl From<FlexiLoggerError> for Error {
    fn from(_value: FlexiLoggerError) -> Self {
        Error::FlexiLoggerError
    }
}
