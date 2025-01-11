package alxdb.me.aoc

import alxdb.me.aoc.input.fetchInput
import alxdb.me.aoc.solutions.Solution
import kotlin.system.exitProcess
import kotlin.time.measureTimedValue

fun parseArgs(args: Array<String>): Pair<Int, Int> {
    if (args.size != 2) {
        System.err.println("aoc: Solve advent of code problems")
        System.err.println("Usage: <year> <day>")
        exitProcess(-1)
    }
    val year: Int
    val day: Int
    try {
        year = args[0].toInt()
    } catch (e: NumberFormatException) {
        System.err.println("Year is not a valid integer")
        exitProcess(-1)
    }
    try {
        day = args[1].toInt()
    } catch (e: NumberFormatException) {
        System.err.println("Day is not a valid integer")
        exitProcess(-1)
    }
    return Pair(year, day)
}

fun main(args: Array<String>) {
    val (year, day) = parseArgs(args)

    val solution = Solution.pick(year, day)
    if (solution == null) {
        System.err.println("Problem not yet solved")
        exitProcess(-1)
    }
    val input = fetchInput(year, day)

    val part1Solution = measureTimedValue { solution.part1(input) }
    println("Part 1 Solution: ${part1Solution.value}")
    println("Solved in ${part1Solution.duration}")

    val part2Solution = measureTimedValue { solution.part2(input) }
    println("Part 2 Solution: ${part2Solution.value}")
    println("Solved in ${part2Solution.duration}")
}