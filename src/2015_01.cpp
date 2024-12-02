#include <text.hpp>

#include <iostream>

auto main() -> int {
  std::optional<char> p;
  std::istreambuf_iterator<char> is{std::cin}, end;
  while (true) {
    char c = *is;
    is++;
    if (p) {
      std::cout << *p;
    }
    if (is == end) {
      if (c != '\n') {
        std::cout << c;
      }
      break;
    }
    p = c;
  }
}
