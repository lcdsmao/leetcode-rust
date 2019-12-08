pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut l_max = height[l];
        let mut r_max = height[r];
        let mut area = 0;
        while l < r {
            if height[l] < height[r] {
                area += (l_max - height[l]).max(0);
                l_max = l_max.max(height[l]);
                l += 1;
            } else {
                area += (r_max - height[r]).max(0);
                r_max = r_max.max(height[r]);
                r -= 1;
            }
        }
        area
    }
}
