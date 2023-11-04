use axum::routing::get;

use std::{
    fs,
    path::PathBuf, collections::HashMap
};
use axum::routing::MethodRouter;

use super::config::Config;
use super::config::ExtensionBehavior;

pub fn assign_handlers(config: &Config) -> HashMap<PathBuf, MethodRouter> {
    let mut routes: HashMap<PathBuf, MethodRouter> = HashMap::new();

    //Assign file response handlers
    for route in assign_handler_from_public_static_routes(config, config.get_serve_root()) {
        routes.insert(route.0, route.1);
    }

    //Assign code response handler if default_behavior is ProcessCode
    match config.get_default_behavior() {
        ExtensionBehavior::ProcessCode => {

        },
        _ => (),
    }

    routes
}

pub fn assign_handler_from_public_static_routes(config: &Config, dir: PathBuf) -> HashMap<PathBuf, MethodRouter> {
    //Look for routes in serve_root
    let dir_contents = fs::read_dir(&dir).expect(&format!("Unable to access serve_root directory '{}'", dir.display()));
    let mut routes = HashMap::new();
    for entry in dir_contents {
        match entry {
            Ok(entry) => {
                let file_type = match entry.file_type() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                if file_type.is_dir() {
                    //Recurse into subdirectories
                    let subroutes = assign_handler_from_public_static_routes(config, entry.path());
                    for route in subroutes {
                        routes.insert(route.0, route.1);
                    }
                } else if file_type.is_file() {
                    //Get file extension
                    let path = entry.path();
                    let file_extension = match path.extension() {
                        Some(ext) => ext,
                        None => continue,
                    };
                    
                    //Get behavior for file extension or default
                    let behavior = match file_extension.to_str() {
                        Some(ext) => {
                            let behaviors = config.get_extension_behaviors();
                            match behaviors.get(ext) {
                                Some(behavior) => behavior.clone(),
                                None => config.get_default_behavior(),
                            }
                        },
                        None => continue,
                    };

                    //Add handler to routes based on behavior
                    let method: MethodRouter;
                    match behavior {
                        ExtensionBehavior::Deny => {
                            println!("Denying '{}'", path.display());
                            continue
                        },
                        ExtensionBehavior::Fetch => {
                            println!("Fetching '{}'", path.display());
                            method = get(|| async {
                                "Hello, World!"
                            })
                        },
                        ExtensionBehavior::ProcessCode => {
                            println!("Processing '{}'", path.display());
                            method = get(|| async {
                                "Hello, World!"
                            })
                        },
                        ExtensionBehavior::FetchAndProcessCode => {
                            println!("Fetching and processing '{}'", path.display());
                            method = get(|| async {
                                "Hello, World!"
                            })
                        }
                    }

                    //Return hashmap
                    routes.insert(path, method);
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
        let args = vec![
            String::from(""),
            String::from("./_test_files/config.toml"),
        ].into_iter();

        let routes = assign_handler_from_public_static_routes(&Config::new(args), root);
        dbg!(&routes);
        println!("Found {} routes; expected 2", routes.len());
        assert_eq!(routes.len(), 2);
    }
}
