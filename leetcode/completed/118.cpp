// https://leetcode.com/problems/pascals-triangle/

#include <vector>

class Solution {
   public:
    std::vector<std::vector<int>> generate(int numRows) {
        std::vector<std::vector<int>> result(numRows);
        for (int i = 0; i < numRows; ++i) {
            result[i].resize(i + 1);
            for (int n = 0; n <= i; ++n) {
                result[i][n] = (n == 0 || n == i)
                                   ? result[i][n] = 1
                                   : result[i - 1][n - 1] + result[i - 1][n];
            }
        }
        return result;
    }
};