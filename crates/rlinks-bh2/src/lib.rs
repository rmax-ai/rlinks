use std::collections::HashMap;

/// A tiny prototype aggregator: counts hits per `code` in-memory.
/// This is intentionally simple and designed for unit tests and as a starting point.
pub fn aggregate_hits(hits: &[(String, u64)]) -> HashMap<String, u64> {
    let mut agg: HashMap<String, u64> = HashMap::new();
    for (code, _ts) in hits {
        *agg.entry(code.clone()).or_insert(0) += 1;
    }
    agg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_counts_correctly() {
        let hits = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("a".to_string(), 3),
        ];
        let agg = aggregate_hits(&hits);
        assert_eq!(agg.get("a"), Some(&2u64));
        assert_eq!(agg.get("b"), Some(&1u64));
    }

    #[test]
    fn empty_is_empty() {
        let hits: Vec<(String, u64)> = vec![];
        let agg = aggregate_hits(&hits);
        assert!(agg.is_empty());
    }
}
