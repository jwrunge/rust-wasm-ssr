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

fn assign_handlers(dir: Option<&str>) -> Vec<(&str, MethodRouter)> {
    //Check for root dir; default to 'public'
    let dir = match dir {
        Some(s) => s,
        None => return vec![],
    };

    //Look for routes in serve_root
    let dir_path = fs::read_dir(dir).expect(&format!("Unable to access serve_root directory '{}'", &dir));
    let mut routes = Vec::new();
    for entry in dir_path {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    //Recurse into subdirectories
                    let subroutes = assign_handlers(path.to_str());
                    for route in subroutes {
                        routes.push(route)
                    }
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