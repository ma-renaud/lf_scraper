#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    SurrealEmbedded(#[from] surrealdb::error::Db),

    #[error(transparent)]
    SurrealApi(#[from] surrealdb::error::Api),
}