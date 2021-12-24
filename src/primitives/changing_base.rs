/// Given an integer (represented as a string) under base b1, convert it to base b2.
///
/// ##  Example 1:
/// - Input: ("12", 10, 2)
/// - Output: "1100"
/// - Explanation: We are converting "12" which is under base 10 (decimal) to base 2 (binary)
///
/// ## Example 2:
/// - Input: ("123", 4, 10)
/// - Output: "27"
/// - Explanation: We are converting "123" which is under base 4 (quaternary) to base 10
///
/// ## Example 3:
/// - Input: ("123", 4, 16)
/// - Output: "1B"
/// - Explanation: Convert "123" from base 4 to 16
///
/// ## Example 4:
/// - Input: ("123", 10, 10)
/// - Output: "123"
/// - Explanation: Convert "123" from base 10 to 10
///
/// ## Constraints:
/// - 2 <= b1 <= 36
/// - 2 <= b2 <= 36
/// - For base-2 up to base-10 use the digits 0-9 to represent digits.
/// - Use the English alphabet characters A-Z in base-11 and higher.
fn changing_base(s: &str, b1: u32, b2: u32) -> String {
    let is_negative = s.starts_with('-');
    // If the number has a minus sign "-", then we start decomposing
    // 'numAsString' from index 1 instead of index 0.
    let start = if is_negative { 1 } else { 0 };
    // The power applied to the base that the most significant digit will
    // be multiplied by. Ex: "324" -> maxPower will be 2 since
    // "324" = (3 * 10^2) + (2 * 10^1) + (4 * 10^0) ^ maxPower
    let max_power = s.len() - 1;
    // The number we are slowly going to build.
    let mut number_under_base10 = 0;

    // Loop over every digit & add it to the base 10 integer representation we are building
    // We will later take this base 10 integer and convert it into b2.
    for i in start..s.len() {
        // Get the integer value that this place "contributes", or in the other words,
        // the value that will be multiplied by (base)^(digit's position).
        let is_place_digit = s.chars().nth(i).unwrap().is_digit(b1);
        let value_contributed_by_place = if is_place_digit {
            s.chars().nth(i).unwrap() as u32 - '0' as u32
        } else {
            s.chars().nth(i).unwrap() as u32 - 'A' as u32 + 10
        };

        // So if we have "895", it means:
        // (8 * 10^2) + (9 * 10^1) + (5 * 10^0)
        // (800) + (90) + (50)
        //
        // If we have "1AB" (under base 16, hex), it means:
        //
        // (1 * 16^2) + (10 * 16^1) + (11 * 16^0)
        // (256) + (160) + (11)
        let position_power_weight = max_power - i;
        let x = value_contributed_by_place * u32::pow(b1, position_power_weight as u32);

        number_under_base10 += x;
    }

    if number_under_base10 == 0 {
        "0".to_string()
    } else {
        let sign = if is_negative { "-" } else { "" };
        sign.to_string() + base10_to_new_base(number_under_base10, b2).as_str()
    }
}

fn base10_to_new_base(number_under_base10: u32, new_base: u32) -> String {
    if number_under_base10 == 0 {
        return "".to_string();
    }

    // lsd => least significant digit
    let lsd_as_char: char;
    let lsd_under_final_base = number_under_base10 % new_base;
    let needs_hex_conversion = lsd_under_final_base > 9;
    if needs_hex_conversion {
        // Convert digit into a letter (theoretically 'A'-'Z') because we can't express
        // values 10 and above as single digit values.
        //
        // If we go past base 36 this will break.
        lsd_as_char = ('A' as u8 + lsd_under_final_base as u8 - 10) as char;
    } else {
        lsd_as_char = ('0' as u8 + lsd_under_final_base as u8) as char;
    }

    let base_10_number_without_lsd = number_under_base10 / new_base;
    let everything_else_to_our_left = base10_to_new_base(base_10_number_without_lsd, new_base);

    everything_else_to_our_left + lsd_as_char.to_string().as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base10_to_new_base() {
        assert_eq!(base10_to_new_base(15, 16), "F");
    }

    #[test]
    fn test_changing_base() {
        assert_eq!(changing_base("12", 10, 2), "1100");
        // assert_eq!(changing_base("123", 4, 10), "27");
        // assert_eq!(changing_base("123", 4, 16), "1B");
        // assert_eq!(changing_base("123", 10, 10), "123");
    }
}