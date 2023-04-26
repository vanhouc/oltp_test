use opentelemetry::sdk::export::trace::stdout;
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

fn main() {
    // Install a new OpenTelemetry trace pipeline
    let tracer = stdout::new_pipeline().install_simple();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });
}
