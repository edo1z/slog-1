use std::{error::Error, fmt};
use tracing::{error, instrument};

// カスタムエラータイプ
#[derive(Debug)]
struct MyRepoError {
    message: String,
    file: &'static str,
    line: u32,
}

impl MyRepoError {
    fn new(message: String, file: &'static str, line: u32) -> MyRepoError {
        MyRepoError {
            message,
            file,
            line,
        }
    }
}

impl fmt::Display for MyRepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}:{})", self.message, self.file, self.line)
    }
}

impl Error for MyRepoError {}

// MyServiceErrorは、内部エラー（MyRepoError）をラップする
#[derive(Debug)]
struct MyServiceError {
    source: MyRepoError,
    file: &'static str,
    line: u32,
}

impl MyServiceError {
    fn new(source: MyRepoError, file: &'static str, line: u32) -> MyServiceError {
        MyServiceError { source, file, line }
    }
}

impl fmt::Display for MyServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <- ({}:{})", self.source, self.file, self.line)
    }
}

impl Error for MyServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// 非同期のリポジトリ関数
#[instrument]
async fn repository(a: i32, b: i32) -> Result<(), MyRepoError> {
    Err(MyRepoError::new(
        "repository ERROR".to_string(),
        file!(),
        line!(),
    ))
}

// 非同期のサービス関数
#[instrument]
async fn service(a: i32, b: i32) -> Result<(), MyServiceError> {
    repository(a, b)
        .await
        .map_err(|e| MyServiceError::new(e, file!(), line!()))
}

// 非同期のユースケース関数
#[instrument]
async fn use_case(x: i32, y: i32) {
    if let Err(e) = service(x * 2, y * 2).await {
        error!("use_case ERROR: {}", e);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let x = 1;
    let y = 2;
    use_case(x, y).await;
}
