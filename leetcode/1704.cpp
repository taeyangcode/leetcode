#include <unordered_set>
#include <string>

class Solution {
   public:
    bool halvesAreAlike(std::string s) {
        std::unordered_set<char> vowels = {'a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'};
        unsigned int first = 0, second = 0, size = s.size(), half = (size / 2);
        for (std::size_t index = 0; index < half; ++index) {
            if (vowels.find(s[index]) != vowels.end()) {
                ++first;
            }
        }
        for (std::size_t index = half + 1; index < size; ++index) {
            if (vowels.find(s[index]) != vowels.end()) {
                ++second;
            }
            if (second > first) {
              return false;
            }
        }
        return first == second;
    }
};
