use axum::{routing::{get, MethodRouter}, Router};
use std::{
    fs,
    env,
    str,
    net::SocketAddr,
};

mod config;

#[tokio::main]
async fn main() {
    let cfg = config::Config::new(env::args());
    let (ip, port) = cfg.parse_ip_port();

    //Set up app
    let mut router = Router::new();
    let addr = SocketAddr::from((ip, port));

    //Set up routes
    for (route, handler) in assign_handlers(cfg.serve_root.as_deref()) {
        router = router.route(route, get(handler));
    }

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn assign_handlers(root: Option<&str>) -> Vec<(&str, MethodRouter)> {
    //Check for serve_root; default to 'public'
    let root = match root {
        Some(s) => s,
        None => "./public",
    };

    //Look for routes in serve_root
    let path = fs::read_dir(root).expect(&format!("Unable to access serve_root directory '{}'", &root));
    let routes = Vec::new();
    for entry in path {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    //Recurse into directory
                } else if path.is_file() {

                }
            },
            Err(_) => {},
        }
    }

    routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_handlers() {
        let root = Some("./public");
        let routes = assign_handlers(root);
        assert_eq!(routes.len(), 2);
    }
}