const std = @import("std");
const aoc = @import("aoc");

pub fn main() !void {
    var dba = std.heap.DebugAllocator(.{}).init;
    defer _ = dba.deinit();
    const a = dba.allocator();

    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);

    if (args.len != 3) {
        std.log.err("Please provide a year and day as positional arguments", .{});
        std.process.exit(2);
    }
    const year = std.fmt.parseInt(u16, args[1], 10) catch {
        std.log.err("'{s}' is not a valid year.", .{args[1]});
        std.process.exit(2);
    };
    const day = std.fmt.parseInt(u16, args[2], 10) catch {
        std.log.err("'{s}' is not a valid day.", .{args[2]});
        std.process.exit(2);
    };

    var stdin_buffer: [1024]u8 = undefined;
    var stdin_reader = std.fs.File.stdin().reader(&stdin_buffer);
    const stdin = &stdin_reader.interface;

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var input_writer = std.Io.Writer.Allocating.init(a);
    defer input_writer.deinit();

    std.log.info("Solving problem for year {d:0>4}, day {d:0>2}", .{ year, day });
    const problemId = aoc.ProblemId.init(year, day);
    try aoc.solveProblem(problemId, stdin, stdout);
}
