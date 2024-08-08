const std = @import("std");

const Steps = struct {
    compile: *std.Build.Step.Compile,
    compile_test: *std.Build.Step.Compile,
    run_test: *std.Build.Step.Run,
};

const StepOptions = struct {
    name: []const u8,
    root_source_file: std.Build.LazyPath,
    target: std.Build.ResolvedTarget,
    optimize: std.builtin.OptimizeMode,
};

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const root = addCompileSteps(std.Build.StaticLibraryOptions, b, .{
        .name = "aoc",
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const fetch_input = addCompileSteps(std.Build.ExecutableOptions, b, .{
        .name = "fetch_input",
        .root_source_file = b.path("src/fetch_input.zig"),
        .target = target,
        .optimize = optimize,
    });
    _ = try addRunStep(b, fetch_input, "fetch_input");

    var solution_tests = std.ArrayList(*std.Build.Step.Run).init(b.allocator);
    {
        const solutions_dir_name = "solutions";
        const solutions_dir = b.path(solutions_dir_name).getPath(b);
        var dir = try std.fs.cwd().openDir(
            solutions_dir,
            .{ .iterate = true },
        );
        defer dir.close();

        var dir_iter = dir.iterate();
        while (try dir_iter.next()) |entry| {
            const solution_name = std.fs.path.stem(entry.name);
            const solution_path = try std.fs.path.join(
                b.allocator,
                &.{ solutions_dir_name, entry.name },
            );

            const compile_steps = addCompileSteps(
                std.Build.ExecutableOptions,
                b,
                .{
                    .name = solution_name,
                    .root_source_file = b.path(solution_path),
                    .target = target,
                    .optimize = optimize,
                },
            );
            compile_steps.compile.linkLibrary(root.compile);

            _ = try addRunStep(b, compile_steps, solution_name);
            try solution_tests.append(compile_steps.run_test);

            const fetch_input_step = b.addRunArtifact(fetch_input.compile);
            fetch_input_step.addArg(solution_name);
            const solution_input = fetch_input_step.captureStdOut();
            compile_steps.compile.root_module.addAnonymousImport(
                "input",
                .{ .root_source_file = solution_input },
            );
            compile_steps.compile_test.root_module.addAnonymousImport(
                "input",
                .{ .root_source_file = solution_input },
            );
        }
    }

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&root.run_test.step);
    for (solution_tests.items) |solution_test| {
        test_step.dependOn(&solution_test.step);
    }
}

fn addCompileSteps(
    comptime T: type,
    b: *std.Build,
    options: StepOptions,
) Steps {
    const compile = switch (T) {
        std.Build.ExecutableOptions => b.addExecutable(.{
            .name = options.name,
            .root_source_file = options.root_source_file,
            .target = options.target,
            .optimize = options.optimize,
        }),
        std.Build.StaticLibraryOptions => b.addStaticLibrary(.{
            .name = options.name,
            .root_source_file = options.root_source_file,
            .target = options.target,
            .optimize = options.optimize,
        }),
        else => undefined,
    };
    b.installArtifact(compile);

    const compile_test = b.addTest(.{
        .root_source_file = options.root_source_file,
        .target = options.target,
        .optimize = options.optimize,
    });
    const run_test = b.addRunArtifact(compile_test);

    return .{
        .compile = compile,
        .compile_test = compile_test,
        .run_test = run_test,
    };
}

fn addRunStep(
    b: *std.Build,
    steps: Steps,
    name: []const u8,
) !*std.Build.Step.Run {
    const run_cmd = b.addRunArtifact(steps.compile);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step(
        name,
        try std.fmt.allocPrint(b.allocator, "Run {s}", .{name}),
    );
    run_step.dependOn(&run_cmd.step);
    return run_cmd;
}
