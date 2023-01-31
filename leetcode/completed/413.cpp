// https://leetcode.com/problems/arithmetic-slices/

// Initial Solution - Recursive

#include <vector>

class Solution {
   public:
    int numberOfArithmeticSlices(std::vector<int>& numbers) {
        const std::size_t& size = numbers.size();

        if (size < 2) {
            return 0;
        }

        int result = 0, streak = 0;
        for (std::size_t index = 2, difference = numbers[1] - numbers[0]; index < size; ++index) {
            const int current_difference = numbers[index] - numbers[index - 1];
            if (current_difference == difference) {
                ++streak;
            }
            else {
                result += (streak * streak + streak) / 2;
                difference = current_difference;
                streak = 0;
            }
        }
        result += (streak * streak + streak) / 2;
        return result;
    }
};

// Iterative solution

#include <vector>

class Solution {
   public:
    int numberOfArithmeticSlices(std::vector<int>& numbers) {
        const std::size_t& size = numbers.size();

        if (size < 2) {
            return 0;
        }

        int result = 0, streak = 0;
        for (std::size_t index = 2, difference = numbers[1] - numbers[0]; index < size; ++index) {
            const int current_difference = numbers[index] - numbers[index - 1];
            if (current_difference == difference) {
                ++streak;
            }
            else {
                result += (streak * streak + streak) / 2;
                difference = current_difference;
                streak = 0;
            }
        }
        result += (streak * streak + streak) / 2;
        return result;
    }
};
