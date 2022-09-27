// https://leetcode.com/problems/binary-tree-pruning/

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
    TreeNode* pruneTree(TreeNode* root) {
        this->prune(root);
        return (!this->hasChild(root) && root->val == 0) ? nullptr : root;
    }

    bool hasChild(TreeNode* node) {
        return node->left != nullptr || node->right != nullptr;
    }

    void prune(TreeNode* node) {
        if (node == nullptr) {
            return;
        }

        prune(node->left);
        prune(node->right);

        if (node->left != nullptr && !this->hasChild(node->left) && node->left->val == 0) {
            delete node->left;
            node->left = nullptr;
        }
        if (node->right != nullptr && !this->hasChild(node->right) && node->right->val == 0) {
            delete node->right;
            node->right = nullptr;
        }
    }
};