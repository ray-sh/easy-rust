// mod cmd;
// use cmd;
// mod cmd;

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

fn longest_substrings(s: &str) -> usize {
    let mut max_len = 0;
    let mut left = 0;
    let mut right = 0;
    while right < s.len() {
        let current_char = s.chars().nth(right).unwrap();
        println!("current char {}", current_char);
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
fn mycontains(s: &str, t: &str) -> bool {
    t.chars().all(|c| s.contains(c))
}
fn min_sup_str<'a>(source: &'a str, target: &str) -> &'a str {
    let mut left = 0;
    let mut min_str = "";
    for right in 0..source.len() {
        if mycontains(&source[left..right], target) {
            if min_str.is_empty() || min_str.len() > (right - left) {
                min_str = &source[left..right];
                left = right;
            }
        }
    }
    min_str
}

fn main() {
    // println!("{}", longest_substrings("abcabcbb"));
    println!("{}", mycontains("asedfcb", "abc"));
   println!("{}", min_sup_str("abdfcb", "abc"));
}
