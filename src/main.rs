// mod cmd;
mod double_pts;
mod slice_window;
// 344 题「反转字符串」就是类似的需求，让你反转一个 char[] 类型的字符数组，我们直接看代码吧：
// void reverseString(char[] s) {
//     // 一左一右两个指针相向而行
//     int left = 0, right = s.length - 1;
//     while (left < right) {
//         // 交换 s[left] 和 s[right]
//         char temp = s[left];
//         s[left] = s[right];
//         s[right] = temp;
//         left++;
//         right--;
//     }
// }
fn reverse_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;
    while left < right {
        let tmp = chars[left];
        chars[left] = chars[right];
        chars[right] = tmp;
        left += 1;
        right -= 1
    }
    chars.iter().collect()
}
fn main() {
    assert_eq!(reverse_str("abc"), "cab");
}
