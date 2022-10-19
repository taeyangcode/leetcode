// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

#include <vector>

class Solution {
   public:
    int maxProfit(std::vector<int>& prices) {
        int min = prices[0];
        int max = prices[0];
        int newMin = prices[0];

        for (int i = 1; i < prices.size(); ++i) {
            if (prices[i] < newMin) {
                newMin = prices[i];
                continue;
            }
            if (prices[i] - newMin > max - min) {
                min = newMin;
                max = prices[i];
            }
        }
        return max - min;
    }
};