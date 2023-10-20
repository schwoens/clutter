const std = @import("std");
const errors = @import("errors.zig");
const InvalidArgumentError = errors.InvalidArgumentError;

pub fn add(args: [][]const u8) !void {
    if (args.len < 1) return InvalidArgumentError.TooFewArguments;
    const description = args[0];
    // default priority
    var priority: u8 = 3;
    if (args.len > 1) {
        priority = std.fmt.parseInt(u8, args[1], 10) catch return InvalidArgumentError.InvalidPriority;
    }

    std.debug.print("Add todo: {s}, Priority: {}\n", .{ description, priority });

    const modifier = switch (priority) {
        3 => "",
        2 => "*",
        1 => "**",
        else => return InvalidArgumentError.InvalidPriority,
    };

    var md_string: []u8 = "- [ ] " ++ modifier ++ description ++ modifier;
    std.debug.print(md_string);
}

pub fn edit() !void {
    std.debug.print("Edit todos\n", .{});
}

pub fn complete(args: [][]const u8) !void {
    _ = args;
    return;
}

pub fn delete(args: [][]const u8) !void {
    _ = args;
    return;
}
