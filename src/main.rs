use tracing::{info, info_span, record_all};
use tracing_core::dispatch::set_global_default;
use tracing_subscriber::{prelude::*, FmtSubscriber, Registry};

fn main() {
    let fmt_subscriber = FmtSubscriber::new();
    let subscriber = Registry::default().with(fmt_subscriber);
    set_global_default(subscriber.into()).expect("setting default subscriber failed");

    let span = info_span!("My span...", field1 = 1, field2 = 2);
    let _guard = span.enter();
    info!("Let's call the macro!");
    record_all!(span, field1 = "One", field2 = "Two");
    info!("Macro has been called!");
}
