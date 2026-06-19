// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
// Example 1:
// Input: nums = [1,2,3,1]
// Output: true
// Explanation:
// The element 1 occurs at the indices 0 and 3.

use std::collections::HashSet;

mod Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for num in nums {
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = Solution::contains_duplicate(nums);
    println!("{}", result);
}