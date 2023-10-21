const std = @import("std");
const commands = @import("commands.zig");
const errors = @import("errors.zig");
const InvalidArgumentError = errors.InvalidArgumentError;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    try handleArguments(args);

    std.debug.print("args: {s}\n", .{args});
    //
    // const stdout_file = std.io.getStdOut().writer();
    // var bw = std.io.bufferedWriter(stdout_file);
    // const stdout = bw.writer();
    //
    // try stdout.print("Run `zig build test` to run the tests.\n", .{});
    //
    // try bw.flush(); // don't forget to flush!
}

const Command = enum {
    add,
    edit,
    complete,
    delete,
};

fn handleArguments(args: [][:0]u8) !void {
    if (args.len < 2) {
        return InvalidArgumentError.TooFewArguments;
    }

    const command = std.meta.stringToEnum(Command, args[1]) orelse return InvalidArgumentError.InvalidCommand;
    const command_args = args[2..];

    try switch (command) {
        .add => commands.add(command_args),
        .edit => commands.edit(),
        .complete => commands.complete(command_args),
        .delete => commands.delete(command_args),
    };
}

const Config = struct {
    data_path: []const u8,
};

fn readConfigFile() Config {

    var config: Config = .{
        .data_path = std.fs.cwd()
    }

    var config_file = std.fs.openFileAbsolute("$XDG_CONFIG_HOME/clutter/clutter.conf", .{ .read = true });
    if (config_file) |file| {
        defer file.close();


        var buf_reader = std.io.bufferedReader(file.reader());
        var in_stream = buf_reader.reader();

        var buf: [128]u8 = undefined;
        while (try in_stream.readUnitlDelimiterOrEof(&buf, "\n")) |line| {

        }
    }
}
