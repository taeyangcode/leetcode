// https://leetcode.com/problems/get-maximum-in-generated-array/

class Solution {
   public:
    int getMaximumGenerated(int n) {
        if (n <= 2) {
            return n != 0;
        }
        int* nums = new int[n + 1];
        nums[0] = 0;
        nums[1] = 1;
        int max = 1;
        for (int i = 1; i <= n / 2; ++i) {
            nums[i * 2] = nums[i];
            if (i * 2 == n) {
                return max;
            }
            nums[i * 2 + 1] = nums[i] + nums[i + 1];
            if (nums[i * 2 + 1] > max) {
                max = nums[i * 2 + 1];
            }
        }
        return max;
    }
};