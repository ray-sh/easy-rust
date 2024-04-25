// while (right < nums.size()) {
//     // 增大窗口
//     window.add(nums[right]);
//     right++;

//     while (window needs shrink) {
//         // 缩小窗口
//         window.remove(nums[left]);
//         left++;
//     }
// }
#[cfg(test)]
mod tests {
    // 76. 最小窗口子串
    fn mycontains(s: &str, t: &str) -> bool {
        t.chars().all(|c| s.contains(c))
    }
    fn min_window<'a>(source: &'a str, target: &str) -> &'a str {
        let mut left = 0;
        let mut min_str = "";
        for right in 0..source.len() {
            let left_char = source.chars().nth(left).unwrap();
            println!(
                "left chart {}, left {}, right {}, window {}",
                left_char,
                left,
                right,
                &source[left..=right]
            );
            if mycontains(&source[left..=right], target) {
                if min_str.is_empty() || min_str.len() > (right - left + 1) {
                    min_str = &source[left..=right];
                    left = right + 1;
                }
            }
            if target.find(left_char) == None {
                left = left + 1;
            }
        }
        min_str
    }
    //3. 最长子序列
    fn length_of_longest_substring(s: &str) -> usize {
        let mut max_len = 0;
        let mut left = 0;
        let mut right = 0;
        while right < s.len() {
            let current_char = s.chars().nth(right).unwrap();
            match s[left..right].find(current_char) {
                Some(index) => {
                    left = left + index + 1;
                }
                None => {
                    max_len = max_len.max(right - left + 1);
                }
            }
            right = right + 1;
        }
        max_len
    }
    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
    }

    #[test]
    fn basic() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window("ADOBECODEBANC", "ABCD"), "ADOBEC");
        assert_eq!(min_window("ADOBECODEBANC", "ABF@CD"), "");
    }
}
