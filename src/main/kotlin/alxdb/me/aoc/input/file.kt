package alxdb.me.aoc.input

import java.io.File
import java.io.FileNotFoundException

val CACHE_DIR = File(System.getProperty("user.home"), ".cache/aoc")

fun fetchInputFromFile(year: Int, day: Int): String? = try {
    String(getInputFile(year, day).readBytes())
} catch (e: FileNotFoundException) {
    null
}

fun writeInputFile(year: Int, day: Int, content: String) = getInputFile(year, day).writeText(content)

private fun getInputFile(year: Int, day: Int): File = CACHE_DIR.run {
    mkdir()
    File(this, String.format("input_%d_%02d", year, day))
}