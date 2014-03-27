test: clean lib
	rustc --test test/template_test.rs -o build/template_test -L lib
	rustc --test test/parser_test.rs -o build/parser_test -L lib
	rustc --test test/scanner_test.rs -o build/scanner_test -L lib
	./build/template_test
	./build/parser_test
	./build/scanner_test
	rustc --lib build/foo.rs && rustc --test test/integration/launcher.rs -L build -o build/it_tests
	./build/it_tests

.PHONY: clean lib all test

LIBNAME := $(shell rustc --crate-file-name src/ers/lib.rs)

all: lib test

lib: lib/$(LIBNAME)

lib/$(LIBNAME): src/ers/lib.rs
	@mkdir -p lib
	rustc -O --lib --out-dir lib $<

clean:
	rm -f build/*
	rm -f lib/*
