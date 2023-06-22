// https://leetcode.com/problems/sort-integers-by-the-power-value/

#include <map>
#include <unordered_map>

class Solution {
   public:
    int getKth(int lo, int hi, int k) {
        std::unordered_map<int, int> cache;
        std::multimap<int, int> result;
        for (int count = 0; lo <= hi; ++lo) {
            for (int i = lo; i != 1; i = (i % 2 == 0) ? (i / 2) : (i * 3) + 1, ++count) {
                if (cache.find(i) != cache.end()) {
                    cache[lo] = cache[i];
                    break;
                }
            }
            cache[lo] += count;
            result.insert({cache[lo], lo});
            count = 0;
        }
        for (std::pair<int, int> p : result) {
            if (--k == 0) {
                return p.second;
            }
        }
        return -1;
    }
};