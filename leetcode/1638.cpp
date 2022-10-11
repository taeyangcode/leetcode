// https://leetcode.com/problems/count-substrings-that-differ-by-one-character/

#include <string>
#include <unordered_map>

class Solution {
   private:
    std::unordered_map<std::string, int> cache;

   public:
    int countSubstrings(std::string s, std::string t) {
        int result = 0;
        for (int i = 0; i < s.size(); ++i) {
            int start = 0;
            int difference = 1;
            for (int j = i + 1; j <= s.size(); ++j) {
                int cacheCount = 0;
                std::string sSub = s.substr(i, j - i);
                if (this->cache.find(sSub) != this->cache.end()) {
                    result += this->cache[sSub];
                    start = 0;
                    ++difference;
                    continue;
                }
                for (int k = j - i; k <= t.size(); ++k) {
                    cacheCount += this->differ(sSub, t.substr(start, difference));
                    ++start;
                }
                this->cache[sSub] = cacheCount;
                result += cacheCount;
                cacheCount = 0;
                start = 0;
                ++difference;
            }
            difference = 1;
        }
        return result;
    }

    bool differ(std::string str1, std::string str2) {
        int same = 0;
        for (int i = 0; i < str1.size(); ++i) {
            same += (str1[i] == str2[i]);
        }
        return (same + 1 == str1.size());
    }
};