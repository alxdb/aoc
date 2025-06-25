#include <iostream>
#include <print>
#include <unordered_set>
#include <vector>

std::vector<std::string> parse_lines() {
  std::vector<std::string> lines;
  std::string current_line;
  while (std::getline(std::cin, current_line)) {
    lines.push_back(current_line);
  }
  return lines;
}

auto main() -> int {
  const auto lines = parse_lines();

  std::unordered_set<std::string> antinodes;
  for (int row_n = 0; row_n < lines.size(); row_n++) {
    const auto &line = lines[row_n];
    for (int col_n = 0; col_n < line.size(); col_n++) {
      if (line[col_n] == '.') {
        continue;
      }
      // std::println(std::cerr, "found {} at ({}, {})", line[col_n], row_n, col_n);
      auto col_range = 0;
      auto row_range = 0;
      auto check_col_bounds = [&](auto range) { return col_n + range < line.size() && col_n - range >= 0; };
      auto check_row_bounds = [&](auto range) { return row_n + range < lines.size() && row_n - range >= 0; };
      auto next_col_in_range = true;
      auto next_row_in_range = true;
      do {
        next_col_in_range = check_col_bounds(col_range + 1);
        next_row_in_range = check_row_bounds(row_range + 1);
        if (next_col_in_range) {
          col_range += 1;
        }
        if (next_row_in_range) {
          row_range += 1;
        }
        auto check_for_antinode = [&](auto sub_col, auto sub_row) {
          int col_a = col_n + sub_col;
          int row_a = row_n + sub_row;
          if (lines[row_a][col_a] != line[col_n]) {
            return;
          }
          int col_b = col_n - sub_col;
          int row_b = row_n - sub_row;
          if (col_b >= 0
              && col_b < line.size()
              && row_b >= 0
              && row_b < lines.size()
              && !antinodes.contains(std::format("{},{}", row_b, col_b))) {
            std::println(std::cerr, "[{}, {}] [{}, {}] found antinode for ({}, {}) - ({}, {}) at ({}, {})",
                         col_n, row_n, row_range, col_range, row_a, col_a, sub_col, sub_row, row_b, col_b);
            antinodes.insert(std::format("{},{}", row_b, col_b));
          }
        };

        if (row_range > 0) {
          for (int sub_col = -col_range; sub_col <= col_range; sub_col++) {
            check_for_antinode(sub_col, -row_range);
          }
        }
        for (int sub_row = -row_range + 1; sub_row < row_range; sub_row++) {
          check_for_antinode(-col_range, sub_row);
          check_for_antinode(+col_range, sub_row);
        }
        if (row_range > 0) {
          for (int sub_col = -col_range; sub_col <= col_range; sub_col++) {
            check_for_antinode(sub_col, +row_range);
          }
        }
      } while (next_col_in_range || next_row_in_range);
    }
  }

  for (int i = 0; i < lines.size(); i++) {
    auto line = lines[i];
    for (int j = 0; j < line.size(); j++) {
      if (antinodes.contains(std::format("{},{}", i, j))) {
        if (line[j] == '.') {
          line[j] = '#';
        } else {
          line[j] = '-';
        }
      }
    }
    std::println("{}", line);
  }

  // std::println("{}", antinodes.size());
  // std::cout << antinodes.size() << '\n';
}

// 339 still too high
// 335 still too high
// 816 too high

// int col_a = col_n + sub_col;
// int row_a = row_n - row_range;
// if (lines[row_a][col_a] != line[col_n]) {
//   continue;
// }
// int col_b = col_n - sub_col;
// int row_b = row_n + row_range;
// if (col_b >= 0 && col_b < line.size() && row_b >= 0 && row_b < lines.size()) {
//   n_antinodes += 1;
// }
