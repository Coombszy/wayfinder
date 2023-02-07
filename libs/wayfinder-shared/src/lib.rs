use thiserror::Error;

#[derive(Error, Debug)]
pub enum WayfindError<T> {
    #[error("{0}")]
    Godaddy(T),
    #[error("{0}")]
    Google(T),
}

pub struct Config {
    pub auth_key: Option<String>,
    pub auth_secret: String,
    pub domain: String,
    pub wait: u64,
}
