const std = @import("std");

const input = @embedFile("input");

fn part1() !i32 {
    var floor: i32 = 0;
    for (input) |c| {
        switch (c) {
            '(' => {
                floor += 1;
            },
            ')' => {
                floor -= 1;
            },
            else => {
                return error.InvalidInput;
            },
        }
    }
    return floor;
}

fn part2() !usize {
    var floor: i32 = 0;
    for (input, 1..) |c, i| {
        switch (c) {
            '(' => {
                floor += 1;
            },
            ')' => {
                floor -= 1;
            },
            else => {
                return error.InvalidInput;
            },
        }
        if (floor < 0) {
            return i;
        }
    }
    return error.NeverReachedBasement;
}

pub fn main() !void {
    std.log.info("answer to part 1 is {}", .{try part1()});
    std.log.info("answer to part 2 is {}", .{try part2()});
}
