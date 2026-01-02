use std::fmt::Write;

use crate::*;

/// Fills a string with the number of leading zeroes needed to make the number 5 digits long.
pub fn fill_leading_zeroes(text: &mut String, num: i32) {
    let missing_zeroes = 5 - num.checked_ilog10().unwrap_or(0) - 1;
    for _ in 0..missing_zeroes {
        text.push('0');
    }
    write!(text, "{num}").expect("string write cannot fail");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_leading_zeroes() {
        assert_eq!(fill_leading_zeroes(0), "00000");
        assert_eq!(fill_leading_zeroes(1), "00001");
        assert_eq!(fill_leading_zeroes(10), "00010");
        assert_eq!(fill_leading_zeroes(100), "00100");
        assert_eq!(fill_leading_zeroes(1000), "01000");
        assert_eq!(fill_leading_zeroes(10000), "10000");
    }
}
