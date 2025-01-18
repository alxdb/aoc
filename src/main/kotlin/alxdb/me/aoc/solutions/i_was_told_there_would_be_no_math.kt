package alxdb.me.aoc.solutions

class IWasToldThereWouldBeNoMath : Solution() {
    data class Box(val w: Int, val l: Int, val h: Int) {
        fun surfaceArea(): Int = 2 * l * w + 2 * w * h + 2 * h * l

        fun smallestSide(): List<Int> = listOf(l, w, h).sorted().take(2)
    }

    private fun parseInput(input: String): List<Box> =
        input.lines().filterNot { it.isEmpty() }
            .map { line -> line.split("x").map { it.toInt() } }
            .map { Box(it[0], it[1], it[2]) }

    override fun part1(input: String): String =
        parseInput(input).sumOf { it.surfaceArea() + it.smallestSide().reduce { a, b -> a * b } }.toString()

    override fun part2(input: String): String {
        TODO("Not yet implemented")
    }
}