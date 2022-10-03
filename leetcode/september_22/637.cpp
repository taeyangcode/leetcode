// https://leetcode.com/problems/average-of-levels-in-binary-tree/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
   public:
    vector<double> averageOfLevels(TreeNode* root) {
        std::map<int, std::pair<double, double>> map;
        this->fillMap(root, map, 0);
        std::vector<double> result;
        for (auto level : map) {
            result.push_back(level.second.first / level.second.second);
        }
        return result;
    }

    void fillMap(TreeNode* node, std::map<int, std::pair<double, double>>& map, int level) {
        if (node == nullptr) {
            return;
        }

        map[level].first += node->val;
        map[level].second++;

        this->fillMap(node->left, map, level + 1);
        this->fillMap(node->right, map, level + 1);
    }
};