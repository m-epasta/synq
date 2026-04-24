default: ci

clippy:
  cargo clippy -- -D warnings

ci:
  clippy

test:
  cargo test -- --no-capture

ast path="tests/files/frame.synq":
  v run print_ast.vsh {{path}}
