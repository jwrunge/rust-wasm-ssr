mod config {
    enum ExtensionBehavior {
        Deny,
        Fetch,
        ProcessCode,
        FetchAndProcessCode,
        FetchAndProcessTemplate,
    }

    struct Config {
        default_behavior: ExtensionBehavior,
        extension_behaviors: Vec<(String, ExtensionBehavior)>
    }
}