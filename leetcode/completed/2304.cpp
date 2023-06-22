// https://leetcode.com/problems/minimum-path-cost-in-a-grid/

#include <vector>
#include <climits>
#include <algorithm>
#include <utility>

class Solution {
   public:
    int minPathCost(std::vector<std::vector<int> >& grid, std::vector<std::vector<int> >& move_cost) {
        const std::size_t rows = grid.size(), columns = grid[0].size();
        std::vector<int> costs(grid[0].begin(), grid[0].end());

        for (std::size_t current_row = 1; current_row < rows; ++current_row) {
            std::vector<int> new_costs(columns, INT_MAX);
            for (std::size_t cost_index = 0; cost_index < columns; ++cost_index) {
                for (std::size_t column_index = 0; column_index < columns; ++column_index) {
                    new_costs[cost_index] = std::min(
                        new_costs[cost_index],
                        costs[column_index] + move_cost[grid[current_row - 1][column_index]][cost_index] + grid[current_row][cost_index]
                    );
                }
            }
            costs = std::move(new_costs);
        }

        return *std::min_element(costs.begin(), costs.end());
    }
};
