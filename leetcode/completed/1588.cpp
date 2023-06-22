// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/

/*
 * Dynamic programming solution
 * Bad time and space complexity; though interesting to understand!
 */

#include <unordered_map>
#include <utility>
#include <vector>

class Solution {
   public:
    int sumOddLengthSubarrays(std::vector<int>& arr) {
        std::unordered_map<int, std::unordered_map<int, int> > cache;
        int result = 0, size = arr.size();
        for (int i = 0; i < size; ++i) {
            result += cache[i][i] = arr[i];
            for (int j = i + 2; j < size; j += 2) {
                result += cache[i][j] = cache[i][j - 2] + arr[j - 1] + arr[j];
            }
        }
        return result;
    }
};

/*
 * O(n) Time approach
 */
#include <vector>

class Solution {
   public:
    int sumOddLengthSubarrays(std::vector<int>& arr) {
        int result = 0, size = arr.size();
        for (int i = 0; i < size; ++i) {
            result += ((i + 1) * (size - i) + 1) / 2 * arr[i];
        }
        return result;
    }
};