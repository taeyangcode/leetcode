// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

// Solution 1: Brute force

#include <vector>

class Solution {
   public:
    int countSquares(std::vector<std::vector<int> >& matrix) {
        int result = 0;
        for (int i = 0; i < matrix.size(); ++i) {
            for (int j = 0; j < matrix[i].size(); ++j) {
                result += this->checkSquare(matrix, 1, i, j);
            }
        }
        return result;
    }

    int checkSquare(std::vector<std::vector<int> >& matrix, int sideLength, int x, int y) {
        if (y + sideLength > matrix[0].size() || x + sideLength > matrix.size()) {
            return 0;
        }
        for (int i = x; i < x + sideLength; ++i) {
            for (int j = y; j < y + sideLength; ++j) {
                if (matrix[i][j] == 0) {
                    return 0;
                }
            }
        }
        return 1 + this->checkSquare(matrix, sideLength + 1, x, y);
    }
};

// Solution 2: DP

#include <algorithm>
#include <vector>

class Solution {
   public:
    int countSquares(std::vector<std::vector<int> >& matrix) {
        int result = 0;
        for (int i = 0; i < matrix.size(); ++i) {
            for (int j = 0; j < matrix[0].size(); ++j) {
                if (matrix[i][j] == 0) {
                    continue;
                }
                if (i == 0 || j == 0) {
                    ++result;
                    continue;
                }
                if (matrix[i - 1][j - 1] == 1) {
                    if (matrix[i - 1][j] >= 1 && matrix[i][j - 1] >= 1) {
                        result += (matrix[i][j] = 2);
                    } else {
                        ++result;
                    }
                    continue;
                }
                result += matrix[i][j] = std::min({matrix[i - 1][j - 1], matrix[i - 1][j], matrix[i][j - 1]}) + 1;
            }
        }
        return result;
    }
};