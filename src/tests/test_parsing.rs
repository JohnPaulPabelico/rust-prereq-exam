// Function: parse_string_to_u32
// Description: Parses a string into an unsigned integer 32 and compares it with the provided unsigned integer 32.
// Parameters:
// - a: The string to parse.
// - b: The expected unsigned integer 32.
// Returns: true if the parsed value matches the expected value, false otherwise.
pub fn parse_string_to_u32(a: &str, b: &u32) -> bool {
    if let Ok(parsed_val) = a.parse::<u32>() {
        return parsed_val == *b;
    }
    false
}

// Function: parse_u32_to_u128
// Description: Converts an unsigned integer 32 to an unsigned integer 128 and compares it with the provided unsigned integer 128.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected unsigned integer 128.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_u32_to_u128(a: u32, b: u128) -> bool {
    let converted_value: u128 = a as u128;
    return converted_value == b;
}
// Function: parse_u32_to_i32
// Description: Parses a string into a signed integer 32 and compares it with the provided signed integer 32.
// Parameters:
// - a: The unsigned integer 32 to parse.
// - b: The expected signed integer 32.
// Returns: true if the parsed value matches the expected value, false otherwise.
pub fn parse_u32_to_i32(a: u32, b: i32) -> bool {
    let converted_value: i32 = a as i32;
    return converted_value == b;
}

// Function: parse_u32_to_f32
// Description: Converts an unsigned integer 32 to a float 32 and compares it with the provided float 32.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected float 32.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_u32_to_f32(a: u32, b: f32) -> bool {
    let converted_value: f32 = a as f32;
    return converted_value == b;
}

// Function: parse_u32_to_string
// Description: Converts an unsigned integer 32 to a string and compares it with the provided string.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected string.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_u32_to_string(a: &u32, b: &str) -> bool {
    let a_string: String = a.to_string();
    return a_string == b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_u32() {
        assert!(parse_string_to_u32("2", &2));
        assert!(parse_string_to_u32("2024", &2024));
    }

    #[test]
    fn test_uparse_u32_to_u128() {
        let (a, c): (u32, u32) = (12, 60);
        let (b, d): (u128, u128) = (12, 60);
        assert!(parse_u32_to_u128(a, b));
        assert!(parse_u32_to_u128(c, d));
    }

    #[test]
    fn test_parse_u32_to_i32() {
        let (a, c): (u32, u32) = (1, 55);
        let (b, d): (i32, i32) = (1, 55);
        assert!(parse_u32_to_i32(a, b));
        assert!(parse_u32_to_i32(c, d));
    }

    #[test]
    fn test_parse_u32_to_f32() {
        assert!(parse_u32_to_f32(32, 32.0));
        assert!(parse_u32_to_f32(14, 14.0));
    }

    #[test]
    fn test_parse_u32_to_string() {
        assert!(parse_u32_to_string(&5, "5"));
        assert!(parse_u32_to_string(&10, "10"));
    }
}