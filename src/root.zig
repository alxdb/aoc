const std = @import("std");

pub const ProblemId = struct {
    value: u32,

    pub fn init(year: u16, day: u16) ProblemId {
        return .{ .value = @as(u32, @intCast(year)) << 16 | @as(u32, @intCast(day)) };
    }
};

pub fn solveProblem(id: ProblemId, input: *std.Io.Reader, output: *std.Io.Writer) !void {
    switch (id.value) {
        ProblemId.init(2015, 1).value => try notQuiteLisp(input, output),
        else => return error.NotImplemented,
    }
}

fn notQuiteLisp(input: *std.Io.Reader, output: *std.Io.Writer) !void {
    var floor: i32 = 0;
    var basement_found: bool = false;
    var basement_position: usize = 1;
    while (input.takeByte() catch null) |val| {
        switch (val) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => return error.InvalidInput,
        }
        if (!basement_found) {
            if (floor == -1) {
                basement_found = true;
            } else {
                basement_position += 1;
            }
        }
    }
    try output.print("{}\n{}", .{ floor, basement_position });
    try output.flush();
}
