const std = @import("std");

pub fn parseSolutionName(solution_name: []const u8) !struct { year: u16, day: u8 } {
    var it = std.mem.split(u8, solution_name, "_");
    const year_s = it.next() orelse return error.InvalidSolutionName;
    const day_s = it.next() orelse return error.InvalidSolutionName;
    if (it.next() != null) {
        return error.InvalidSolutionName;
    }

    return .{
        .year = try std.fmt.parseInt(u16, year_s, 0),
        .day = try std.fmt.parseInt(u8, day_s, 0),
    };
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const aoc_token = (try std.process.getEnvMap(alloc)).get("AOC_TOKEN") orelse {
        std.log.err("No AOC_TOKEN envvar found", .{});
        std.process.exit(255);
    };
    const aoc_cookie = try std.fmt.allocPrint(alloc, "session={s}", .{aoc_token});

    var arg_iter = try std.process.ArgIterator.initWithAllocator(alloc);
    std.debug.assert(arg_iter.skip());

    const solution_name = arg_iter.next() orelse {
        std.log.err("No solution name provided", .{});
        std.process.exit(255);
    };

    const solution_id = parseSolutionName(solution_name) catch {
        std.log.err("Invalid solution name: {s}", .{solution_name});
        std.process.exit(255);
    };
    const url =
        try std.fmt.allocPrint(
        alloc,
        "https://adventofcode.com/{}/day/{}/input",
        solution_id,
    );

    var client = std.http.Client{ .allocator = alloc };
    var response_data = std.ArrayList(u8).init(alloc);
    const request = .{
        .location = .{ .url = url },
        .extra_headers = &.{
            .{
                .name = "Cookie",
                .value = aoc_cookie,
            },
        },
        .response_storage = .{ .dynamic = &response_data },
    };
    const response = try client.fetch(request);
    if (response.status != std.http.Status.ok) {
        std.log.err("Invalid response from aoc ({}) {s}", .{ response.status, response_data.items });
        std.process.exit(255);
    }

    const stdout = std.io.getStdOut().writer();
    try stdout.print("{s}", .{response_data.items});
}
