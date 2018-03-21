//! This crate provides a simple and cheap "noop" implementation of the
//! tracing API and all of its sub-components.
#![doc(html_root_url = "https://docs.rs/opentracing-noop/0.1.0")]

extern crate opentracing_api;

use opentracing_api::{FinishedSpan, Span, SpanContext, TagValue};
use std::iter::{empty, Empty};

/// The `NoopSpanContext` just returns an empty iterator on
/// baggage items and all of its associated content.
pub struct NoopSpanContext {}

impl Default for NoopSpanContext {
    fn default() -> Self {
        NoopSpanContext {}
    }
}

impl<'a> SpanContext<'a> for NoopSpanContext {
    type Iter = Empty<(&'a String, &'a String)>;

    fn baggage_items(&'a self) -> Self::Iter {
        empty()
    }
}

pub struct NoopSpan {
    ctx: NoopSpanContext,
    op_name: String,
}

impl NoopSpan {
    pub fn new() -> Self {
        Self {
            ctx: NoopSpanContext::default(),
            op_name: "NoopSpan".into(),
        }
    }
}

impl<'a> Span<'a> for NoopSpan {
    type Context = NoopSpanContext;

    fn context(&self) -> &Self::Context {
        &self.ctx
    }

    fn set_tag<S>(&mut self, _key: S, _value: TagValue)
    where
        S: Into<String>,
    {
    }

    fn unset_tag<S>(&mut self, _key: S)
    where
        S: Into<String>,
    {
    }

    fn tag<S>(&self, _key: S) -> Option<&TagValue>
    where
        S: Into<String>,
    {
        None
    }

    fn log(&mut self, _event: String) {}

    fn log_at(&mut self, _timestamp: u64, _event: String) {}

    fn set_baggage_item<S>(&mut self, _key: S, _value: String)
    where
        S: Into<String>,
    {
    }

    fn unset_baggage_item<S>(&mut self, _key: S)
    where
        S: Into<String>,
    {
    }

    fn baggage_item<S>(&self, _key: S) -> Option<&String>
    where
        S: Into<String>,
    {
        None
    }

    fn set_operation_name<S>(&mut self, _name: S)
    where
        S: Into<String>,
    {
    }

    fn operation_name(&self) -> &String {
        &self.op_name
    }

    fn finish(self) -> FinishedSpan<Self::Context> {
        self.finish_at(0)
    }

    fn finish_at(self, _timestamp: u64) -> FinishedSpan<Self::Context> {
        FinishedSpan::new(self.ctx)
    }
}

#[cfg(test)]
mod tests {

    use super::NoopSpanContext;
    use opentracing_api::SpanContext;

    #[test]
    fn test_empty_baggage() {
        let ctx = NoopSpanContext::default();
        assert_eq!(None, ctx.baggage_items().next());
    }

}
