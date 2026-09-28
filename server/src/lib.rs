pub fn health_json() -> &'static str {
    return r#"{"status":"ok"}"#;
}

pub fn version_json() -> String {
    format!(r#"{{"version":"{}"}}"#, env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_payload_reports_ok() {
        assert_eq!(health_json(), r#"{"status":"ok"}"#);
    }

    #[test]
    fn version_payload_uses_package_version() {
        assert_eq!(version_json(), r#"{"version":"0.1.0"}"#);
    }
}
