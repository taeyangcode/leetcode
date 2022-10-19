// https://leetcode.com/problems/all-possible-full-binary-trees/

#include <unordered_map>
#include <vector>

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
   private:
    std::unordered_map<int, std::vector<TreeNode *> > map;

   public:
    std::vector<TreeNode *> allPossibleFBT(int n) {
        this->map[1] = {new TreeNode(0)};
        return this->findTrees(n);
    }

    std::vector<TreeNode *> findTrees(int n) {
        if (n % 2 == 0) {
            return {};
        }
        if (this->map.find(n) != this->map.end()) {
            return this->map[n];
        }

        std::vector<TreeNode *> trees;
        for (int i = 1; i < n; i += 2) {
            std::vector<TreeNode *> leftTrees = this->findTrees(i);
            std::vector<TreeNode *> rightTrees = this->findTrees(n - i - 1);

            for (TreeNode *leftTree : leftTrees) {
                for (TreeNode *rightTree : rightTrees) {
                    trees.emplace(trees.begin(), new TreeNode(0, leftTree, rightTree));
                }
            }
        }
        this->map[n] = trees;
        return trees;
    }
};