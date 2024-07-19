impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let mut last_count = 0;

        for c in s.chars() {
            if c == ' ' {
                if count != 0 {
                    last_count = count;
                }
                count = 0;
            } else {
                count += 1;
            }
        }

        if count != 0 {
            count
        } else {
            last_count
        }
    }
}
