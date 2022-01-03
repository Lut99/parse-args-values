/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   28 Dec 2021, 18:09:54
 * Last edited:
 *   03 Jan 2022, 10:13:08
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
        let mut res: std::option::Option<u8>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_u8("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_u8("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_u8(&format!("{}", u8::MAX), &mut errors);
        assert_eq!(res, Some(u8::MAX));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_u8("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_u8(&format!("{}", u8::MAX as u16 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_u16() {
        // Prepare to parse
        let mut res: std::option::Option<u16>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_u16("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_u16("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_u16(&format!("{}", u16::MAX), &mut errors);
        assert_eq!(res, Some(u16::MAX));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_u16("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_u16(&format!("{}", u16::MAX as u32 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_u32() {
        // Prepare to parse
        let mut res: std::option::Option<u32>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_u32("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_u32("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_u32(&format!("{}", u32::MAX), &mut errors);
        assert_eq!(res, Some(u32::MAX));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_u32("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_u32(&format!("{}", u32::MAX as u64 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_u64() {
        // Prepare to parse
        let mut res: std::option::Option<u64>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_u64("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_u64("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_u64(&format!("{}", u64::MAX), &mut errors);
        assert_eq!(res, Some(u64::MAX));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_u64("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_u64(&format!("{}", u64::MAX as u128 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_u128() {
        // Prepare to parse
        let mut res: std::option::Option<u128>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_u128("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_u128("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_u128(&format!("{}", u128::MAX), &mut errors);
        assert_eq!(res, Some(u128::MAX));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_u128("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_u128("340282366920938463463374607431768211456", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }



    #[test]
    fn parse_i8() {
        // Prepare to parse
        let mut res: std::option::Option<i8>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_i8("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_i8("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_i8(&format!("{}", i8::MAX), &mut errors);
        assert_eq!(res, Some(i8::MAX));
        // Parse a negative value
        res = str_to_i8("-122", &mut errors);
        assert_eq!(res, Some(-122));
        // Parse the min negative value
        res = str_to_i8(&format!("{}", i8::MIN), &mut errors);
        assert_eq!(res, Some(i8::MIN));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_i8("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_i8(&format!("{}", i8::MAX as i16 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (negative)
        errlen = errors.len();
        res = str_to_i8(&format!("{}", i8::MIN as i16 - 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_i16() {
        // Prepare to parse
        let mut res: std::option::Option<i16>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_i16("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_i16("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_i16(&format!("{}", i16::MAX), &mut errors);
        assert_eq!(res, Some(i16::MAX));
        // Parse a negative value
        res = str_to_i16("-122", &mut errors);
        assert_eq!(res, Some(-122));
        // Parse the min negative value
        res = str_to_i16(&format!("{}", i16::MIN), &mut errors);
        assert_eq!(res, Some(i16::MIN));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_i16("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_i16(&format!("{}", i16::MAX as i32 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (negative)
        errlen = errors.len();
        res = str_to_i16(&format!("{}", i16::MIN as i32 - 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_i32() {
        // Prepare to parse
        let mut res: std::option::Option<i32>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_i32("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_i32("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_i32(&format!("{}", i32::MAX), &mut errors);
        assert_eq!(res, Some(i32::MAX));
        // Parse a negative value
        res = str_to_i32("-122", &mut errors);
        assert_eq!(res, Some(-122));
        // Parse the min negative value
        res = str_to_i32(&format!("{}", i32::MIN), &mut errors);
        assert_eq!(res, Some(i32::MIN));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_i32("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_i32(&format!("{}", i32::MAX as i64 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (negative)
        errlen = errors.len();
        res = str_to_i32(&format!("{}", i32::MIN as i64 - 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_i64() {
        // Prepare to parse
        let mut res: std::option::Option<i64>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_i64("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_i64("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_i64(&format!("{}", i64::MAX), &mut errors);
        assert_eq!(res, Some(i64::MAX));
        // Parse a negative value
        res = str_to_i64("-122", &mut errors);
        assert_eq!(res, Some(-122));
        // Parse the min negative value
        res = str_to_i64(&format!("{}", i64::MIN), &mut errors);
        assert_eq!(res, Some(i64::MIN));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_i64("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_i64(&format!("{}", i64::MAX as i128 + 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (negative)
        errlen = errors.len();
        res = str_to_i64(&format!("{}", i64::MIN as i128 - 1), &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
    }

    #[test]
    fn parse_i128() {
        // Prepare to parse
        let mut res: std::option::Option<i128>;
        let mut errlen: usize;
        let mut errors: Vec<String> = Vec::new();

        // Parse zero
        res = str_to_i128("0", &mut errors);
        assert_eq!(res, Some(0));
        // Parse a positive value
        res = str_to_i128("122", &mut errors);
        assert_eq!(res, Some(122));
        // Parse the max positive value
        res = str_to_i128(&format!("{}", i128::MAX), &mut errors);
        assert_eq!(res, Some(i128::MAX));
        // Parse a negative value
        res = str_to_i128("-122", &mut errors);
        assert_eq!(res, Some(-122));
        // Parse the min negative value
        res = str_to_i128(&format!("{}", i128::MIN), &mut errors);
        assert_eq!(res, Some(i128::MIN));

        // Parse an illegal character
        errlen = errors.len();
        res = str_to_i128("12b2", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (positive)
        errlen = errors.len();
        res = str_to_i128("170141183460469231731687303715884105728", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
        // Parse an overflow (negative)
        errlen = errors.len();
        res = str_to_i128("-170141183460469231731687303715884105729", &mut errors);
        assert_eq!(res, None);
        assert_eq!(errors.len(), errlen + 1);
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

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for u8: {} > {}.", raw, u8::MAX));
            return None;
        }
        result2 = result2.unwrap().checked_add(value);
        if result2 == None {
            errors.push(format!("Overflow for u8: {} > {}.", raw, u8::MAX));
            return None;
        }
        result = result2.unwrap();
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
        
        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for u16: {} > {}.", raw, u16::MAX));
            return None;
        }
        result2 = result2.unwrap().checked_add(value);
        if result2 == None {
            errors.push(format!("Overflow for u16: {} > {}.", raw, u16::MAX));
            return None;
        }
        result = result2.unwrap();
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
        
        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for u32: {} > {}.", raw, u32::MAX));
            return None;
        }
        result2 = result2.unwrap().checked_add(value);
        if result2 == None {
            errors.push(format!("Overflow for u32: {} > {}.", raw, u32::MAX));
            return None;
        }
        result = result2.unwrap();
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
        
        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for u64: {} > {}.", raw, u64::MAX));
            return None;
        }
        result2 = result2.unwrap().checked_add(value);
        if result2 == None {
            errors.push(format!("Overflow for u64: {} > {}.", raw, u64::MAX));
            return None;
        }
        result = result2.unwrap();
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
        
        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for u128: {} > {}.", raw, u128::MAX));
            return None;
        }
        result2 = result2.unwrap().checked_add(value);
        if result2 == None {
            errors.push(format!("Overflow for u128: {} > {}.", raw, u128::MAX));
            return None;
        }
        result = result2.unwrap();
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
        println!("\"{}\"[{}] = {}", raw, i, c);

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i8.", c));
            return None;
        }
        let value: i8 = char::to_digit(c, 10).unwrap() as i8;

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for i8: {} {} {}.", raw, if !negative { '>' } else { '<' }, if !negative { i8::MAX } else { i8::MIN }));
            return None;
        }
        if !negative {
            result2 = result2.unwrap().checked_add(value);
            if result2 == None {
                errors.push(format!("Overflow for i8: {} > {}.", raw, i8::MAX));
                return None;
            }
        } else {
            result2 = result2.unwrap().checked_sub(value);
            if result2 == None {
                errors.push(format!("Overflow for i8: {} < {}.", raw, i8::MIN));
                return None;
            }
        }
        result = result2.unwrap();

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to a signed integer of 16 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_i16(raw: &str, errors: &mut Vec<String>) -> Option<i16> {
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
    let mut result: i16 = 0;
    while i < chars.len() {
        // Get the char
        let c = chars[i];
        println!("\"{}\"[{}] = {}", raw, i, c);

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i16.", c));
            return None;
        }
        let value: i16 = char::to_digit(c, 10).unwrap() as i16;

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for i16: {} {} {}.", raw, if !negative { '>' } else { '<' }, if !negative { i16::MAX } else { i16::MIN }));
            return None;
        }
        if !negative {
            result2 = result2.unwrap().checked_add(value);
            if result2 == None {
                errors.push(format!("Overflow for i16: {} > {}.", raw, i16::MAX));
                return None;
            }
        } else {
            result2 = result2.unwrap().checked_sub(value);
            if result2 == None {
                errors.push(format!("Overflow for i16: {} < {}.", raw, i16::MIN));
                return None;
            }
        }
        result = result2.unwrap();

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to a signed integer of 32 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_i32(raw: &str, errors: &mut Vec<String>) -> Option<i32> {
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
    let mut result: i32 = 0;
    while i < chars.len() {
        // Get the char
        let c = chars[i];
        println!("\"{}\"[{}] = {}", raw, i, c);

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i32.", c));
            return None;
        }
        let value: i32 = char::to_digit(c, 10).unwrap() as i32;

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for i32: {} {} {}.", raw, if !negative { '>' } else { '<' }, if !negative { i32::MAX } else { i32::MIN }));
            return None;
        }
        if !negative {
            result2 = result2.unwrap().checked_add(value);
            if result2 == None {
                errors.push(format!("Overflow for i32: {} > {}.", raw, i32::MAX));
                return None;
            }
        } else {
            result2 = result2.unwrap().checked_sub(value);
            if result2 == None {
                errors.push(format!("Overflow for i32: {} < {}.", raw, i32::MIN));
                return None;
            }
        }
        result = result2.unwrap();

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to a signed integer of 64 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_i64(raw: &str, errors: &mut Vec<String>) -> Option<i64> {
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
    let mut result: i64 = 0;
    while i < chars.len() {
        // Get the char
        let c = chars[i];
        println!("\"{}\"[{}] = {}", raw, i, c);

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i64.", c));
            return None;
        }
        let value: i64 = char::to_digit(c, 10).unwrap() as i64;

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for i64: {} {} {}.", raw, if !negative { '>' } else { '<' }, if !negative { i64::MAX } else { i64::MIN }));
            return None;
        }
        if !negative {
            result2 = result2.unwrap().checked_add(value);
            if result2 == None {
                errors.push(format!("Overflow for i64: {} > {}.", raw, i64::MAX));
                return None;
            }
        } else {
            result2 = result2.unwrap().checked_sub(value);
            if result2 == None {
                errors.push(format!("Overflow for i64: {} < {}.", raw, i64::MIN));
                return None;
            }
        }
        result = result2.unwrap();

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}

/// Tries to convert a string to a signed integer of 128 bits.
/// 
/// **Arguments**
///  * `raw`: The raw string to parse.
///  * `errors`: A list of errors to which any errors that occur will be written. Only happens if 'None' is returned.
/// 
/// **Returns**  
/// The parsed value as the given unsigned integer type, or None if an error occurred.
pub fn str_to_i128(raw: &str, errors: &mut Vec<String>) -> Option<i128> {
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
    let mut result: i128 = 0;
    while i < chars.len() {
        // Get the char
        let c = chars[i];
        println!("\"{}\"[{}] = {}", raw, i, c);

        // Try to cast to an integer
        if c < '0' || c > '9' { 
            errors.push(format!("Illegal character '{}' for i128.", c));
            return None;
        }
        let value: i128 = char::to_digit(c, 10).unwrap() as i128;

        // Add it with overflow checks
        let mut result2 = result.checked_mul(10);
        if result2 == None {
            errors.push(format!("Overflow for i128: {} {} {}.", raw, if !negative { '>' } else { '<' }, if !negative { i128::MAX } else { i128::MIN }));
            return None;
        }
        if !negative {
            result2 = result2.unwrap().checked_add(value);
            if result2 == None {
                errors.push(format!("Overflow for i128: {} > {}.", raw, i128::MAX));
                return None;
            }
        } else {
            result2 = result2.unwrap().checked_sub(value);
            if result2 == None {
                errors.push(format!("Overflow for i128: {} < {}.", raw, i128::MIN));
                return None;
            }
        }
        result = result2.unwrap();

        // Increment
        i += 1;
    }

    // Return the result
    return Some(result);
}




