use serde::Deserialize;
use std::fs;
use toml;

const TEST_CONFIG_PATH: &'static str = "./_test_files/config.toml";

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
    pub listen_on: Option<String>,
    pub serve_root: Option<String>,
    pub watch_dirs: Option<Vec<String>>,
    pub default_behavior: Option<ExtensionBehavior>,
    pub extension_behaviors: Option<Vec<(String, ExtensionBehavior)>>,
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
        config
    }

    pub fn parse_ip_port(&self)-> ([u8; 4], u16) {
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

    // Test config file load
    #[test]
    fn test_new_config() {
        let args = vec![
            String::from(""),
            String::from(TEST_CONFIG_PATH),
        ].into_iter();

        let config = Config::new(args);

        match config.default_behavior {
            Some(ExtensionBehavior::Deny) => (),
            _ => panic!("Default behavior should be 'Deny'"),
        }

        assert_eq!(config.extension_behaviors.as_ref().unwrap()[0].0, String::from(".html"));
        match config.extension_behaviors.as_ref().unwrap()[0].1 {
            ExtensionBehavior::Fetch => (),
            _ => panic!("Default behavior should be 'Fetch'"),
        }

        assert_eq!(config.extension_behaviors.as_ref().unwrap()[1].0, String::from(".temp"));
        match config.extension_behaviors.as_ref().unwrap()[1].1 {
            ExtensionBehavior::FetchAndProcessTemplate => (),
            _ => panic!("Default behavior should be 'Fetch'"),
        }
    }

    // Test config file address return
    #[test]
    fn test_config_ip_port_return() {
        let args = vec![
            String::from(""),
            String::from(TEST_CONFIG_PATH),
        ].into_iter();

        let config = Config::new(args);

        let (ip, port) = config.parse_ip_port();
        println!("IP: {}.{}.{}.{}, PORT: {}", ip[0], ip[1], ip[2], ip[3], port);
    }
}
