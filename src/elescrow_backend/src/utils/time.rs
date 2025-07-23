use ic_cdk::api::time;
use crate::utils::constants::*;

pub fn now() -> u64 {
    time()
}

pub fn nanos_to_seconds(nanos: u64) -> u64 {
    nanos / NANOS_PER_SECOND
}

pub fn seconds_to_nanos(seconds: u64) -> u64 {
    seconds * NANOS_PER_SECOND
}

pub fn is_past(timestamp: u64) -> bool {
    timestamp < now()
}

pub fn is_future(timestamp: u64) -> bool {
    timestamp > now()
}

pub fn is_within_range(timestamp: u64, start: u64, end: u64) -> bool {
    timestamp >= start && timestamp <= end
}

pub fn age_in_seconds(timestamp: u64) -> u64 {
    let current = now();
    if current > timestamp {
        nanos_to_seconds(current - timestamp)
    } else {
        0
    }
}

pub fn age_in_days(timestamp: u64) -> u64 {
    let current = now();
    if current > timestamp {
        (current - timestamp) / NANOS_PER_DAY
    } else {
        0
    }
}

pub fn add_duration(timestamp: u64, duration_nanos: u64) -> u64 {
    timestamp.saturating_add(duration_nanos)
}

pub fn subtract_duration(timestamp: u64, duration_nanos: u64) -> u64 {
    timestamp.saturating_sub(duration_nanos)
}

pub fn start_of_day(timestamp: u64) -> u64 {
    (timestamp / NANOS_PER_DAY) * NANOS_PER_DAY
}

pub fn end_of_day(timestamp: u64) -> u64 {
    start_of_day(timestamp) + NANOS_PER_DAY - 1
}

pub fn start_of_week(timestamp: u64) -> u64 {
    let days_since_epoch = timestamp / NANOS_PER_DAY;
    let day_of_week = (days_since_epoch + 3) % 7;
    let monday = days_since_epoch - day_of_week;
    monday * NANOS_PER_DAY
}

pub fn format_timestamp(timestamp: u64) -> String {
    let total_seconds = nanos_to_seconds(timestamp);
    let days = total_seconds / 86400;
    let remaining = total_seconds % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;
    
    let years = 1970 + (days / 365);
    let day_of_year = days % 365;
    
    format!("{}-{:03}-{:02}:{:02}:{:02}", years, day_of_year, hours, minutes, seconds)
}

pub fn parse_duration(duration_str: &str) -> Result<u64, String> {
    if duration_str.is_empty() {
        return Err("Empty duration string".to_string());
    }
    
    let (value_str, unit) = duration_str.split_at(duration_str.len() - 1);
    let value: u64 = value_str.parse()
        .map_err(|_| "Invalid duration value".to_string())?;
    
    let nanos = match unit {
        "s" => value * NANOS_PER_SECOND,
        "m" => value * NANOS_PER_MINUTE,
        "h" => value * NANOS_PER_HOUR,
        "d" => value * NANOS_PER_DAY,
        "w" => value * NANOS_PER_WEEK,
        _ => return Err(format!("Unknown duration unit: {}", unit)),
    };
    
    Ok(nanos)
}

pub fn get_next_occurrence(last_run: u64, interval: u64) -> u64 {
    let current = now();
    let elapsed = current.saturating_sub(last_run);
    let periods_passed = elapsed / interval;
    last_run + ((periods_passed + 1) * interval)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_duration_parsing() {
        assert_eq!(parse_duration("5s").unwrap(), 5 * NANOS_PER_SECOND);
        assert_eq!(parse_duration("10m").unwrap(), 10 * NANOS_PER_MINUTE);
        assert_eq!(parse_duration("2h").unwrap(), 2 * NANOS_PER_HOUR);
        assert_eq!(parse_duration("1d").unwrap(), NANOS_PER_DAY);
        assert_eq!(parse_duration("1w").unwrap(), NANOS_PER_WEEK);
        assert!(parse_duration("invalid").is_err());
    }
    
    #[test]
    fn test_time_calculations() {
        let timestamp = 1_000_000_000_000; 
        assert_eq!(nanos_to_seconds(timestamp), 1000);
        assert_eq!(seconds_to_nanos(1000), timestamp);
        
        let day_start = start_of_day(NANOS_PER_DAY * 5 + NANOS_PER_HOUR * 10);
        assert_eq!(day_start, NANOS_PER_DAY * 5);
    }
}