// https://leetcode.com/problems/stone-game/

#include <vector>

class Solution {
   public:
    int stoneGame(std::vector<int>& piles) {
        int oddSum = 0, evenSum = 0;
        for (int i = 0; i < piles.size(); ++i) {
            if (i & 1) {
                oddSum += piles[i];
            } else {
                evenSum += piles[i];
            }
        }
        return (oddSum > evenSum) * oddSum + (evenSum > oddSum) * evenSum;
    }
};

#include <vector>

class Solution {
   public:
    int stoneGame(std::vector<int>& piles) {
        return true;
    }
};