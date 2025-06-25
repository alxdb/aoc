find_program(CURL curl)

add_library(aoc INTERFACE)
target_include_directories(aoc INTERFACE utils)

macro(solution year day)
  math(EXPR stripped_day ${day})
  add_custom_command(
    OUTPUT ${year}_${day}.txt
    COMMAND
      ${CURL} --cookie "session=$ENV{AOC_TOKEN}"
      https://adventofcode.com/${year}/day/${stripped_day}/input -o
      ${year}_${day}.txt
    COMMENT "Retrieving input from adventofcode.com for ${year} ${day}")
  add_custom_target(${year}_${day}_input DEPENDS ${year}_${day}.txt)

  add_executable(${year}_${day} ${year}_${day}.cpp)
  add_dependencies(${year}_${day} ${year}_${day}_input)
  target_link_libraries(${year}_${day} PRIVATE aoc)

  add_test(NAME ${year}_${day}
           COMMAND ${year}_${day} <
                   ${CMAKE_CURRENT_BUILD_DIR}/${year}_${day}.txt)
endmacro()

solution(2015 01)
solution(2024 08)
