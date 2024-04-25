// mod cmd;
mod slice_window;
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
        if s1.chars().all(|c| s2[left..left + w_size].contains(c))
        {
            return true;
        }
        left = left + 1;
    }
    false
}

fn main() {
    println!("{}", has_sub("ab", "eidbaooo"));
}
