// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

class Solution {
   public:
    vector<int> numsSameConsecDiff(int n, int k) {
        vector<int> v;
        for (unsigned int i = 1; i <= 9; ++i) {
            insertNum(v, std::to_string(i), i, n, k);
        }
        return v;
    }

    void insertNum(std::vector<int>& vector, std::string str, int n, int len, int dif) {
        if (str.length() == len) {
            vector.push_back(std::stoi(str));
            return;
        }

        int back = str.back() - '0';
        if (back - dif >= 0) {
            insertNum(vector, str + std::to_string(back - dif), n, len, dif);
            if (dif == 0) {
                return;
            }
        }
        if (back + dif < 10) {
            insertNum(vector, str + std::to_string(back + dif), n, len, dif);
        }
    }
};