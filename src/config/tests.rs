#[cfg(test)]
mod tests {
    #[test]
    fn test_new_config() {
        let config = Config::new(String::from("_test_files/config.toml"));
        
        match config.default_behavior {
            ExtensionBehavior::Deny => (),
            _ => panic!("Default behavior should be 'Deny'")
        }

        assert_eq!(config.extension_behaviors[0].0, String::from(".html"));
        match config.extension_behaviors[0].1 {
            ExtensionBehavior::Fetch => (),
            _ => panic!("Default behavior should be 'Fetch'")
        }

        assert_eq!(config.extension_behaviors[1].0, String::from(".temp"));
        match config.extension_behaviors[1].1 {
            ExtensionBehavior::FetchAndProcessTemplate => (),
            _ => panic!("Default behavior should be 'Fetch'")
        }
    }
}