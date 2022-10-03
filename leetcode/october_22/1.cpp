// https://leetcode.com/problems/decode-ways/

#include <string>
#include <unordered_map>

class Solution {
   private:
    std::unordered_map<int, int> cache;

   public:
    int numDecodings(std::string s) {
        if (s.length() == 1) {
            return (s[0] != '0');
        }
        return this->decode(s);
    }

    int decode(std::string& str, int index = 0) {
        if (str[index] == '0') {
            return 0;
        }
        if (index >= str.length() - 1) {
            return 1;
        }
        if (this->cache.find(index) != this->cache.end()) {
            return this->cache[index];
        }

        if (std::stoi(str.substr(index, 2)) <= 26) {
            this->cache[index] = this->decode(str, index + 1) + this->decode(str, index + 2);
        } else {
            this->cache[index] = this->decode(str, index + 1);
        }
        return this->cache[index];
    }
};