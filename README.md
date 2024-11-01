# Minimal break example for Rust-Analyzer and Bazel


To reproduce the error,

## Generate Rust-analyzer config

```sh
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
bazel run --config=genra

```

## Edit

1. open the file ring/ring.rs.
2. Start typing and use a non-existing variable.
3. Notice Rust-analyzer doesn't detect the error automatically.

## Work-around

Errors have to be manually checked with 

```sh
bazel build --config=clippy //...
```

There are also some tests:

```sh
bazel test --test_output=all //...
```


## Does not work

What we need is the equivalent of 

```sh
cargo check --message-format=json
```

This does not work

```sh
bazel run @rules_rust//tools/upstream_wrapper:cargo check
```