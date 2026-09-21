use std::char;

fn get_max_char(size: usize) -> char {
    char::from_u32('A' as u32 + (size - 1) as u32).unwrap()
}

pub fn parse_and_return_value(key_value: &str, size: usize) -> Option<usize> {
    let event_key_opt = key_value.chars().nth(0);
    let max_char = get_max_char(size);
    match event_key_opt {
        None => None,
        Some(value)
            if value >= 'A' && value <= max_char
                || value >= 'a' && value <= max_char.to_ascii_lowercase() =>
        {
            Some(value.to_ascii_lowercase() as usize - 'a' as usize)
        }
        Some(value) if value < 'A' || value > max_char => None,
        Some(_) => None,
    }
}

pub fn timer_text(seconds: u64) -> String {
    format!("Time: {} seconds", seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_reset_and_elapsed_timer_values() {
        assert_eq!(timer_text(0), "Time: 0 seconds");
        assert_eq!(timer_text(42), "Time: 42 seconds");
    }
}
