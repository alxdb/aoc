const std = @import("std");

pub const ProblemId = struct {
    value: u32,

    pub fn init(year: u16, day: u16) ProblemId {
        return .{ .value = @as(u32, @intCast(year)) << 16 | @as(u32, @intCast(day)) };
    }
};

pub fn solveProblem(id: ProblemId, input: []const u8, output: *std.Io.Writer) !void {
    switch (id.value) {
        ProblemId.init(2015, 1).value => try notQuiteLisp(input, output),
        else => return error.NotImplemented,
    }
}

fn notQuiteLisp(input: []const u8, output: *std.Io.Writer) !void {
    var floor: i32 = 0;
    var basement_position: ?usize = null;
    for (input, 0..) |val, idx| {
        switch (val) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => return error.InvalidInput,
        }
        if (basement_position == null and floor == -1) {
            basement_position = idx;
        }
    }
    try output.print("{}\n{}", .{ floor, basement_position orelse {
        return error.InvalidInput;
    } });
    try output.flush();
}
