const std = @import("std");

const input = @embedFile("input");

const Box = struct {
    l: u32,
    w: u32,
    h: u32,

    pub fn parse(line: []const u8) !Box {
        var iter = std.mem.splitSequence(u8, line, "x");
        return .{
            .l = try std.fmt.parseInt(u32, iter.next() orelse return error.InvalidInput, 0),
            .w = try std.fmt.parseInt(u32, iter.next() orelse return error.InvalidInput, 0),
            .h = try std.fmt.parseInt(u32, iter.next() orelse return error.InvalidInput, 0),
        };
    }
};

fn part1() !u32 {
    var input_iter = std.mem.tokenizeSequence(u8, input, "\n");
    var wrapping_paper: u32 = 0;

    while (input_iter.next()) |line| {
        const box = try Box.parse(line);
        const surface_areas = [_]u32{ box.l * box.w, box.w * box.h, box.h * box.l };

        var min_area: u32 = std.math.maxInt(u32);
        for (surface_areas) |area| {
            wrapping_paper += 2 * area;
            if (area < min_area) {
                min_area = area;
            }
        }
        wrapping_paper += min_area;
    }

    return wrapping_paper;
}

fn part2() !u32 {
    var input_iter = std.mem.tokenizeSequence(u8, input, "\n");
    var ribbon_length: u32 = 0;

    while (input_iter.next()) |line| {
        const box = try Box.parse(line);
        const volume = box.l * box.w * box.h;
        ribbon_length += volume;

        var sides = [_]u32{ box.l, box.w, box.h };
        std.sort.block(u32, &sides, {}, comptime std.sort.asc(u32));
        ribbon_length += 2 * sides[0];
        ribbon_length += 2 * sides[1];
    }

    return ribbon_length;
}

pub fn main() !void {
    std.log.info("answer to part 1 is {}", .{try part1()});
    std.log.info("answer to part 2 is {}", .{try part2()});
}
