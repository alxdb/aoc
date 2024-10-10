#include <algorithm>
#include <expected>
#include <iomanip>
#include <iostream>
#include <ranges>

auto lines(std::istream &in) {
  return std::ranges::subrange{std::istreambuf_iterator<char>{in},
                               std::istreambuf_iterator<char>{}}
         | std::views::lazy_split('\n')
         | std::views::filter([](auto r) { return (r.begin() == r.end()); });
}

auto main() -> int {
  for (auto line : lines(std::cin)) {
    std::string input;
    std::ranges::copy(line, std::back_inserter(input));
    std::cout << "line input: " << std::quoted(input) << '\n';
  }
}
