cmake_minimum_required(VERSION 3.29)
project(aoc)

include(CTest)

add_executable(main main.cpp)

add_executable(main_test main.t.cpp)
add_test(main_test main_test)
