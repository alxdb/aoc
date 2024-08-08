const std = @import("std");
const input = @embedFile("input");

fn updateFloor(c: u8, floor: *i32) !void {
    switch (c) {
        '(' => floor.* += 1,
        ')' => floor.* -= 1,
        else => return error.InvalidInput,
    }
}

fn part1() !i32 {
    var floor: i32 = 0;
    for (input) |c| {
        try updateFloor(c, &floor);
    }
    return floor;
}

fn part2() !usize {
    var floor: i32 = 0;
    for (input, 1..) |c, i| {
        try updateFloor(c, &floor);
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

test "solutions" {
    try std.testing.expectEqual(74, try part1());
    try std.testing.expectEqual(1795, try part2());
}
