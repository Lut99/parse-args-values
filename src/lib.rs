/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   28 Dec 2021, 18:09:54
 * Last edited:
 *   02 Jan 2022, 18:45:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   A small library that adds value parsers for the Parse Args library.
**/


/***** UNIT TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u8() {
        // Prepare to parse
        let to_parse = "192";
        let mut errors: Vec<String> = Vec::new();

        // Parse it!
        let res = str_to_u8(to_parse, &mut errors);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 192);
    }

    #[test]
    fn parse_u16() {
        // Prepare to parse
        let to_parse = "7623";
        let mut errors: Vec<String> = Vec::new();

        // Parse it!
        let res = str_to_u16(to_parse, &mut errors);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 7623);
    }

    #[test]
    fn parse_u32() {
        // Prepare to parse
        let to_parse = "285876234";
        let mut errors: Vec<String> = Vec::new();

        // Parse it!
        let res = str_to_u32(to_parse, &mut errors);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 285876234);
    }

    #[test]
    fn parse_u64() {
        // Prepare to parse
        let to_parse = "2858762312736487124";
        let mut errors: Vec<String> = Vec::new();

        // Parse it!
        let res = str_to_u64(to_parse, &mut errors);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 2858762312736487124);
    }

    #[test]
    fn parse_u128() {
        // Prepare to parse
        let to_parse = "285876231273648289794827397124";
        let mut errors: Vec<String> = Vec::new();

        // Parse it!
        let res = str_to_u128(to_parse, &mut errors);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 285876231273648289794827397124);
    }
}





/***** LIBRARY FUNCTIONS *****/
/// Tries to convert a string to an unsigned integer of 8 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_u8(raw: &str, errors: &mut Vec<String>) -> Option<u8> {
    // Loop to convert
    let mut result: u8 = 0;
    for c in raw.chars() {
        // Try to cast to an integer
        if c < '0' || c > '9' {
            errors.push(format!("Illegal character '{}' for u8.", c));
            return None;
        }
        let value: u8 = char::to_digit(c, 10).unwrap() as u8;
        
        // Make sure it doesn't overflow
        if result > u8::MAX / 10 - value {
            errors.push(format!("Overflow for u8: {} > {}.", raw, u8::MAX));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to an unsigned integer of 16 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_u16(raw: &str, errors: &mut Vec<String>) -> Option<u16> {
    // Loop to convert
    let mut result: u16 = 0;
    for c in raw.chars() {
        // Try to cast to an integer
        if c < '0' || c > '9' {
            errors.push(format!("Illegal character '{}' for u16.", c));
            return None;
        }
        let value: u16 = char::to_digit(c, 10).unwrap() as u16;
        
        // Make sure it doesn't overflow
        if result > u16::MAX / 10 - value {
            errors.push(format!("Overflow for u16: {} > {}.", raw, u16::MAX));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to an unsigned integer of 32 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_u32(raw: &str, errors: &mut Vec<String>) -> Option<u32> {
    // Loop to convert
    let mut result: u32 = 0;
    for c in raw.chars() {
        // Try to cast to an integer
        if c < '0' || c > '9' {
            errors.push(format!("Illegal character '{}' for u32.", c));
            return None;
        }
        let value: u32 = char::to_digit(c, 10).unwrap() as u32;
        
        // Make sure it doesn't overflow
        if result > u32::MAX / 10 - value {
            errors.push(format!("Overflow for u32: {} > {}.", raw, u32::MAX));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to an unsigned integer of 64 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_u64(raw: &str, errors: &mut Vec<String>) -> Option<u64> {
    // Loop to convert
    let mut result: u64 = 0;
    for c in raw.chars() {
        // Try to cast to an integer
        if c < '0' || c > '9' {
            errors.push(format!("Illegal character '{}' for u64.", c));
            return None;
        }
        let value: u64 = char::to_digit(c, 10).unwrap() as u64;
        
        // Make sure it doesn't overflow
        if result > u64::MAX / 10 - value {
            errors.push(format!("Overflow for u64: {} > {}.", raw, u64::MAX));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to an unsigned integer of 128 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_u128(raw: &str, errors: &mut Vec<String>) -> Option<u128> {
    // Loop to convert
    let mut result: u128 = 0;
    for c in raw.chars() {
        // Try to cast to an integer
        if c < '0' || c > '9' {
            errors.push(format!("Illegal character '{}' for u128.", c));
            return None;
        }
        let value: u128 = char::to_digit(c, 10).unwrap() as u128;
        
        // Make sure it doesn't overflow
        if result > u128::MAX / 10 - value {
            errors.push(format!("Overflow for u128: {} > {}.", raw, u128::MAX));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;
    }

    // Return the result
    return Some(result);
}



/// Tries to convert a string to a signed integer of 8 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_i8(raw: &str, errors: &mut Vec<String>) -> Option<i8> {
    // First, parse negative or not
    let mut negative: bool = false;
    let chars = raw.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '-' { break; }
        negative = !negative;
        i += 1;
    }

    // Loop to convert
    let mut result: i8 = 0;
    while i < chars.len() {
        // Get the char
        let c = chars[i];

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i8.", c));
            return None;
        }
        let value: i8 = char::to_digit(c, 10).unwrap() as i8;
        
        // Make sure it doesn't overflow
        if !negative && result > i8::MAX / 10 - value {
            errors.push(format!("Overflow for i8: {} > {}.", raw, i8::MAX));
            return None;
        } else if negative && result < i8::MIN / 10 + value {
            errors.push(format!("Overflow for i8: {} < {}.", raw, i8::MIN));
            return None;
        }

        // Add it 
        result *= 10;
        result += value;

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}
