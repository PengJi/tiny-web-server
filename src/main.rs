use log::{debug, info, warn, error};
use log4rs;

mod example_handlers;
mod example_routers;

use example_routers::get_router;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("start program");

    debug!("get router");
    let example_app = get_router();

    debug!("start server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(example_app.into_make_service())
        .await
        .unwrap();
}
