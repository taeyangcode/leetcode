// https://leetcode.com/problems/climbing-stairs/

/*
 * Approach: The pattern behind climbStairs reflects the fibonacci sequence
 * with the slight caveat that the solution will be fibonacci(n + 1) due to
 * the lower bound of variable n being 1 rather than 0.
 */

class Solution {
   public:
    int climbStairs(int n) {
        if (n <= 3) {
            return n;
        }
        int laster = 2, last = 3, current = 5;
        for (int i = 4; i < n; ++i) {
            laster = last;
            last = current;
            current = laster + last;
        }
        return current;
    }
};