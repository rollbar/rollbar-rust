error_chain! {
    errors {
        MissingAccessToken(name: String) {
            description("access_token is missing"),
            display("app:{} is missing params.access_token", name),
        }
        MissingTargets(name: String) {
            description("targets is missing"),
            display("app:{} is missing targets", name),
        }
        ProcessFileFailed {
            description("process_file failed"),
            display("process_file failed"),
        }
        BadInput(key: &'static str, value: String) {
            description("bad configuration file input"),
            display("{} had a bad value: {}", key, value),
        }
    }
}
