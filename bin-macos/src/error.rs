#[derive(Debug)]
pub enum Error {
    Poly(libpoly::Error),
    Seed,
    StreamConfig(cpal::DefaultStreamConfigError),
    BuildStream(cpal::BuildStreamError),
    PlayStream(cpal::PlayStreamError),
    NoDefaultDevice,
}

impl From<libpoly::Error> for Error {
    fn from(error: libpoly::Error) -> Self {
        Error::Poly(error)
    }
}

impl From<cpal::DefaultStreamConfigError> for Error {
    fn from(error: cpal::DefaultStreamConfigError) -> Self {
        Error::StreamConfig(error)
    }
}

impl From<cpal::BuildStreamError> for Error {
    fn from(error: cpal::BuildStreamError) -> Self {
        Error::BuildStream(error)
    }
}

impl From<cpal::PlayStreamError> for Error {
    fn from(error: cpal::PlayStreamError) -> Self {
        Error::PlayStream(error)
    }
}
