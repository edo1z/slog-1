use std::{error::Error, fmt};
use tracing::{error, instrument};

#[derive(Debug)]
enum ServiceError {
    HogeError(String),
}

impl From<String> for ServiceError {
    fn from(err: String) -> Self {
        Self::HogeError(err)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HogeError(err) => write!(f, "HogeError: {}", err),
        }
    }
}
impl Error for ServiceError {}

#[instrument]
async fn service(x: i32, y: i32) -> Result<(), ServiceError> {
    if x == y {
        let err = ServiceError::from("x == y".to_string());
        error!("{} ({}:{})", err, file!(), line!());
        return Err(err);
    }
    Ok(())
}

#[instrument]
async fn use_case(x: i32, y: i32) {
    match service(x, y).await {
        Ok(_) => println!("use_case success"),
        Err(_) => println!("use_case fail"),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let x = 1;
    let y = 1;
    use_case(x, y).await;
}
