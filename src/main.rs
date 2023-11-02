use axum::{routing::{get, MethodRouter}, Router};
use std::{
    fs,
    env,
    net::SocketAddr,
    path::PathBuf,
};

mod config;

#[tokio::main]
async fn main() {
    let cfg = config::Config::new(env::args());
    let (ip, port) = cfg.get_listen_on();

    //Set up app
    let mut router = Router::new();
    let addr = SocketAddr::from((ip, port));

    //Set up routes
    for (route, handler) in assign_handlers(cfg.get_serve_root()) {
        let path_str = match route.to_str() {
            Some(s) => s,
            None => continue,
        };
        router = router.route(path_str, get(handler));
    }

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn assign_handlers(dir: PathBuf) -> Vec<(PathBuf, MethodRouter)> {
    //Look for routes in serve_root
    let dir_contents = fs::read_dir(&dir).expect(&format!("Unable to access serve_root directory '{}'", dir.display()));
    let mut routes = Vec::new();
    for entry in dir_contents {
        match entry {
            Ok(entry) => {
                let file_type = match entry.file_type() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                if file_type.is_dir() {
                    //Recurse into subdirectories
                    let subroutes = assign_handlers(entry.path());
                    for route in subroutes {
                        routes.push(route)
                    }
                } else if file_type.is_file() {
                    //Set up handler for file
                    routes.push((entry.path(), MethodRouter::new()))
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
        let root = PathBuf::from("./_test_files/public_two_routes");
        let routes = assign_handlers(root);
        dbg!(&routes);
        println!("Found {} routes; expected 2", routes.len());
        assert_eq!(routes.len(), 2);
    }
}
