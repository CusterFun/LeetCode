struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        let mut dp = vec![vec![0; amount as usize + 1]; n];
        for c in 0..=amount / coins[0] {
            dp[0][(c * coins[0]) as usize] = 1;
        }
        for i in 1..n {
            for j in 0..=amount as usize {
                let k = j / coins[i] as usize;
                for c in 0..=k {
                    dp[i][j] += dp[i - 1][j - c * coins[i] as usize];
                }
            }
        }

        dp[n - 1][amount as usize]
    }

    pub fn change2(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for j in 0..coins.len() {
            for i in 0..=amount as usize {
                if i >= coins[j] as usize {
                    dp[i] += dp[i - coins[j] as usize];
                }
            }
        }
        dp[amount as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change2(3, vec![2]), 0);
        assert_eq!(Solution::change2(10, vec![10]), 1);
    }
}
