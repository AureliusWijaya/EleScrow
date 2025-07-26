fn get_heap_memory_size() -> u64 {
    1024 * 1024 * 100
}

pub fn format_principal_short(principal: &candid::Principal) -> String {
    let full = principal.to_text();
    if full.len() > 16 {
        format!("{}...{}", &full[..6], &full[full.len()-6..])
    } else {
        full
    }
}

pub fn format_duration(nanos: u64) -> String {
    let seconds = nanos / 1_000_000_000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    
    if days > 0 {
        format!("{} days", days)
    } else if hours > 0 {
        format!("{} hours", hours)
    } else if minutes > 0 {
        format!("{} minutes", minutes)
    } else {
        format!("{} seconds", seconds)
    }
}

pub fn generate_unique_id(counter: u64) -> String {
    let timestamp = ic_cdk::api::time();
    format!("{}-{}", timestamp, counter)
}

pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}

pub fn is_anonymous(principal: &candid::Principal) -> bool {
    principal == &candid::Principal::anonymous()
}

pub fn calculate_percentage(value: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (value as f64 / total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(1_000_000_000), "1 seconds");
        assert_eq!(format_duration(60_000_000_000), "1 minutes");
        assert_eq!(format_duration(3600_000_000_000), "1 hours");
        assert_eq!(format_duration(86400_000_000_000), "1 days");
    }
    
    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric("test123"));
        assert!(is_alphanumeric("test_123"));
        assert!(is_alphanumeric("test-123"));
        assert!(!is_alphanumeric("test@123"));
        assert!(!is_alphanumeric("test 123"));
    }
    
    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }
}