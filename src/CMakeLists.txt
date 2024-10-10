add_library(aoc)
target_sources(aoc PUBLIC utils/text.cpp)
target_include_directories(aoc PUBLIC utils)

macro(solution name)
  add_executable(${name} ${name}.cpp)
  target_link_libraries(${name} PUBLIC aoc)
endmacro()

solution(2015_01)
