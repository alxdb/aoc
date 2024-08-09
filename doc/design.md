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
