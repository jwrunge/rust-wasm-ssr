use serde::Deserialize;
use std::{
    fs, 
    collections::HashMap,
    path::PathBuf, 
    cell::RefCell
};
use toml;

#[cfg(debug_assertions)]
const TEST_CONFIG_PATH: &'static str = "./_test_files/config.toml";

#[derive(Deserialize)]
#[derive(Clone)]
pub enum ExtensionBehavior {
    Deny,                    //Disallow the extension from returning
    Fetch,                   //Fetch the file and return it
    FetchAndProcessCode,     //Fetch the file and process it with a function
}

#[derive(Deserialize)]
pub struct RoutingOptions {
    pub default_behavior: RefCell<Option<ExtensionBehavior>>,
    pub extension_behaviors: RefCell<Option<Vec<(String, ExtensionBehavior)>>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub listen_on: Option<String>,
    pub serve_root: Option<String>,
    pub watch_dirs: Option<Vec<String>>,
    pub routing: RefCell<Option<RoutingOptions>>,
}

impl Config {
    pub fn new(args: impl Iterator<Item = String>) -> Config {
        let config: Config =
            toml::from_str(
                &get_config_file_contents(
                    get_config_path_or_fallback(args)
                )
            )
            .expect("Unable to parse config file");

        //Set route prop defaults
        let default_default_behavior = ExtensionBehavior::Deny;
        let default_extension_behaviors = vec![("html".to_string(), ExtensionBehavior::Fetch)];

        //Set route prop
        if(*config.routing.borrow()).is_none() {
            config.set_routing_options(RoutingOptions {
                default_behavior: RefCell::new(Some(default_default_behavior)),
                extension_behaviors: RefCell::new(Some(default_extension_behaviors)),
            });
        }
        else {
            if(*config.routing.borrow()).as_ref().unwrap().default_behavior.borrow().is_none() {
                config.set_routing_default_behavior(default_default_behavior);
            }
            if(*config.routing.borrow()).as_ref().unwrap().extension_behaviors.borrow().is_none() {
                config.set_routing_extension_behaviors(default_extension_behaviors);
            }
        }

        config
    }

    /*
     * Internal mutations
    */
    fn set_routing_options(&self, routing_options: RoutingOptions) {
        *self.routing.borrow_mut() = Some(routing_options);
    }

    fn set_routing_default_behavior(&self, behavior: ExtensionBehavior) {
        //If routing is uninitialized, return
        if(*self.routing.borrow()).is_none() {
            return;
        }

        //Set default behavior
        *self.routing.borrow_mut().as_mut().unwrap().default_behavior.borrow_mut() = Some(behavior);
        // *self.routing.borrow_mut().unwrap().default_behavior.borrow_mut() = Some(behavior);
    }

    fn set_routing_extension_behaviors(&self, behaviors: Vec<(String, ExtensionBehavior)>) {
        //If routing is uninitialized, return
        if(*self.routing.borrow()).is_none() {
            return;
        }

        //Set extension behaviors
        *self.routing.borrow_mut().as_mut().unwrap().extension_behaviors.borrow_mut() = Some(behaviors);
    }

    /*
     * Getters
    */
    pub fn get_listen_on(&self)-> ([u8; 4], u16) {
        //Get config IP and port; default to 127.0.0.1:8080
        let mut split = match &self.listen_on {
            Some(s) => s.split(":"),
            None => "127.0.0.1:8080".split(":"),
        };

        //Parse out IP (default to 127.0.0.1)
        let ip: [u8; 4] = match split.next() {
            Some("localhost") => [127, 0, 0, 1],
            Some(s) => match s.split(".") {
                parts => {
                    let mut ip: [u8; 4] = [0, 0, 0, 0];
                    for (i, part) in parts.enumerate() {
                        ip[i] = match part.parse::<u8>() {
                            Ok(p) => p,
                            Err(_) => 0,
                        };
                    }
                    ip
                }
            },
            None => [127, 0, 0, 1],
        };

        //Set port (default to 8080)
        let port = match split.next() {
            Some(p) => match p.parse::<u16>() {
                Ok(p) => p,
                Err(_) => 8080,
            },
            None => 8080,
        };

        (ip, port)
    }

    pub fn get_serve_root(&self)-> PathBuf {
        //Get config serve_root; default to "./public"
        let serve_root = match &self.serve_root {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from("./public"),
        };

        println!("Serve root: {}", serve_root.display());

        serve_root
    }

    pub fn _get_watch_dirs(&self)-> Vec<PathBuf> {
        match self.watch_dirs {
            Some(ref dirs) => {
                dirs.iter().map(|dir| PathBuf::from(dir)).collect()
            },
            None => Vec::new(),
        }
    }

    pub fn get_routing_default_behavior(&self)-> ExtensionBehavior {
        match (*self.routing.borrow().as_ref().unwrap().default_behavior.borrow()).clone() {
            Some(behavior) => behavior,
            None => panic!("Default behavior must be 'Deny' or 'ProcessCode'"),
        }
    }

    pub fn get_routing_extension_behaviors(&self)-> HashMap<String, ExtensionBehavior> {
        let mut behaviors = HashMap::new();
        match &self.routing.borrow().as_ref().unwrap().extension_behaviors.borrow().clone() {
            Some(behaviors_vec) => {
                for (extension, behavior) in behaviors_vec {
                    behaviors.insert(extension.clone(), behavior.clone());
                }
            },
            None => (),
        }
        behaviors
    }
}

fn get_config_path_or_fallback(mut args: impl Iterator<Item = String>) -> String {
    match args.nth(1) {
        Some(path) => path,
        None => fallback_file(),
    }
}

fn get_config_file_contents(config_path: String) -> String {
    match fs::read_to_string(config_path) {
        Ok(file_contents) => file_contents,
        Err(_) => {
            // Fall back to config test file if in debug mode
            fallback_file()
        },
    }
}

// If in debug mode, return the path to the test config file
#[cfg(debug_assertions)]
fn fallback_file()-> String {
    TEST_CONFIG_PATH.to_string()
}

#[cfg(not(debug_assertions))]
fn fallback_file()-> String {
    "config.toml".to_string()
}

/***
 * TESTS
 */
#[cfg(test)]
mod tests {
    use super::*;

    enum TestConfig {
        Default,
        Uninitialized,
    }

    fn get_test_config(config_type: TestConfig) -> Config {
        Config::new(
            match config_type {
                TestConfig::Default => {
                    vec![
                        String::from(""),
                        String::from(TEST_CONFIG_PATH),
                    ].into_iter()
                },
                TestConfig::Uninitialized => {
                    vec![
                        String::from(""),
                        String::from("_test_files/config.uninitialized.toml"),
                    ].into_iter()
                },
            }
        )
    }

    // Test config file address return
    #[test]
    fn test_config_ip_port_return() {
        let config = get_test_config(TestConfig::Default);
        let (ip, port) = config.get_listen_on();
        println!("IP: {}.{}.{}.{}, PORT: {}", ip[0], ip[1], ip[2], ip[3], port);
        assert_eq!(ip, [127, 0, 0, 1]);
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_get_serve_root() {
        //Test from standard config file
        let config = get_test_config(TestConfig::Default);
        let serve_root = config.get_serve_root();
        println!("Serve root: {}", serve_root.display());
        assert_eq!(serve_root, PathBuf::from("./public"));

        //Test from config file, unitialized serve root
        let config = get_test_config(TestConfig::Uninitialized);
        let serve_root = config.get_serve_root();
        println!("Serve root: {}", serve_root.display());
        assert_eq!(serve_root, PathBuf::from("./public"));
    }

    #[test]
    fn test_behaviors() {
        //Test default behavior from standard config file
        let config = get_test_config(TestConfig::Default);
        assert!(match *config.routing.borrow().as_ref().unwrap().default_behavior.borrow() {
            Some(ExtensionBehavior::Fetch) => true,
            _ => panic!("Default behavior is invalid; should be 'Deny', 'Fetch', or 'FetchAndProcessCode'"),
        });

        //Test default behavior from config file, unitialized default behavior
        let config = get_test_config(TestConfig::Uninitialized);
        assert!(match *config.routing.borrow().as_ref().unwrap().default_behavior.borrow() {
            Some(ExtensionBehavior::Deny) => true,
            _ => panic!("Default behavior is invalid; should be 'Deny', 'Fetch', or 'FetchAndProcessCode'"),
        });
    }
}
