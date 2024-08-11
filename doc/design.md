# idea

have a separate tester tool, which will download inputs (to a global location like tmp),
run the executables with that input and verify the results.

can verify the results by showing the user, asking them if it's correct, and if it is
storing it in a global location, and next time can use it to test the solutions again.


this crate could then be installed by other users and their inputs can also be validated.

although then, maybe we don't need separate executables...

without separate executables, we'd need to maintain a map of solutions to inputs manually.

or just codegen it, codegen is way easier in rust (parameterizable?).

separate executables are nice because:
- they are nicely separated, and easily runnable
- no mapping needed, just map to the bin name (assuming they are installed?)

how can we run the other binaries without assuming they're in the path?

or assume they're on the path, and just put `target/debug` in the path while developing.
they should be in the path when installed.

in fact, this could be used to test any suite of aoc solutions!

# impl

# run solution

An executable is specified, where the executable name is the name of the day that needs to be solved.
e.g. aoc2015_01, aoc2031_25 etc

The local cache is checked for input, if it is not present, it is fetched from the API.
The API key should be present specified as an environment variable.

The executable is run with the input provided via stdin.

The executable returns the output on stdout, one line per solution part, and exits with a non-zero status if there is an error.

The answer cache is checked to see if the solution is already solved.
If it is already solved, the provided answer is checked against the cached answer, and the status is returned.
If is it not present in the cache, the user is prompted to check the answer on aoc, if it is correct, it is stored in the cache.
If it is incorrect, the answer is discarded.

