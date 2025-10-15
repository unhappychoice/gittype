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
