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
    let config = config::Config::new(env::args());

    //Set up routes
    let mut router: Router = Router::new();
    for (route, handler) in handlers::assign_handlers(&config) {
        let path_str = match route.to_str() {
            Some(s) => s,
            None => continue,
        };

        //Remove leading . in path_str
        let path_str = if path_str.starts_with('.') {
            &path_str[1..]
        } else {
            path_str
        };

        println!("Assigning handler for route '{}'", path_str);
        router = router.route(path_str, handler);
    }

    router = router.route("/", get(|| async {
        "Hello, YOU!"
    }));
    //Startup mssage
    {
        let listen_on = config.get_listen_on();

        println!("Server starting. Serving from {} on {:?}.{:?}.{:?}.{:?}:{:?}",
            config.get_serve_root().display(),
            listen_on.0[0],
            listen_on.0[1],
            listen_on.0[2],
            listen_on.0[3],
            listen_on.1,
        );
    }

    //Bind to port and serve
    axum::Server::bind(
        &SocketAddr::from((&config).get_listen_on())
    )
    .serve(router.into_make_service())
    .await
    .expect("Unable to bind to port");
}
