use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut set = HashSet::new();
        let mut left = 0;
        let mut right = 0;
        let mut max = 1;
        while right < s.len() {
            let current_char = s.chars().nth(right).unwrap();

            if set.contains(&current_char) {
                set.remove(&s.chars().nth(left).unwrap());
                left += 1;
            } else {
                // If current character is not in the set, add it to the set
                set.insert(current_char);
                // Update max length if necessary
                max = max.max(right - left + 1);
                right += 1;
            }
        }

        max as i32
    }
}
