use p017_letter_combinations_of_a_phone_number::Solution;

fn main() {
    let res = Solution::letter_combinations("23".to_string());
    assert_eq!(res, vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);

    let res = Solution::letter_combinations("".to_string());
    assert_eq!(res, Vec::<String>::new());
}
