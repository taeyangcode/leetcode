// https://leetcode.com/problems/generate-parentheses/

#include <unordered_map>
#include <vector>

// Solution 1

class Solution {
   private:
    std::unordered_map<std::string, std::string> map;
    std::vector<std::string> result;

   public:
    std::vector<std::string> generateParenthesis(int n) {
        this->generate(n, "()");
        return this->result;
    }

    void generate(int n, std::string current) {
        if (this->map.find(current) != this->map.end()) {
            return;
        }
        if (n == 1) {
            this->result.push_back(current);
            return;
        }
        this->generate(n - 1, "()" + current);
        this->map["()" + current] = "()" + current;
        this->generate(n - 1, current + "()");
        this->map[current + "()"] = current + "()";
        for (int i = 0; i < current.size(); ++i) {
            if (current[i] == '(') {
                std::string newStr = current.substr(0, i + 1) + "()" + current.substr(i + 1, current.size());
                this->generate(n - 1, newStr);
                this->map[newStr] = newStr;
            }
        }
    }
};

// Solution 2

#include <vector>

class Solution {
   public:
    std::vector<std::string> generateParenthesis(int n) {
        std::vector<std::string> result;
        this->generate(result, n, "(", 1, 0);
        return result;
    }

    void generate(std::vector<std::string>& list, int& n, std::string current, int open, int closed) {
        if (open + closed == n * 2) {
            list.push_back(current);
            return;
        }
        if (open == closed) {
            this->generate(list, n, current + "(", open + 1, closed);
            return;
        }
        if (open == n) {
            this->generate(list, n, current + ")", open, closed + 1);
            return;
        }
        this->generate(list, n, current + "(", open + 1, closed);
        this->generate(list, n, current + ")", open, closed + 1);
    }
};