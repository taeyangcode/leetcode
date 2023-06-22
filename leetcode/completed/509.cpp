// https://leetcode.com/problems/fibonacci-number/

/*
 * Strategy: Recursion
 * Time:  O(2^n)
 * Space: O(n)
 */
class Solution {
   public:
    int fib(int n) {
        if (n <= 2) {
            return n != 0;
        }
        return this->fib(n - 1) + this->fib(n - 2);
    }
};

/*
 * Strategy: Dyanamic Programming - Caching all fibonacci solutions
 * Time:  O(n)
 * Space: O(n + 1)
 */
class Solution {
   public:
    int fib(int n) {
        int* cache = new int[n + 1];
        if (n == 0 || n == 1) {
            return n != 0;
        }
        cache[0] = 0;
        cache[1] = 1;
        for (int i = 2; i <= n; ++i) {
            cache[i] = cache[i - 1] + cache[i - 2];
        }
        return cache[n];
    }
};

/*
 * Strategy: Dynamic Programming - Caching only last three (required) solutions
 * Time:  O(n)
 * Space: O(1)
 */
class Solution {
   public:
    int fib(int n) {
        if (n <= 2) {
            return n != 0;
        }
        int lastest = 1, last = 1, current = 2;
        for (int i = 4; i <= n; ++i) {
            lastest = last;
            last = current;
            current = lastest + last;
        }
        return current;
    }
};