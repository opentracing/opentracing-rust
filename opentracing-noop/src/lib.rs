//! This crate provides a simple and cheap "noop" implementation of the
//! tracing API and all of its sub-components.
#![doc(html_root_url = "https://docs.rs/opentracing-noop/0.1.0")]

extern crate opentracing_api;

use opentracing_api::SpanContext;
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
