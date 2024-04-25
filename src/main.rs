// mod cmd;
mod slice_window;

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

fn main() {
    // println!("{}", longest_substrings("abcabcbb"));
    // println!("{}", mycontains("asedfcb", "abc"));
    println!("{}", min_window("ADOBECODEBANC", "ABC"));
}
