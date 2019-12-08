use p034_find_first_and_last_position_of_element_in_sorted_array::Solution;

fn main() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );

    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
}
