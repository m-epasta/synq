#!/usr/bin/env -S v

arg := args[1]
if arg == '-d' {
	execute('cargo publish -p synq-codec --dry-run')
	execute('cargo publish -p synq-net --dry-run')
	execute('cargo publish -p synq-core --dry-run')
	execute('cargo publish -p synq --dry-run')
	exit(0)
} else if arg == '--all' {
	execute('cargo publish -p synq-codec')
	execute('cargo publish -p synq-net')
	execute('cargo publish -p synq-core')
	execute('cargo publish -p synq')
	exit(0)
} else {
	eprintln('arg should be else -d (dry run) or --all(all crates)')
	eprintln('instead, got: ${arg}')
	exit(-1)
}
