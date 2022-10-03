// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/

#include <map>
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
   public:
    std::vector<std::vector<int>> verticalTraversal(TreeNode *root) {
        std::map<int, std::map<int, std::vector<int>>> map;
        this->fillMap(map, root, 0, 0);
        std::vector<std::vector<int>> vector;
        for (auto pair : map) {
            std::vector<int> input;
            for (auto pair2 : pair.second) {
                for (int value : pair2.second) {
                    input.push_back(value);
                }
            }
            vector.push_back(input);
        }
        return vector;
    }

    void fillMap(std::map<int, std::map<int, std::vector<int>>> &map, TreeNode *node, int level, int column) {
        if (node == nullptr) {
            return;
        }

        std::vector<int> &vec = map[column][level];
        vec.push_back(node->val);
        if (vec.size() > 1) {
            std::sort(vec.begin(), vec.end());
        }

        fillMap(map, node->left, level + 1, column - 1);
        fillMap(map, node->right, level + 1, column + 1);
    }
};