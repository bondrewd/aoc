const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const file = try std.fs.cwd().openFile("data.txt", .{});
    const data = try file.readToEndAlloc(allocator, 1 << 13);

    const ans1 = try last_floor(data);
    const ans2 = try first_cross(data);

    std.debug.print("Ans1: {d}\n", .{ans1});
    std.debug.print("Ans2: {d}\n", .{ans2.?});
}

fn last_floor(buf: []u8) !i32 {
    var floor: i32 = 0;
    for (buf) |c| switch (c) {
        '(' => floor += 1,
        ')' => floor -= 1,
        else => return error.UnknownCharacter,
    };

    return floor;
}

fn first_cross(buf: []u8) !?i32 {
    var floor: i32 = 0;
    for (buf, 1..) |c, i| {
        switch (c) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => return error.UnknownCharacter,
        }

        if (floor == -1) return @intCast(i);
    }

    return null;
}
