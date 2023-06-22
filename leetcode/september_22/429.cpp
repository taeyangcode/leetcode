// https://leetcode.com/problems/n-ary-tree-level-order-traversal/

/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
   public:
    vector<vector<int>> levelOrder(Node* root) {
        std::vector<std::vector<int>> result;
        this->fillResult(result, root, 0);
        return result;
    }

    void fillResult(std::vector<std::vector<int>>& v, Node* node, int level) {
        if (node == nullptr) {
            return;
        }

        if (level >= v.size()) {
            std::vector<int> level;
            level.push_back(node->val);
            v.push_back(level);
        } else {
            v[level].push_back(node->val);
        }

        for (Node* child : node->children) {
            this->fillResult(v, child, level + 1);
        }
    }
};