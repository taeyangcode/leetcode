// https://leetcode.com/problems/counting-bits/

#include <vector>

class Solution {
   public:
    std::vector<int> countBits(int n) {
        std::vector<int> result(n + 1);
        result[0] = 0;
        if (n == 0) {
            return result;
        }
        result[1] = 1;
        int power = 1;
        for (int i = 2; i <= n; ++i) {
            if ((i & (i - 1)) == 0) {
                power *= 2;
            }
            result[i] = 1 + result[i - power];
        }
        return result;
    }
};