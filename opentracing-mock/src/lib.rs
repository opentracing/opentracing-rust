//! This crate provides a mocking implementation of the tracing
//! API, especially useful for testing.
#![doc(html_root_url = "https://docs.rs/opentracing-mock/0.1.0")]

extern crate opentracing_api;

use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use opentracing_api::{FinishedSpan, Span, SpanContext, TagValue};

pub struct MockSpanContext {
    baggage: HashMap<String, String>,
}

impl MockSpanContext {
    /// Create a new `MockSpanContext` with the given baggage.
    fn new(baggage: HashMap<String, String>) -> Self {
        MockSpanContext { baggage }
    }

    /// Create a new `MockSpanContext` with empty baggage.
    fn empty() -> Self {
        MockSpanContext::new(HashMap::new())
    }

    fn set_baggage_item(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }

    fn unset_baggage_item(&mut self, key: &String) {
        self.baggage.remove(key);
    }

    fn baggage_item(&self, key: &String) -> Option<&String> {
        self.baggage.get(key)
    }
}

impl<'a> SpanContext<'a> for MockSpanContext {
    type Iter = HashMapIter<'a, String, String>;

    fn baggage_items(&'a self) -> Self::Iter {
        self.baggage.iter()
    }
}

pub struct MockSpan {
    ctx: MockSpanContext,
    tags: HashMap<String, TagValue>,
    op_name: String,
}

impl MockSpan {
    pub fn new<S>(op_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            ctx: MockSpanContext::empty(),
            tags: HashMap::new(),
            op_name: op_name.into(),
        }
    }
}

impl<'a> Span<'a> for MockSpan {
    type Context = MockSpanContext;

    fn context(&self) -> &Self::Context {
        &self.ctx
    }

    fn set_tag<S>(&mut self, key: S, value: TagValue)
    where
        S: Into<String>,
    {
        self.tags.insert(key.into(), value);
    }

    fn unset_tag<S>(&mut self, key: S)
    where
        S: Into<String>,
    {
        self.tags.remove(&key.into());
    }

    fn tag<S>(&self, key: S) -> Option<&TagValue>
    where
        S: Into<String>,
    {
        self.tags.get(&key.into())
    }

    fn log(&mut self, _event: String) {
        unimplemented!()
    }

    fn log_at(&mut self, _timestamp: u64, _event: String) {
        unimplemented!()
    }

    fn set_baggage_item<S>(&mut self, key: S, value: String)
    where
        S: Into<String>,
    {
        self.ctx.set_baggage_item(key.into(), value);
    }

    fn unset_baggage_item<S>(&mut self, key: S)
    where
        S: Into<String>,
    {
        self.ctx.unset_baggage_item(&key.into())
    }

    fn baggage_item<S>(&self, key: S) -> Option<&String>
    where
        S: Into<String>,
    {
        self.ctx.baggage_item(&key.into())
    }

    fn set_operation_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.op_name = name.into();
    }

    fn operation_name(&self) -> &String {
        &self.op_name
    }

    fn finish(self) -> FinishedSpan<Self::Context> {
        unimplemented!()
    }

    fn finish_at(self, _timestamp: u64) -> FinishedSpan<Self::Context> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::{MockSpan, MockSpanContext};
    use opentracing_api::{Span, SpanContext, TagValue};
    use std::collections::HashMap;

    #[test]
    fn test_map_baggage() {
        let mut items = HashMap::new();
        items.insert("key".into(), "value".into());

        let ctx = MockSpanContext::new(items);
        let mut iter = ctx.baggage_items();
        assert_eq!(
            Some((&String::from("key"), &String::from("value"))),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_set_get_unset_tag() {
        let mut span = MockSpan::new("myop");
        assert_eq!(None, span.tag("key"));
        span.set_tag("key", TagValue::String("some content".into()));
        assert_eq!(
            Some(&TagValue::String("some content".into())),
            span.tag("key")
        );
        span.unset_tag("key");
        assert_eq!(None, span.tag("key"));
    }

    #[test]
    fn test_set_get_baggage() {
        let mut span = MockSpan::new("myop");
        assert_eq!(None, span.baggage_item("key"));
        span.set_baggage_item("key", "value".into());
        assert_eq!(Some(&String::from("value")), span.baggage_item("key"));
        span.unset_baggage_item("key");
        assert_eq!(None, span.baggage_item("key"));
    }

    #[test]
    fn test_set_get_operation_name() {
        let mut span = MockSpan::new("my_op_name");
        assert_eq!(&String::from("my_op_name"), span.operation_name());
        span.set_operation_name("a_different_op_name");
        assert_eq!(&String::from("a_different_op_name"), span.operation_name());
    }
}
