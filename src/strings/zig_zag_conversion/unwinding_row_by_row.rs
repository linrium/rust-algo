/// # Problem:
/// Given a string s and an integer value rows return the "zigzag" encoding of s read off row-by-row.
///
/// # Example 1:
/// Input:
/// - s = "YELLOWPINK"
/// - rows = 4
/// Output: "YPEWILONLK"
/// Explanation: There are 4 rows in the zigzag formatted string.
/// Y     P    (row 1: "YP")
/// E   W I    (row 2: "EWI")
/// L O   N    (row 3: "LON")
/// L     K    (row 4: "LK")
///
/// # Example 2:
/// Input:
/// - s = "REDBLUEBLACK"
/// - rows = 2
/// Output: "RDLELCEBUBAK"
/// Explanation: There are 2 rows in the zigzag formatted string.
/// R D L E L C    (row 1: "RDLELC")
/// E B U B A K    (row 2: "EBUBAK")
///
/// # Example 3:
/// Input:
/// - s = "REDBLUEBLACK"
/// - rows = 1
/// Output: "REDBLUEBLACK"
/// Explanation: There is only 1 row in the zigzag formatted string.
/// R E D B L U E B L A C K    (row 1: "REDBLUEBLACK")
///
/// # Constraints:
/// - rows > 0
///
/// # Approach:
/// - We loop over every character in the string and track what row each character belongs to in the zigzag format. We combine all of these row results at the end.
///
/// # Complexity:
/// - Time: O(n)
/// - Space: O(n)
///
/// # Companies:
/// - PayPal
/// - Amazon
///
/// # Difficulty: Medium
fn zig_zag_conversion(s: String, num_rows: i32) -> String {
    let mut output = Vec::with_capacity(num_rows as usize);
    for _ in 0..num_rows {
        output.push("".to_string());
    }

    let mut row: i32 = 0;
    let mut down = false;
    for c in s.chars() {
        output[row as usize] = output[row as usize].to_string() + &c.to_string();

        if row == 0 || row == num_rows - 1 {
            down = !down;
        }

        if num_rows > 1 {
            if down {
                row += 1;
            } else {
                row -= 1;
            }
        }
    }

    output.join("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zig_zag_conversion_1() {
        let s = "YELLOWPINK".to_string();
        let num_rows = 4;
        let result = super::zig_zag_conversion(s, num_rows);
        assert_eq!(result, "YPEWILONLK".to_string());
    }

    #[test]
    fn test_zig_zag_conversion_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = super::zig_zag_conversion(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn test_zig_zag_conversion_3() {
        let s = "REDBLUEBLACK".to_string();
        let num_rows = 2;
        let result = super::zig_zag_conversion(s, num_rows);
        assert_eq!(result, "RDLELCEBUBAK".to_string());
    }

    #[test]
    fn test_zig_zag_conversion_4() {
        let s = "REDBLUEBLACK".to_string();
        let num_rows = 1;
        let result = super::zig_zag_conversion(s, num_rows);
        assert_eq!(result, "REDBLUEBLACK".to_string());
    }
}