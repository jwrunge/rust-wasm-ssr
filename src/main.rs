use axum::routing::{get, Router};
use std::{
    env,
    net::SocketAddr,
};

mod config;
mod handlers;

#[tokio::main]
async fn main() {
    //Load config
    let cfg = config::Config::new(env::args());

    //Set up routes
    let mut router: Router = Router::new();
    for (route, handler) in handlers::assign_handlers(&cfg) {
        let path_str = match route.to_str() {
            Some(s) => s,
            None => continue,
        };
        router = router.route(path_str, get(handler));
    }

    //Bind to port and serve
    axum::Server::bind(
        &SocketAddr::from((&cfg).get_listen_on())
    )
    .serve(router.into_make_service())
    .await
    .expect("Unable to bind to port");
}
