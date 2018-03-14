//! This crate provides a mocking implementation of the tracing
//! API, especially useful for testing.
#![doc(html_root_url = "https://docs.rs/opentracing-mock/0.1.0")]

extern crate opentracing_api;

use opentracing_api::SpanContext;
use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;

pub struct MockSpanContext {
    baggage: HashMap<String, String>,
}

impl MockSpanContext {
    /// Create a new `MockSpanContext` with the given baggage.
    pub fn new(baggage: HashMap<String, String>) -> Self {
        MockSpanContext { baggage }
    }
}

impl<'a> SpanContext<'a> for MockSpanContext {
    type Iter = HashMapIter<'a, String, String>;

    fn baggage_items(&'a self) -> Self::Iter {
        self.baggage.iter()
    }
}

#[cfg(test)]
mod tests {

    use super::MockSpanContext;
    use opentracing_api::SpanContext;
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

}
