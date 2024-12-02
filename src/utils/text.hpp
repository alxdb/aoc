#pragma once

#include <istream>
#include <ranges>

namespace aoc {

inline auto lines(std::istream &in) {
  auto in_r = std::ranges::subrange{std::istreambuf_iterator<char>{in},
                                    std::istreambuf_iterator<char>{}};
  return in_r | std::views::lazy_split('\n');
}

} // namespace aoc
