// https://leetcode.com/problems/arithmetic-slices/

#include <vector>

class Solution {
   public:
    int numberOfArithmeticSlices(std::vector<int>& numbers) {
        const std::size_t& size = numbers.size();

        if (size < 2) {
            return 0;
        }

        int result = 0;
        for (std::size_t index = 0; index < size - 2; ++index) {
            const int slices = this->count_slices(numbers, index + 1, numbers[index + 1] - numbers[index], size);
            index += slices;
            result += (slices * slices + slices) / 2;
        }
        return result;
    }

    int count_slices(const std::vector<int>& numbers, const std::size_t index, const int change, const std::size_t size) {
        if (index == size - 1) {
            return 0;
        }
        if (numbers[index + 1] - numbers[index] == change) {
            return 1 + count_slices(numbers, index + 1, change, size);
        }
        return 0;
    }
};
