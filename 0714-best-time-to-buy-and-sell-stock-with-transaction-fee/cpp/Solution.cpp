class Solution {
public:
    int maxProfit(vector<int>& prices, int fee) {
        int n = prices.size();
        vector<vector<int>> dp(n, vector<int>(2));
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        for (int i = 1; i < n; ++i) {
            dp[i][0] = std::max(dp[i-1][0], dp[i-1][1] - prices[i]);
            dp[i][1] = std::max(dp[i-1][0] + prices[i] - fee, dp[i-1][1]);
        }
        return std::max(dp[n-1][0], dp[n-1][1]);
    }
};