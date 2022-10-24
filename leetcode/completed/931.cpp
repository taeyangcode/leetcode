// https://leetcode.com/problems/minimum-falling-path-sum/

#include <set>
#include <unordered_map>
#include <vector>

class Solution {
   private:
    std::unordered_map<unsigned int, std::unordered_map<unsigned int, int> > cache;

    int minPath(std::vector<std::vector<int> >& matrix, int row, int column, unsigned int length) {
        if (row + 1 >= length) {
            return matrix[row][column];
        }

        int minSum = matrix[row][column];
        std::set<int> sums;

        auto& nextRow = this->cache[row + 1];
        const auto& nextRowEnd = nextRow.end();
        if (column - 1 >= 0) {
            if (nextRow.find(column - 1) != nextRowEnd) {
                sums.insert(nextRow[column - 1]);
            } else {
                sums.insert(this->minPath(matrix, row + 1, column - 1, length));
            }
        }
        if (column + 1 < length) {
            if (nextRow.find(column + 1) != nextRowEnd) {
                sums.insert(nextRow[column + 1]);
            } else {
                sums.insert(this->minPath(matrix, row + 1, column + 1, length));
            }
        }
        if (nextRow.find(column) != nextRowEnd) {
            sums.insert(nextRow[column]);
        } else {
            sums.insert(this->minPath(matrix, row + 1, column, length));
        }

        return this->cache[row][column] = minSum + *sums.begin();
    }

   public:
    int minFallingPathSum(std::vector<std::vector<int> >& matrix) {
        unsigned int length = matrix.size();
        int minSum = INT_MAX;
        for (int index = 0; index < length; ++index) {
            int sum = this->minPath(matrix, 0, index, length);
            if (sum < minSum) {
                minSum = sum;
            }
        }

        return minSum;
    }
};