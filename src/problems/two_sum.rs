//! Given an array of integers nums and an integer target,
//! return indices of the two numbers such that they add up to target.
//! 
//! Each input would have exactly one solution, 
//! and you may not use the same element twice.
//!
//! Constraints:
//! 2 <= nums.length <= 10^4
//! -10^9 <= nums[i] <= 10^9
//! -10^9 <= target <= 10^9
//! **Only one valid answer exists**.

#[cfg(feature = "leetcode")]
use crate::*;

// We will improve our solution along the way,
// leaving previous solutions unused.
#[allow(dead_code)]
impl Solution {
    /// The obvious solution is to brute force.
    /// This iterates over all possible combinations,
    /// checking if the sum equals the target.
    fn naive_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answers = vec![];
        'outer: for (x, &a) in nums.iter().enumerate() {
            for (y, &b) in nums.iter().enumerate() {
                // "you may not use the same element twice"
                if x == y {
                    continue 'outer;
                }

                if a + b == target {
                    answers.push(x as i32);
                    answers.push(y as i32);
                    return answers;
                }
            }
        }

        // "Only one valid answer exists"
        unreachable!();
    }

    /// Then, the second train of thought is to create a Map of value to index.
    ///
    /// And then search for target - num in the map.
    fn better_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let val2idx: HashMap<i32, i32> = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, val)| (val, idx as i32))
            .collect();
        
        for (i, &val) in nums.iter().enumerate() {
            let i = i as i32;
            if let Some(&i2) = val2idx.get(&(target - val)) {
                // "you may not use the same element twice"
                if i == i2 { 
                    continue; 
                }

                return vec![i, i2];
            }
        }

        unreachable!();
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::better_two_sum(nums, target)
    }
}