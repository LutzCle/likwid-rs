use thiserror::Error;

pub type Result<T> = std::result::Result<T, LikwidError>;

#[derive(Error, Debug)]
pub enum LikwidError {
    #[error("dynamic loading of likwid library failed")]
    DynamicLibraryError(#[from] libloading::Error),

    #[error("register region failed")]
    LikwidError(#[from] std::io::Error),

    #[error("region name contains a nul byte")]
    InvalidRegion(#[from] std::ffi::NulError),
}
