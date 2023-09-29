use crate::*;
pub fn fill_leading_zeroes(num: i32) -> String {
    let missing_zeroes = 5 - num.checked_ilog10().unwrap_or(0) - 1;
    let lead = "0".repeat(missing_zeroes as usize);
    let mut output = num.to_string();
    output.insert_str(0, &lead);
    output
}