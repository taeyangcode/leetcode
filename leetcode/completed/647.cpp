// https://leetcode.com/problems/palindromic-substrings/description/

#include <string>

class Solution {
   public:
    int countSubstrings(std::string input) {
        const std::size_t length = input.size();
        int result = 0;
        for (std::size_t index = 0; index < length; ++index) {
            result += this->palindrone(input, length, index, index) + this->palindrone(input, length, index, index + 1);
        }
        return result;
    }

    int palindrone(const std::string& string, const std::size_t& length, const std::size_t left_index, const std::size_t right_index) {
        int palindrone_count = 0;
        for (std::size_t left = left_index, right = right_index;
            (left <= right) && (left >= 0 && right < length) && (string[left] == string[right]);
            --left, ++right, ++palindrone_count) {
        }
        return palindrone_count;
    }
};

/*
 * 128/130 - Fails
 *
 * #include <string>
 * #include <unordered_map>
 * 
 * class Solution {
 *    private:
 *     std::unordered_map<std::string, bool> cache;
 * 
 *    public:
 *     int countSubstrings(std::string s) {
 *         const std::size_t size = s.size();
 *         return (size == 1)
 *             ? 1
 *             : size + this->findPalindrones(s, size, 2, 0);
 *     }
 * 
 *     int findPalindrones(const std::string& word, const std::size_t wordLength, const std::size_t substringLength, const std::size_t index) {
 *         if (substringLength == wordLength) {
 *             return this->isPalindrone(word);
 *         }
 * 
 *         if (index + substringLength > wordLength) {
 *             return this->findPalindrones(word, wordLength, substringLength + 1, 0);
 *         }
 * 
 *         const std::string substring = word.substr(index, substringLength);
 *         if (this->cache.find(substring) != this->cache.end()) {
 *             return this->cache[substring] + this->findPalindrones(word, wordLength, substringLength, index + 1);
 *         }
 *         return this->isPalindrone(substring) + this->findPalindrones(word, wordLength, substringLength, index + 1);
 *     }
 * 
 *     bool isPalindrone(const std::string string) {
 *        for (std::size_t start = 0, end = string.size() - 1; start < end; ++start, --end) {
 *             const std::string substring = string.substr(start, end - start + 1);
 *             if (this->cache.find(substring) != this->cache.end()) {
 *                 this->cache[string] = this->cache[substring];
 *                 return this->cache[string];
 *             }
 * 
 *             if (string[start] != string[end]) {
 *                 this->cache[string] = false;
 *                 return false;
 *             }
 *         }
 *         this->cache[string] = true;
 *         return true;
 *     }
 * };
 */
