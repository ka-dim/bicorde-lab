use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
struct VersionResponse {
    version: &'static str,
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/version", get(version))
}

async fn root() -> &'static str {
    "bicorde server"
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub fn health_json() -> &'static str {
    r#"{"status":"ok"}"#
}

pub fn version_json() -> String {
    format!(r#"{{"version":"{}"}}"#, env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode, header::CONTENT_TYPE},
    };
    use tower::ServiceExt;

    #[test]
    fn health_payload_reports_ok() {
        assert_eq!(health_json(), r#"{"status":"ok"}"#);
    }

    #[test]
    fn version_payload_uses_package_version() {
        assert_eq!(version_json(), r#"{"version":"0.1.0"}"#);
    }

    #[tokio::test]
    async fn health_route_returns_ok_json() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("valid health request"),
            )
            .await
            .expect("health response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE),
            Some(&"application/json".parse().expect("valid content type"))
        );

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read health body");

        assert_eq!(&body[..], br#"{"status":"ok"}"#);
    }

    #[tokio::test]
    async fn version_route_returns_package_version() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/version")
                    .body(Body::empty())
                    .expect("valid version request"),
            )
            .await
            .expect("version response");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read version body");

        assert_eq!(&body[..], br#"{"version":"0.1.0"}"#);
    }

    #[tokio::test]
    async fn unknown_route_returns_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/unknown")
                    .body(Body::empty())
                    .expect("valid unknown request"),
            )
            .await
            .expect("unknown route response");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
