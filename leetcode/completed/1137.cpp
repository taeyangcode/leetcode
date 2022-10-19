// https://leetcode.com/problems/n-th-tribonacci-number/submissions/

class Solution {
   public:
    int tribonacci(int n) {
        if (n == 0) {
            return 0;
        }
        if (n == 1) {
            return 1;
        }
        if (n == 2) {
            return 1;
        }
        if (n == 3) {
            return 2;
        }
        int lastest = 0, laster = 1, last = 1, current = 2;
        for (int i = 4; i <= n; ++i) {
            lastest = laster;
            laster = last;
            last = current;
            current = lastest + laster + last;
        }
        return current;
    }
};