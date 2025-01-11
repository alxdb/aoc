package alxdb.me.aoc.solutions

class NotQuiteLisp : Solution() {

    private fun parseInput(input: String): List<Int> = input.map {
        when (it) {
            '(' -> +1
            ')' -> -1
            else -> throw RuntimeException("Invalid input")
        }
    }

    override fun part1(input: String): String = parseInput(input).sum().toString()


    override fun part2(input: String): String =
        parseInput(input).scan(0) { acc, it -> acc + it }.takeWhile { it > -1 }.size.toString()
}