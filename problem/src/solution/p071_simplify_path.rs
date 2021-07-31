pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let paths: Vec<&str> = path.split('/').collect();
        let mut stack: Vec<&str> = vec![];

        for p in paths.into_iter() {
            match p {
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                "." | "" => {}
                _ => stack.push(p),
            }
        }

        let mut res = stack.join("/");
        res.insert(0, '/');
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string(),
        );
        assert_eq!(
            Solution::simplify_path("/../".to_string()),
            "/".to_string(),
        );
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string(),
        );
    }
}
