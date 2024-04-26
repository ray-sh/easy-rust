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
    fn min_window<'a>(source: &'a str, target: &str) -> &'a str {
        let mut left = 0;
        let mut min_str = "";
        for right in 0..source.len() {
            let left_char = source.chars().nth(left).unwrap();
            let window = &source[left..=right];
            if target.chars().all(|c| window.contains(c)) {
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
    #[test]
    fn test_76() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window("ADOBECODEBANC", "ABCD"), "ADOBEC");
        assert_eq!(min_window("ADOBECODEBANC", "ABF@CD"), "");
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

    // 567
    // 给你两个字符串 s1 和 s2 ，写一个函数来判断 s2 是否包含 s1 的排列。如果是，返回 true ；否则，返回 false 。

    // 换句话说，s1 的排列之一是 s2 的 子串 。

    // 示例 1：

    // 输入：s1 = "ab" s2 = "eidbaooo"
    // 输出：true
    // 解释：s2 包含 s1 的排列之一 ("ba").
    // 示例 2：

    // 输入：s1= "ab" s2 = "eidboaoo"
    // 输出：false
    // 提示：

    // 1 <= s1.length, s2.length <= 104
    // s1 和 s2 仅包含小写字母

    fn has_sub(s1: &str, s2: &str) -> bool {
        let mut left = 0;
        let w_size = s1.len();
        while left < s2.len() - w_size {
            if s1.chars().all(|c| s2[left..left + w_size].contains(c)) {
                return true;
            }
            left = left + 1;
        }
        false
    }
    #[test]
    fn test_567() {
        assert_eq!(has_sub("ab", "eidbaooo"), true);
        assert_eq!(has_sub("ab", "eidboaoo"), false);
        assert_eq!(has_sub("oa", "eidboaoo"), true);
    }
}
