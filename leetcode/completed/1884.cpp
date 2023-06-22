// https://leetcode.com/problems/egg-drop-with-2-eggs-and-n-floors/

class Solution {
   public:
    int twoEggDrop(int n) {
        int result = 1;
        for (; n > 0; ++result, n -= result) {
        }
        return result;
    }
};