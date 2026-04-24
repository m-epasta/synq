#!/usr/bin/env -S v

mut path := args[1]

if !path.starts_with('tests/files/') && !path.ends_with('.synq') {
	path = 'tests/files/${path}.synq'
} else if !path.starts_with('tests/files') {
	path = 'tests/files/${path}'
}

system('cargo build -q --release')
cmd := "cargo run -q --bin print_ast -- \"${path}\""
println(cmd)
res := execute(cmd)
if res.exit_code == 0 {
	println(res.output)
} else {
	eprintln(res.output)
}
