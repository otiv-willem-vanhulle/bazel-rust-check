# No Cargo check in Bazel

This repository reproduces an issue that I have with Rust-Analyzer. Rust-Analyzer is a language server for Rust.

## Identifiers and references

The first step is to generate a Rust-analyze config in Bazel

In general you can do that with

```sh
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

This makes it possible to click on identifiers.

## Cargo check

But then I would like to use Cargo check. Since there is not `cargo.toml` file in a Bazel project we have to use an alternative command.

In a Cargo project, the command is:

```sh
cargo check --message-format=json
```

We can adjust this command inside the JSON settings of VS Code:

```json
{
  "rust-analyzer.check.overrideCommand": [
    "cargo",
    "check",
    "--message-format=json"
  ]
}
```

But we can't use Cargo. So what should we do?

The following command fails because it detects there is not Cargo.toml file.

```sh
bazel run @rules_rust//tools/upstream_wrapper:cargo check
```

Maybe we can do something similar to the command

```
bazel build --output_groups=+clippy_checks //...
```

which checks with Clippy but add ` --message-format=json` somewhere?

With or without this modification to the JSON settings, nothing happens if I hit save in my editor. It doesn't matter which editor. It also happens in editors like Zed, Lapce, Helix and so on.

Too see this, you can 

1. open the file ring/ring.rs.
2. Start typing and use a non-existing variable.
3. Notice Rust-analyzer doesn't detect the error automatically.

This problem also occurs when you start with a Cargo project and remove the cargo.toml file. From the moment that file is removed, also all the check functionality stops working.

## Work-arounds

The only way to work around this to run clippy manually with 

```sh
bazel build --config=clippy //...
```

You can also run tests and see that they fail.
```sh
bazel test --test_output=all //...
```

