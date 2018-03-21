use SpanContext;
use TagValue;

/// The `Span` represents the OpenTracing specification's Span contract.
pub trait Span<'a> {
    /// The associated `SpanContext`.
    type Context: SpanContext<'a>;

    /// Retrieve the associated `SpanContext`.
    ///
    /// This may be called any time, including after `finish`.
    fn context(&self) -> &Self::Context;

    /// Sets a key:value tag on the `Span`.
    fn set_tag<S>(&mut self, key: S, value: TagValue)
    where
        S: Into<String>;

    /// Allows to unset a tag based on the given key. Noop if
    /// it doesn't exist.
    fn unset_tag<S>(&mut self, key: S)
    where
        S: Into<String>;

    /// Returns a tag value if set, none otherwise
    fn tag<S>(&self, key: S) -> Option<&TagValue>
    where
        S: Into<String>;

    /// Record an event at the current walltime timestamp.
    fn log(&mut self, event: String);

    /// Record an event at the given walltime timestamp.
    fn log_at(&mut self, timestamp: u64, event: String);

    /// Sets a baggage item in the Span (and its SpanContext) as a key/value pair.
    fn set_baggage_item<S>(&mut self, key: S, value: String)
    where
        S: Into<String>;

    /// Allows to unset a baggage item based on the given key. Noop if
    /// it doesn't exist.
    fn unset_baggage_item<S>(&mut self, key: S)
    where
        S: Into<String>;

    /// the value of the baggage item identified by the given key, or None if no such item
    /// could be found.
    fn baggage_item<S>(&self, key: S) -> Option<&String>
    where
        S: Into<String>;

    /// Sets the string name for the logical operation this span represents.
    fn set_operation_name<S>(&mut self, name: S)
    where
        S: Into<String>;

    /// Returns the operation name if set, None otherwise.
    fn operation_name(&self) -> &String;

    /// Sets the end timestamp to now and finishes (records) the span.
    fn finish(self) -> FinishedSpan<Self::Context>;

    /// Sets an explicit end timestamp and finishes (records) the span.
    fn finish_at(self, timestamp: u64) -> FinishedSpan<Self::Context>;
}

pub struct FinishedSpan<C> {
    context: C,
}

impl<'a, C> FinishedSpan<C>
where
    C: SpanContext<'a>,
{
    pub fn new(context: C) -> Self {
        FinishedSpan { context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }
}
