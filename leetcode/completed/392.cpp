// https://leetcode.com/problems/is-subsequence/

#include <string>

class Solution {
   public:
    bool isSubsequence(std::string s, std::string t) {
        if (s.size() == 0 || t.size() < s.size()) {
            return s.size() == 0;
        }
        int sIndex = 0;
        for (int i = 0; i < t.size(); ++i) {
            if (s[sIndex] == t[i]) {
                ++sIndex;
            }
            if (sIndex == s.size()) {
                return true;
            }
        }
        return false;
    }
};