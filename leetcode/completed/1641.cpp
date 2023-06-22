// https://leetcode.com/problems/count-sorted-vowel-strings/

class Solution {
   public:
    int countVowelStrings(int n) {
        if (n == 1) {
            return 5;
        }
        int* cache = new int[5];
        cache[0] = cache[1] = cache[2] = cache[3] = cache[4] = 1;
        while (--n) {
            cache[3] = cache[3] + cache[4];
            cache[2] = cache[2] + cache[3];
            cache[1] = cache[1] + cache[2];
            cache[0] = cache[0] + cache[1];
        }
        return cache[0] + cache[1] + cache[2] + cache[3] + cache[4];
    }
};