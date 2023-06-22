// https://leetcode.com/problems/count-good-nodes-in-binary-tree/

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
    int goodNodes(TreeNode* root) {
        return 1 + this->nodeHelper(root);
    }

    int nodeHelper(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        unsigned int goodNodeCount = 0;
        if (root->left != nullptr) {
            if (root->val > root->left->val) {
                root->left->val = root->val;
            } else {
                ++goodNodeCount;
            }
        }
        if (root->right != nullptr) {
            if (root->val > root->right->val) {
                root->right->val = root->val;
            } else {
                ++goodNodeCount;
            }
        }
        return goodNodeCount + nodeHelper(root->left) + nodeHelper(root->right);
    }
};