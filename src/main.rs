// mod cmd;
mod double_pts;
mod slice_window;
//167 给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列  ，请你从数组中找出满足相加之和等于目标数 target 的两个数。如果设这两个数分别是 numbers[index1] 和 numbers[index2] ，则 1 <= index1 < index2 <= numbers.length 。

// 以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2。

// 你可以假设每个输入 只对应唯一的答案 ，而且你 不可以 重复使用相同的元素。

// 你所设计的解决方案必须只使用常量级的额外空间。
// int[] twoSum(int[] nums, int target) {
//     // 一左一右两个指针相向而行
//     int left = 0, right = nums.length - 1;
//     while (left < right) {
//         int sum = nums[left] + nums[right];
//         if (sum == target) {
//             // 题目要求的索引是从 1 开始的
//             return new int[]{left + 1, right + 1};
//         } else if (sum < target) {
//             left++; // 让 sum 大一点
//         } else if (sum > target) {
//             right--; // 让 sum 小一点
//         }
//     }
//     return new int[]{-1, -1};
// }
fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        }
    }
    None
}
fn main() {
    assert_eq!(two_sum(vec![1, 2, 3, 9], 5).unwrap(), (1, 2));
    assert_eq!(two_sum(vec![1, 2, 3, 9], 22), None);
}
