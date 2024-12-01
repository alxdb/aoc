const std = @import("std");
const input = @embedFile("input");

const Lists = struct {
    left: std.ArrayList(u32),
    right: std.ArrayList(u32),

    pub fn init(allocator: std.mem.Allocator) Lists {
        return Lists{
            .left = std.ArrayList(u32).init(allocator),
            .right = std.ArrayList(u32).init(allocator),
        };
    }
};

fn parseLists(allocator: std.mem.Allocator) !Lists {
    var lists = Lists.init(allocator);
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var entries = std.mem.tokenizeScalar(u8, line, ' ');
        try lists.left.append(try std.fmt.parseInt(u32, entries.next().?, 0));
        try lists.right.append(try std.fmt.parseInt(u32, entries.next().?, 0));
    }
    return lists;
}

fn part1(allocator: std.mem.Allocator) !u32 {
    const lists = try parseLists(allocator);
    std.mem.sort(u32, lists.left.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, lists.right.items, {}, comptime std.sort.asc(u32));
    var total_distance: u32 = 0;
    for (0..lists.left.items.len) |i| {
        var distance: i32 = @intCast(lists.left.items[i]);
        distance -= @intCast(lists.right.items[i]);
        total_distance += @abs(distance);
    }
    return total_distance;
}

fn part2(allocator: std.mem.Allocator) !u32 {
    const lists = try parseLists(allocator);

    var occurence_map = std.AutoHashMap(u32, u32).init(allocator);
    for (lists.right.items) |item| {
        const result = try occurence_map.getOrPut(item);
        if (result.found_existing) {
            result.value_ptr.* += 1;
        } else {
            result.value_ptr.* = 1;
        }
    }
    var similarity_score: u32 = 0;
    for (lists.left.items) |item| {
        const occurrence = occurence_map.get(item);
        if (occurrence != null) {
            similarity_score += item * occurrence.?;
        }
    }
    return similarity_score;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    std.log.info("answer to part 1 is {}", .{try part1(allocator)});
    std.log.info("answer to part 2 is {}", .{try part2(allocator)});
}

test "solutions" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    try std.testing.expectEqual(2264607, try part1(allocator));
    try std.testing.expectEqual(19457120, try part2(allocator));
}
