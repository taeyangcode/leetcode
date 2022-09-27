// https://leetcode.com/problems/construct-string-from-binary-tree/

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
    string tree2str(TreeNode* root) {
        return this->traverse(root);
    }

    std::string traverse(TreeNode* node) {
        if (node == nullptr) {
            return "";
        }
        bool hasLeft = node->left != nullptr;
        bool hasRight = node->right != nullptr;
        if (hasLeft && hasRight) {
            return std::to_string(node->val) + "(" + this->traverse(node->left) + ")(" + this->traverse(node->right) + ")";
        } else if (hasLeft) {
            return std::to_string(node->val) + "(" + this->traverse(node->left) + ")";
        } else if (hasRight) {
            return std::to_string(node->val) + "()(" + this->traverse(node->right) + ")";
        }
        return std::to_string(node->val);
    }
};