// mod cmd;
mod double_pts;
mod dp;
mod slice_window;
// 力扣第 509 题「斐波那契数」就是这个问题，请读者不要嫌弃这个例子简单，只有简单的例子才能让你把精力充分集中在算法背后的通用思想和技巧上，而不会被那些隐晦的细节问题搞的莫名其妙。想要困难的例子，接下来的动态规划系列里有的是。
// int fib(int N) {
//     // 备忘录全初始化为 0
//     int[] memo = new int[N + 1];
//     // 进行带备忘录的递归
//     return dp(memo, N);
// }

// int fib(int n) {
//     if (n == 0 || n == 1) {
//         // base case
//         return n;
//     }
//     // 分别代表 dp[i - 1] 和 dp[i - 2]
//     int dp_i_1 = 1, dp_i_2 = 0;
//     for (int i = 2; i <= n; i++) {
//         // dp[i] = dp[i - 1] + dp[i - 2];
//         int dp_i = dp_i_1 + dp_i_2;
//         // 滚动更新
//         dp_i_2 = dp_i_1;
//         dp_i_1 = dp_i;
//     }
//     return dp_i_1;
// }

fn fib(n: usize) -> u128 {
    if n == 0 || n == 1 {
        return n as u128;
    }
    let mut dp_i_2 = 1;
    let mut dp_i_1 = 0;
    for _ in 2..=n {
        let dp_i = dp_i_1 + dp_i_2;
        dp_i_1 = dp_i_2;
        dp_i_2 = dp_i;
    }
    // Return the nth Fibonacci number
    return dp_i_2;
}

fn main() {
    println!("{}", fib(10));
}
