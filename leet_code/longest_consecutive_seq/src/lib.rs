use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // if you wonder nums.cloned().collect() would also work. Since we don't expect any slices ex. &[i32] = &[1,2,3]
        // we can't just clone the slices so we need the .iter(). Chatgpt for details
        let num_set: HashSet<i32> = nums.iter().cloned().collect();

        let mut max_length: i32 = 0;
        for &num in num_set.iter() {
            let mut length = 1;
            if !num_set.contains(&(num - 1)) {
                if num_set.contains(&(num + 1)) {
                    length += 1;
                    while num_set.contains(&(num + length)) {
                        length += 1;
                    }
                }
            }
            max_length = max_length.max(length);
            length = 0;
        }
        max_length
    }
}
