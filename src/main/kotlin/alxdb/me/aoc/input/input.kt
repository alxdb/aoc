package alxdb.me.aoc.input

fun fetchInput(year: Int, day: Int): String =
    fetchInputFromFile(year, day) ?: fetchInputFromApi(year, day).also { writeInputFile(year, day, it) }