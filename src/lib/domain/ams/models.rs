use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmsError {
    #[error("Provider not found")]
    ProviderNotFound,
}
