use serde::Deserialize;
use std::fs;
use toml;

const TEST_CONFIG_PATH: &str = "./_test_files/config.toml";

#[derive(Debug)]
#[derive(Deserialize)]
pub enum ExtensionBehavior {
    Deny,                    //Disallow the extension from returning
    Fetch,                   //Fetch the file and return it
    ProcessCode,             //Map the request to a function
    FetchAndProcessCode,     //Fetch the file and process it with a function
    FetchAndProcessTemplate, //Fetch the file and process it as a template
}

#[derive(Deserialize)]
pub struct Config {
    pub listen_on: String,
    pub serve_root: String,
    pub watch_dirs: Vec<String>,
    pub default_behavior: ExtensionBehavior,
    pub extension_behaviors: Vec<(String, ExtensionBehavior)>,
}

impl Config {
    pub fn new(config_path: String) -> Config {
        //Get config file
        let config_contents =
            match fs::read_to_string(config_path) {
                Ok(file_contents) => file_contents,
                Err(_) => {
                    // Fall back to config test file if in debug mode
                    match fallback_file_if_debug() {
                        Some(path) => path,
                        None => panic!("Unable to read config file"),
                    }
                },
            };
        let config: Config =
            toml::from_str(&config_contents).expect("Unable to parse config file");

        config
    }
}

// If in debug mode, return the path to the test config file
#[cfg(debug_assertions)]
fn fallback_file_if_debug()-> Option<String> {
    Some(TEST_CONFIG_PATH.to_string())
}

#[cfg(not(debug_assertions))]
fn fallback_file_if_debug()-> Option<String> {
    None
}

/***
 * TESTS
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = Config::new(TEST_CONFIG_PATH.to_string());

        match config.default_behavior {
            ExtensionBehavior::Deny => (),
            _ => panic!("Default behavior should be 'Deny'"),
        }

        assert_eq!(config.extension_behaviors[0].0, String::from(".html"));
        match config.extension_behaviors[0].1 {
            ExtensionBehavior::Fetch => (),
            _ => panic!("Default behavior should be 'Fetch'"),
        }

        assert_eq!(config.extension_behaviors[1].0, String::from(".temp"));
        match config.extension_behaviors[1].1 {
            ExtensionBehavior::FetchAndProcessTemplate => (),
            _ => panic!("Default behavior should be 'Fetch'"),
        }
    }
}
