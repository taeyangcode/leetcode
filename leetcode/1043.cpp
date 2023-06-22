#include <algorithm>
#include <vector>

class Solution {
   public:
    int maxSumAfterPartitioning(std::vector<int>& arr, int k) {
        std::vector<int> cache(arr.size());
        for (int i = 0; i < arr.size(); ++i) {
            int max = -1;
            for (int j = i; j >= 0 && j > j - k; --j) {
                if (arr[j] <= max) {
                    cache[j] = max;
                    continue;
                }
                max = arr[j];
            }
        }
    }
};