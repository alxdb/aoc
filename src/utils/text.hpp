import std;

auto lines(std::istream &in) {
  return std::ranges::subrange{std::istreambuf_iterator<char>{in},
                               std::istreambuf_iterator<char>{}}
         | std::views::lazy_split('\n')
         | std::views::filter([](auto r) { return (r.begin() == r.end()); });
}
