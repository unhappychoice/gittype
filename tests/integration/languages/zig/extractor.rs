use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_zig_function_extraction,
    language: "zig",
    extension: "zig",
    source: r#"
pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) i32 {
    return a - b;
}
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_zig_struct_extraction,
    language: "zig",
    extension: "zig",
    source: r#"
const Point = struct {
    x: f32,
    y: f32,
};

pub const Config = struct {
    debug: bool,
    verbose: bool,
};
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Struct: 2,
    }
}

test_language_extractor! {
    name: test_zig_mixed_extraction,
    language: "zig",
    extension: "zig",
    source: r#"
const std = @import("std");

pub const Vector = struct {
    x: f32,
    y: f32,

    pub fn init(x: f32, y: f32) Vector {
        return Vector{ .x = x, .y = y };
    }

    pub fn length(self: Vector) f32 {
        return @sqrt(self.x * self.x + self.y * self.y);
    }
};

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, Zig!\n", .{});
}
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Struct: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_zig_enum_extraction,
    language: "zig",
    extension: "zig",
    source: r#"
const Color = enum {
    Red,
    Green,
    Blue,

    pub fn toRGB(self: Color) [3]u8 {
        return switch (self) {
            .Red => [_]u8{255, 0, 0},
            .Green => [_]u8{0, 255, 0},
            .Blue => [_]u8{0, 0, 255},
        };
    }
};

const Status = enum(u8) {
    Ok = 0,
    Error = 1,
    Pending = 2,
};
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Enum: 2,
        Function: 1,
        Conditional: 1,
    }
}

test_language_extractor! {
    name: test_zig_union_extraction,
    language: "zig",
    extension: "zig",
    source: r#"
const Value = union(enum) {
    int: i32,
    float: f64,
    boolean: bool,

    pub fn print(self: Value) void {
        switch (self) {
            .int => |val| std.debug.print("int: {}\n", .{val}),
            .float => |val| std.debug.print("float: {}\n", .{val}),
            .boolean => |val| std.debug.print("bool: {}\n", .{val}),
        }
    }
};

const Result = union {
    Ok: i32,
    Err: []const u8,
};
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Struct: 2,
        Function: 1,
        Conditional: 1,
    }
}

test_language_extractor! {
    name: test_zig_error_and_test_blocks,
    language: "zig",
    extension: "zig",
    source: r#"
const std = @import("std");

const FileError = error {
    AccessDenied,
    NotFound,
    OutOfMemory,
};

pub fn readFile(path: []const u8) FileError![]const u8 {
    if (path.len == 0) {
        return FileError.NotFound;
    }
    return "file contents";
}

test "basic addition" {
    const result = 2 + 2;
    try std.testing.expect(result == 4);
}

test "file reading" {
    const contents = try readFile("test.txt");
    try std.testing.expect(contents.len > 0);
}
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Function: 1,
        Conditional: 1,
    }
}
