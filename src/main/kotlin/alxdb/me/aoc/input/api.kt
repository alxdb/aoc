package alxdb.me.aoc.input

import java.net.CookieManager
import java.net.HttpCookie
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

const val AOC_URL = "https://adventofcode.com"
const val AOC_TOKEN_ENV = "AOC_TOKEN"

fun fetchInputFromApi(year: Int, day: Int): String = HttpClient.newBuilder()
    .cookieHandler(authorize())
    .build().use {
        it.send(request(year, day), HttpResponse.BodyHandlers.ofString())
            .body()
    }

private fun authorize(): CookieManager = CookieManager().apply {
    val cookie = HttpCookie("session", System.getenv(AOC_TOKEN_ENV)!!).apply {
        path = "/"
        version = 0
    }
    cookieStore.add(URI(AOC_URL), cookie)
}

private fun request(year: Int, day: Int): HttpRequest =
    HttpRequest.newBuilder(URI("$AOC_URL/$year/day/$day/input")).build()