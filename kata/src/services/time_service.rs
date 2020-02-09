pub fn calculate_seconds(time: &str) -> u64 {
    let time_tokens: Vec<&str> = time.split(":").collect();
    let mut return_value: u64 = 0;
    if time_tokens.len() == 2 {
        if let Ok(hours) = time_tokens[0].parse::<u64>() {
            return_value += hours * 3600 as u64;
        }
        if let Ok(mins) = time_tokens[1].parse::<u64>() {
            return_value += mins * 60 as u64;
        }
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_seconds() {
        assert_eq!(calculate_seconds("24:12"), 87120);
        assert_eq!(calculate_seconds("01:54"), 6840);
        assert_eq!(calculate_seconds("12:10"), 43800);
        assert_eq!(calculate_seconds("11:11"), 40260);
        assert_eq!(calculate_seconds("00:01"), 60);
    }

    #[test]
    fn test_calculate_seconds_invalid_returns_zero() {
        assert_eq!(calculate_seconds("A:B"), 0);
    }
}