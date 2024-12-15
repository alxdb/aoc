add_library(aoc INTERFACE)
target_include_directories(aoc INTERFACE utils)

macro(solution name)
  add_executable(${name} ${name}.cpp)
  target_link_libraries(${name} PRIVATE aoc)
endmacro()

solution(2015_01)
solution(2024_08)
