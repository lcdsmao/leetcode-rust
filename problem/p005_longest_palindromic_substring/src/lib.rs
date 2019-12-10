pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut i = 0;
        let mut start: i32 = 0;
        let mut end: i32 = 0;
        while i < s.len() - (end - start) as usize / 2 {
            let (l, r) = Self::find_palindrome(&s, i, i);
            if r - l > end - start {
                start = l;
                end = r;
            }
            let (l, r) = Self::find_palindrome(&s, i, i + 1);
            if r - l > end - start {
                start = l;
                end = r;
            }
            i = i + 1;
        }
        s[start as usize..end as usize].to_string()
    }

    fn find_palindrome(s: &String, i: usize, j: usize) -> (i32, i32) {
        let mut l = i as i32;
        let mut r = j as i32;
        let chs = s.as_bytes();
        while l >= 0 && r < s.len() as i32 && chs[l as usize] == chs[r as usize] {
            l = l - 1;
            r = r + 1;
        }
        (l + 1, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = Solution::longest_palindrome(String::from("babad"));
        assert_eq!(s, String::from("bab"));

        let s = Solution::longest_palindrome(String::from("cbbd"));
        assert_eq!(s, String::from("bb"));
    }
}
