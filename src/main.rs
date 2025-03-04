use tracing::Value;
use tracing::{info, info_span};
use tracing_core::dispatch::set_global_default;
use tracing_subscriber::{prelude::*, FmtSubscriber, Registry};

fn main() {
    let fmt_subscriber = FmtSubscriber::new();
    let subscriber = Registry::default().with(fmt_subscriber);
    set_global_default(subscriber.into()).expect("setting default subscriber failed");

    let span = info_span!("My span...", field1 = 1, field2 = 2);
    let _guard = span.enter();
    info!("Let's call the macro!");

    // The hope here is, that if I manage to build the ValueSet and call
    // record_all in regular Rust code, I can probably manage to wrap it
    // up in a macro too. However, the results are a bit surprising.
    // New fields are added to the span, not replaced.
    let meta = span.metadata().unwrap();
    let field_set = meta.fields();
    let values = &[
        (
            &field_set.field("field1").unwrap(),
            Some(&"One" as &dyn Value),
        ),
        (
            &field_set.field("field2").unwrap(),
            Some(&"Two" as &dyn Value),
        ),
    ];
    let value_set = field_set.value_set(values);

    span.record_all(&value_set);

    info!("Macro has been called!");
}
