pub enum PropagationError {
    UnsupportedFormat,
    SpanContextNotFound,
    InvalidSpanContext,
    InvalidCarrier,
    SpanContextCorrupted
}

pub type PropagationOption = Option<PropagationError>;

pub trait TextMapReader {
    fn foreach_key(&self, f: &Fn(&str, &str) -> PropagationOption)
        -> PropagationOption;
}

pub trait TextMapWriter {
    fn set(&mut self, key: &str, value: &str);
}
