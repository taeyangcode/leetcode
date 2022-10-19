// https://leetcode.com/problems/min-cost-climbing-stairs/

#include <vector>

class Solution {
   public:
    int minCostClimbingStairs(std::vector<int> cost) {
        std::vector<int> cache(cost.size() + 1, -1);
        return std::min(this->findMinCost(cost, cache, cost.size() - 1), this->findMinCost(cost, cache, cost.size() - 2));
    }

    int findMinCost(std::vector<int>& cost, std::vector<int>& cache, int position) {
        if (position == 0 || position == 1) {
            return cost[position];
        }
        if (cache[position] != -1) {
            return cache[position];
        }
        cache[position] = std::min(this->findMinCost(cost, cache, position - 1) + cost[position], this->findMinCost(cost, cache, position - 2) + cost[position]);
        return cache[position];
    }
};