.PHONY: all
all: bin/count bin/align bin/distinct bin/sample bin/hist bin/rand bin/stats \
	bin/prefix bin/prefix bin/lorem bin/pace bin/dbg bin/pick bin/ascii \
	bin/groupby bin/cumsum bin/tld bin/ends bin/jwtdec bin/base64url \
	bin/urldecode bin/urlencode bin/rule bin/llen bin/ddiff bin/scatter \
	bin/gen-strs bin/news bin/hex2num bin/beep bin/map-range bin/bgrep \
	bin/merge bin/http bin/xvars

bin/%: %/src/main.rs
	cd $* && cargo +nightly build -r && cp target/release/$* ../bin
