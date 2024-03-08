use std::fs;

use axum_demo::{
    error::AppError,
    route::{create_route, kafka_consumer::consume_and_print},
    setting::AppConfig,
};
use tracing::{error, info, Level};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        info!("This will be logged to stdout");
    });
    let file_contents =
        fs::read_to_string("/Users/yiny/rust_projects/axum-demo/src/config.example.yaml")
            .expect("LogRocket: Should have been able to read the file");
    let config: AppConfig =
        serde_yaml::from_str(&file_contents).map_err(|_| panic!("read config failed"))?;

    let app = create_route(config.clone()).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    print_start_info();
    tokio::task::spawn(consume_and_print(config.clone()));
    axum::serve(listener, app)
        .await
        .map_err(|_| panic!("start server error"))?;

    Ok(())
}

fn print_start_info() {
    info!("App is running on 127.0.0.1:3000");
    error!("App is running on 127.0.0.1:3000");
    let ascii = r"
    _____                                 ________                         
    /  _  \ ___  _____ __  _____           \______ \   ____   _____   ____  
   /  /_\  \\  \/  /  |  \/     \   ______  |    |  \_/ __ \ /     \ /  _ \ 
  /    |    \>    <|  |  /  Y Y  \ /_____/  |    `   \  ___/|  Y Y  (  <_> )
  \____|__  /__/\_ \____/|__|_|  /         /_______  /\___  >__|_|  /\____/ 
          \/      \/           \/                  \/     \/      \/        
    ";
    eprintln!("{ascii}");
}
