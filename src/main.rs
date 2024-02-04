// #[tokio::main]
// async fn main() {
//     // let file_contents =
//     //     fs::read_to_string("/Users/yiny/rust_projects/axum-demo/src/config.example.yaml").expect("LogRocket: Should have been able to read the file");
//     // println!("{file_contents}");
//     // let config: GlobalConfig = serde_yaml::from_str(&file_contents).unwrap();
//     // println!("{config:?}");
//     run().await
// }
mod error;
mod middleware;
mod model;
mod route;

use route::create_route;

#[tokio::main]
async fn main() {
    let app = create_route();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
