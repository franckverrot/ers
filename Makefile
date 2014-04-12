.PHONY: clean lib all test

test: clean lib ers
	rustc --test test/template_test.rs -o build/template_test -L lib
	rustc --test test/parser_test.rs -o build/parser_test -L lib
	rustc --test test/scanner_test.rs -o build/scanner_test -L lib
	./build/template_test
	./build/parser_test
	./build/scanner_test
	rustc --crate-type lib build/foo.rs --out-dir build && rustc --test test/integration/launcher.rs -L build -o build/it_tests
	./build/it_tests
	rustdoc --test src/ers/lib.rs -L lib

LIBNAME := $(shell rustc --crate-file-name src/ers/lib.rs)

all: lib test

lib: lib/$(LIBNAME)

lib/$(LIBNAME): src/ers/lib.rs
	@mkdir -p lib build
	rustc -O --crate-type lib --out-dir lib $<

clean:
	rm -f build/*
	rm -f lib/*
	rm -f bin/ers

license:
	`open http://www.gnu.org/licenses/gpl.txt`

ers:
	rustc bin/ers.rs -L lib
