// mod cmd;
mod double_pts;
mod slice_window;
// 给你一个 升序排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
fn dedup(nums: &mut Vec<i32>) -> &Vec<i32> {
    todo!()
}
fn main() {
    let mut nums = vec![1, 1, 2, 3, 4, 4, 5];
    println!("{:?}", dedup(&mut nums));
}
