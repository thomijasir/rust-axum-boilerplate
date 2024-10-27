mod constant;
mod routes;

use crate::constant::app_config::AppConfig;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  let config = AppConfig::load();
  println!("config: {:?}", config);

  //   let app = routes::route_app();
  //   // run our app with hyper, listening globally on port 3000
  //   let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  //   axum::serve(listener, app).await.unwrap();

  Ok(())
}
// async fn main() {
//   // build our application with a single route
//   let app = routes::route_app();

//   // run our app with hyper, listening globally on port 3000
//   let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//   axum::serve(listener, app).await.unwrap();
// }
