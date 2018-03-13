/// `SpanContext` represents Span state that must propagate to
/// descendant Spans and across process boundaries.
///
/// `SpanContext` is logically divided into two pieces: (1) the user-level "Baggage" that
/// propagates across Span boundaries and (2) any Tracer-implementation-specific fields
/// that are needed to identify or otherwise contextualize the associated Span instance
/// (e.g., a `(trace_id, span_id, sampled)` tuple).
pub trait SpanContext<'a> {
    /// Associated type defining how to iterate over baggage items.
    type Iter: Iterator<Item = (&'a String, &'a String)>;

    /// Iterate over baggage items.
    ///
    /// Baggage items are key/value pairs that are propagated from
    /// the associated `Span` throught the trace.
    fn baggage_items(&'a self) -> Self::Iter;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::hash_map::Iter as HashMapIter;
    use super::SpanContext;

    struct TestContext {
        items: HashMap<String, String>,
    }

    impl<'a> SpanContext<'a> for TestContext {
        type Iter = HashMapIter<'a, String, String>;

        fn baggage_items(&'a self) -> Self::Iter {
            self.items.iter()
        }
    }

    #[test]
    fn get_items() {
        let mut items = HashMap::new();
        items.insert("key".into(), "value".into());
        let context = TestContext { items: items };
        let items: Vec<(&String, &String)> = context.baggage_items().collect();
        assert_eq!(items, [(&"key".into(), &"value".into())])
    }
}
