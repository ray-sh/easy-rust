// mod cmd;
mod double_pts;
mod slice_window;

// 5  给你一个字符串 s，找到 s 中最长的回文子串。

// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
// 在 s 中寻找以 s[l] 和 s[r] 为中心的最长回文串

fn palindrome<'a>(s: &'a str, mut l: i32, mut r: i32) -> &'a str {
    while r < s.len() as i32 && l >= 0 && s.chars().nth(l as usize) == s.chars().nth(r as usize) {
        l -= 1;
        r += 1;
    }
    &s[(l + 1) as usize..r as usize]
}
fn longest_palindrome(s: &str) -> &str {
    let mut i: i32 = 0;
    let mut result = "";
    while i < s.len() as i32 {
        let s1 = palindrome(s, i, i);
        let s2 = palindrome(s, i, i + 1);
        if s1.len() > result.len() {
            result = s1;
        }
        if s2.len() > result.len() {
            result = s2;
        }
        i += 1;
    }
    result
}
fn main() {
    println!("{}", palindrome("babad", 0, 2));
    println!("{}", longest_palindrome("babcbab"));
}
