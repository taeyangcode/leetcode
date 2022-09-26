// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

class Solution {
   public:
    vector<int> numsSameConsecDiff(int n, int k) {
        std::unordered_set<int> set;
        vector<int> v;
        for (unsigned int i = 1; i <= 9; ++i) {
            insertNum(set, std::to_string(i), i, n, k);
        }
        for (const auto e : set) {
            v.push_back(e);
        }
        return v;
    }

    void insertNum(std::unordered_set<int>& set, std::string str, int n, int len, int dif) {
        if (str.length() == len) {
            set.insert(std::stoi(str));
            return;
        }

        int back = str.back() - '0';
        if (back - dif >= 0) {
            insertNum(set, str + std::to_string(back - dif), n, len, dif);
        }
        if (back + dif < 10) {
            insertNum(set, str + std::to_string(back + dif), n, len, dif);
        }
    }
};