use candid::Principal;
use regex::Regex;
use lazy_static::lazy_static;
use crate::types::errors::ApiError;

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,30}$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    static ref URL_REGEX: Regex = Regex::new(
        r"^https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)$"
    ).unwrap();
    static ref PRINCIPAL_REGEX: Regex = Regex::new(
        r"^[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{3}$"
    ).unwrap();
}

pub fn validate_username(username: &str) -> Result<(), ApiError> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(ApiError::ValidationError {
            field: "username".to_string(),
            message: "Username must be 3-30 characters, containing only letters, numbers, underscore, or hyphen".to_string(),
        });
    }
    
    let reserved = ["admin", "root", "system", "api", "support", "help", "info"];
    if reserved.contains(&username.to_lowercase().as_str()) {
        return Err(ApiError::ValidationError {
            field: "username".to_string(),
            message: "This username is reserved".to_string(),
        });
    }
    
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), ApiError> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(ApiError::ValidationError {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        });
    }
    
    if email.len() > 254 {
        return Err(ApiError::ValidationError {
            field: "email".to_string(),
            message: "Email address too long".to_string(),
        });
    }
    
    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<(), ApiError> {
    if !PHONE_REGEX.is_match(phone) {
        return Err(ApiError::ValidationError {
            field: "phone".to_string(),
            message: "Invalid phone number format. Use E.164 format (e.g., +1234567890)".to_string(),
        });
    }
    
    Ok(())
}

pub fn validate_url(url: &str) -> Result<(), ApiError> {
    if !URL_REGEX.is_match(url) {
        return Err(ApiError::ValidationError {
            field: "url".to_string(),
            message: "Invalid URL format".to_string(),
        });
    }
    
    if url.len() > 2048 {
        return Err(ApiError::ValidationError {
            field: "url".to_string(),
            message: "URL too long".to_string(),
        });
    }
    
    Ok(())
}

pub fn validate_principal(principal: &Principal) -> Result<(), ApiError> {
    if principal == &Principal::anonymous() {
        return Err(ApiError::Unauthorized {
            reason: "Anonymous principal not allowed".to_string(),
        });
    }
    
    let principal_text = principal.to_text();
    if !PRINCIPAL_REGEX.is_match(&principal_text) {
        return Err(ApiError::ValidationError {
            field: "principal".to_string(),
            message: "Invalid principal format".to_string(),
        });
    }
    
    Ok(())
}

pub fn validate_amount(amount: u64, min: Option<u64>, max: Option<u64>) -> Result<(), ApiError> {
    if amount == 0 {
        return Err(ApiError::ValidationError {
            field: "amount".to_string(),
            message: "Amount must be greater than zero".to_string(),
        });
    }
    
    if let Some(min_amount) = min {
        if amount < min_amount {
            return Err(ApiError::ValidationError {
                field: "amount".to_string(),
                message: format!("Amount must be at least {}", min_amount),
            });
        }
    }
    
    if let Some(max_amount) = max {
        if amount > max_amount {
            return Err(ApiError::ValidationError {
                field: "amount".to_string(),
                message: format!("Amount cannot exceed {}", max_amount),
            });
        }
    }
    
    Ok(())
}

pub fn validate_text(text: &str, field: &str, min_length: usize, max_length: usize) -> Result<String, ApiError> {
    let trimmed = text.trim();
    
    if trimmed.len() < min_length {
        return Err(ApiError::ValidationError {
            field: field.to_string(),
            message: format!("{} must be at least {} characters", field, min_length),
        });
    }
    
    if trimmed.len() > max_length {
        return Err(ApiError::ValidationError {
            field: field.to_string(),
            message: format!("{} cannot exceed {} characters", field, max_length),
        });
    }
    
    let sanitized = trimmed.chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect::<String>();
    
    Ok(sanitized)
}

pub fn validate_timestamp(timestamp: u64, field: &str) -> Result<(), ApiError> {
    use ic_cdk::api::time;
    
    let now = time();
    
    let min_timestamp = 1_577_836_800_000_000_000; 
    if timestamp < min_timestamp {
        return Err(ApiError::ValidationError {
            field: field.to_string(),
            message: "Timestamp too far in the past".to_string(),
        });
    }
    
    let max_future = now + (10 * 365 * 24 * 60 * 60 * 1_000_000_000); 
    if timestamp > max_future {
        return Err(ApiError::ValidationError {
            field: field.to_string(),
            message: "Timestamp too far in the future".to_string(),
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_username_validation() {
        assert!(validate_username("john_doe").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("a").is_err());
        assert!(validate_username("user@name").is_err());
        assert!(validate_username("admin").is_err());
    }
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user+tag@domain.co.uk").is_ok());
        assert!(validate_email("invalid.email").is_err());
        assert!(validate_email("@example.com").is_err());
    }
}