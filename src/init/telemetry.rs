use super::InitResult;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct TelemetryLifecycle {
    provider: SdkTracerProvider,
}

impl TelemetryLifecycle {
    pub fn setup() -> InitResult<TelemetryLifecycle> {
        let provider = Self::init_otlp_exporter()?;
        let lifecycle = TelemetryLifecycle { provider };

        lifecycle.init_opentelementry_tracing_subscriber()?;

        Ok(lifecycle)
    }

    pub fn shutdown(self) -> InitResult<()> {
        self.provider.force_flush()?;
        self.provider.shutdown()?;

        Ok(())
    }

    fn init_otlp_exporter() -> InitResult<SdkTracerProvider> {
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let otlp_exporter = SpanExporter::builder().with_tonic().build()?;

        let resource = Resource::builder()
            .with_service_name("devops-dreamland-app")
            .build();

        let provider = SdkTracerProvider::builder()
            .with_batch_exporter(otlp_exporter)
            .with_resource(resource)
            .build();

        Ok(provider)
    }

    fn init_opentelementry_tracing_subscriber(&self) -> InitResult<()> {
        let tracer = self.provider.tracer("app");

        let open_telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(open_telemetry_layer)
            .try_init()?;

        Ok(())
    }
}
