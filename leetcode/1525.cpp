// https://leetcode.com/problems/number-of-good-ways-to-split-a-string/submissions/

/*
 * Solution using HashMap (unordered_map)
 */

#include <string>
#include <unordered_map>
#include <vector>

class Solution {
   public:
    int numSplits(std::string s) {
        unsigned int result = 0;
        std::size_t size = s.size();
        std::vector<unsigned int> leftChars(size), rightChars(size);
        std::unordered_map<char, unsigned int> leftCharCount, rightCharCount;

        for (unsigned int index = 0, leftGoodWays = 0, rightGoodWays = 0; index < size; ++index) {
            leftChars[index] = leftGoodWays += (++leftCharCount[s[index]] == 1);
            rightChars[size - index - 1] = rightGoodWays += (++rightCharCount[s[size - index - 1]] == 1);
        }
        for (unsigned int index = 0; index < size - 1; ++index) {
            result += (leftChars[index] == rightChars[index + 1]);
        }
        return result;
    }
};

/*
 * Solution using HashSet (unordered_set)
 */

#include <string>
#include <unordered_set>
#include <vector>

class Solution {
   public:
    int numSplits(std::string s) {
        unsigned int result = 0;
        std::size_t size = s.size();
        std::vector<unsigned int> leftChars(size), rightChars(size);
        std::unordered_set<char> leftUnique, rightUnique;

        for (unsigned int index = 0, leftGoodWays = 0, rightGoodWays = 0; index < size; ++index) {
            if (leftUnique.find(s[index]) == leftUnique.end()) {
                ++leftGoodWays;
                leftUnique.insert(s[index]);
            }
            if (rightUnique.find(s[size - index - 1]) == rightUnique.end()) {
                ++rightGoodWays;
                rightUnique.insert(s[size - index - 1]);
            }
            leftChars[index] = leftGoodWays;
            rightChars[size - index - 1] = rightGoodWays;
        }
        for (unsigned int index = 0; index < size - 1; ++index) {
            result += (leftChars[index] == rightChars[index + 1]);
        }
        return result;
    }
};