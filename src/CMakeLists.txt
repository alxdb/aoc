add_library(aoc INTERFACE)
target_include_directories(aoc INTERFACE utils.hpp)

macro(solution name)
  add_executable(${name} ${name}.cpp)
  target_link_libraries(${name} PUBLIC aoc)
endmacro()

solution(2015_01)
