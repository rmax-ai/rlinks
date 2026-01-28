use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Redirect {
    pub code: String,
    pub target: String,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("invalid code")]
    InvalidCode,
    #[error("reserved code")]
    ReservedCode,
    #[error("invalid target URL")]
    InvalidTarget,
    #[error("target must be https")]
    InsecureTarget,
}

pub fn validate_redirect(r: &Redirect) -> Result<(), ValidationError> {
    // code: 2-32 chars, letters, digits, hyphen
    let re = Regex::new(r"^[a-zA-Z0-9\-]{2,32}$").unwrap();
    if !re.is_match(&r.code) {
        return Err(ValidationError::InvalidCode);
    }

    // reserved words (documented in docs/SPEC.md Reserved Codes)
    let reserved = ["api", "admin", "www"];
    if reserved.contains(&r.code.as_str()) {
        return Err(ValidationError::ReservedCode);
    }

    // validate URL and require https
    let url = Url::parse(&r.target).map_err(|_| ValidationError::InvalidTarget)?;
    if url.scheme() != "https" {
        return Err(ValidationError::InsecureTarget);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_redirect_passes() {
        let r = Redirect {
            code: "ok-1".to_string(),
            target: "https://example.com/path".to_string(),
        };
        assert!(validate_redirect(&r).is_ok());
    }

    #[test]
    fn invalid_code_fails() {
        let r = Redirect {
            code: "!bad".to_string(),
            target: "https://example.com".to_string(),
        };
        assert!(matches!(
            validate_redirect(&r),
            Err(ValidationError::InvalidCode)
        ));
    }

    #[test]
    fn reserved_code_fails() {
        let r = Redirect {
            code: "api".to_string(),
            target: "https://example.com".to_string(),
        };
        assert!(matches!(
            validate_redirect(&r),
            Err(ValidationError::ReservedCode)
        ));
    }

    #[test]
    fn insecure_target_fails() {
        let r = Redirect {
            code: "ok".to_string(),
            target: "http://example.com".to_string(),
        };
        assert!(matches!(
            validate_redirect(&r),
            Err(ValidationError::InsecureTarget)
        ));
    }

    #[test]
    fn invalid_target_fails() {
        let r = Redirect {
            code: "ok".to_string(),
            target: "not-a-url".to_string(),
        };
        assert!(matches!(
            validate_redirect(&r),
            Err(ValidationError::InvalidTarget)
        ));
    }
}
