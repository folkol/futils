.PHONY: all
all: bin/count bin/align bin/distinct bin/sample bin/hist bin/rand bin/stats bin/prefix bin/prefix bin/lorem bin/pace bin/dbg bin/pick bin/ascii bin/groupby bin/cumsum bin/tld

bin/%: %/src/main.rs
	cd $* && cargo build -r && cp target/release/$* ../bin
