// https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/

#include <algorithm>
#include <map>
#include <vector>

class Solution {
   private:
    std::map<std::vector<int>, unsigned int> cache;

    unsigned int constructTrees(std::vector<int>& leaves, unsigned int split) {
        std::vector<int> leftTree(leaves.begin(), leaves.begin() + split), rightTree(leaves.begin() + split, leaves.end());
        unsigned int rootValue = *std::max_element(leftTree.begin(), leftTree.end()) * *std::max_element(rightTree.begin(), rightTree.end());

        unsigned int lowestLeft = UINT_MAX;
        if (this->cache.find(leftTree) != this->cache.end()) {
            lowestLeft = this->cache[leftTree];
        } else {
            for (unsigned int index = 1; index < leftTree.size(); ++index) {
                unsigned int cost = this->constructTrees(leftTree, index);
                if (cost < lowestLeft) {
                    lowestLeft = cost;
                }
            }
            if (lowestLeft == UINT_MAX) {
                lowestLeft = 0;
            }
            this->cache[leftTree] = lowestLeft;
        }

        unsigned int lowestRight = UINT_MAX;
        if (this->cache.find(rightTree) != this->cache.end()) {
            lowestRight = this->cache[rightTree];
        } else {
            for (unsigned int index = 1; index < rightTree.size(); ++index) {
                unsigned int cost = this->constructTrees(rightTree, index);
                if (cost < lowestRight) {
                    lowestRight = cost;
                }
            }
            if (lowestRight == UINT_MAX) {
                lowestRight = 0;
            }
            this->cache[rightTree] = lowestRight;
        }

        return this->cache[leaves] = rootValue + lowestLeft + lowestRight;
    }

   public:
    int mctFromLeafValues(std::vector<int>& arr) {
        unsigned int lowestCost = UINT_MAX;
        for (unsigned int index = 1; index < arr.size(); ++index) {
            unsigned int cost = this->constructTrees(arr, index);
            if (cost < lowestCost) {
                lowestCost = cost;
            }
        }

        return lowestCost;
    }
};