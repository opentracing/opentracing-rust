use std::time::Instant;

pub trait SpanContext {
    fn foreach_baggage_item(&self, f: &Fn(&str, &str) -> bool);
}

pub struct LogRecord {
    Instant timestamp;
    Vec<log::Field> fields;
}

pub struct FinishSpanOptions {
    Instant finish_time;
    Vec<LogRecord> log_records;
}

pub trait Span {
    fn finish(&mut self);

    fn finish_with_options(&mut self, &FinishSpanOptions options);

    fn set_operation_name(&mut self, &str operation_name);

    fn set_tag(&mut self, &str key, &util::Value value);

    fn log_fields(&mut self, &Vec<log::Field> fields);

    fn set_baggage_item(&mut self, &str restricted_key, &str value);

    fn baggage_item(&self, &str restricted_key) -> str;

    fn tracer(&self) -> &Tracer;
}
