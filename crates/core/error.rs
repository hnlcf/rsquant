use quant_api::Error as ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Quant api error by `{0}`")]
    Api(#[from] ApiError),

    #[error("Quant io error by `{0}`")]
    IO(#[from] std::io::Error),
}
