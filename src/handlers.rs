use std::{
    fs,
    path::PathBuf
};
use axum::routing::MethodRouter;

use super::config::Config;

pub fn assign_handlers(config: &Config) -> Vec<(PathBuf, MethodRouter)> {
    let mut routes = Vec::new();

    //Assign code response handlers
    
    //Assign file response handlers
    for route in assign_handler_from_public_static_routes(config.get_serve_root()) {
        routes.push(route);
    }

    routes
}

pub fn assign_handler_from_public_static_routes(dir: PathBuf) -> Vec<(PathBuf, MethodRouter)> {
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
                    let subroutes = assign_handler_from_public_static_routes(entry.path());
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
    fn test_assign_public_static_handlers() {
        let root = PathBuf::from("./_test_files/public_two_routes");
        let routes = assign_handler_from_public_static_routes(root);
        dbg!(&routes);
        println!("Found {} routes; expected 2", routes.len());
        assert_eq!(routes.len(), 2);
    }
}
