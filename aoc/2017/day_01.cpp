#include <expected>
#include <iostream>
#include <optional>

auto solution(std::istream &is)
    -> std::expected<int, std::remove_reference_t<decltype(is)>::int_type> {
  std::optional<int> prev;
  std::optional<int> first;
  while (is.good()) {
    auto next = is.get();
  }
}

auto main() -> int {}
