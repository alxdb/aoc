#include <text.hpp>

#include <iostream>
#include <ranges>

auto main() -> int {
  for (auto line : aoc::lines(std::cin)) {
    std::cout << "line: " << (line | std::ranges::to<std::string>()) << '\n';
  }
}
