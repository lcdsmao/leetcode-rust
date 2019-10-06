use p014_longest_common_prefix::Solution;

fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());

    let strs = vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ];
    assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
}
