// https://leetcode.com/problems/pascals-triangle-ii/

#include <vector>

class Solution {
   public:
    std::vector<int> getRow(int rowIndex) {
        std::vector<int> prev(rowIndex + 1);
        std::vector<int> row(rowIndex + 1);
        for (int i = 0; i <= rowIndex; ++i) {
            row[0] = row[i] = 1;
            for (int n = 1; n < i; ++n) {
                row[n] = prev[n - 1] + prev[n];
            }
            prev = row;
        }
        return row;
    }
};