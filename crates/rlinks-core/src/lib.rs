mod redirect;

pub use redirect::{validate_redirect, Redirect};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serde() {
        let r = Redirect {
            code: "abc123".to_string(),
            target: "https://example.com".to_string(),
        };
        let s = serde_json::to_string(&r).unwrap();
        let parsed: Redirect = serde_json::from_str(&s).unwrap();
        assert_eq!(r, parsed);
    }
}
