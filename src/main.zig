const std = @import("std");

const InvalidArgumentsError = error{
    NoArguments,
};

const AocId = struct { y: u8, d: u8, p: u8 };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const alloc = arena.allocator();

    var args = try std.process.argsWithAllocator(alloc);
    defer args.deinit();

    if (!args.skip()) {
        std.debug.assert(false);
    }
}
