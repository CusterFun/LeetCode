impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0i32, |res, val| res ^ val)
    }

    pub fn single_number_2(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^= num
        }
        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
