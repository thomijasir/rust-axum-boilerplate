pub struct ApplicationServer;

impl ApplicationServer {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn start(&self) {
    // build our application with a single route
    let app = routes::route_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
  }
}
