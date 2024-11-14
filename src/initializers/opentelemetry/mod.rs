use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct OpenTelemetryInitializer;

#[async_trait]
impl Initializer for OpenTelemetryInitializer {
    fn name(&self) -> String {
        "opentelemetry".to_string()
    }

    async fn before_run(&self, _app_context: &AppContext) -> Result<()> {
        match init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers() {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Failed to initialize opentelemetry subscriber: {:?}", e);
                Err(Error::Message(e.to_string()))
            }
        }
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let router = router
            .layer(OtelInResponseLayer)
            .layer(OtelAxumLayer::default());
        Ok(router)
    }
}
