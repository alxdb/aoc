package alxdb.me.aoc.solutions

abstract class Solution {
    abstract fun part1(input: String): String
    abstract fun part2(input: String): String

    companion object {
        fun pick(year: Int, day: Int): Solution? = when (Pair(year, day)) {
            Pair(2015, 1) -> NotQuiteLisp()
            Pair(2015, 2) -> IWasToldThereWouldBeNoMath()
            Pair(2015, 3) -> PerfectlySphericalHousesInAVacuum()
            else -> null
        }
    }
}