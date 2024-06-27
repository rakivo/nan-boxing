all: build build/libnan.rlib build/test

build:
	mkdir -p build

build/libnan.rlib: src/nan.rs
	rustc --crate-type=rlib --edition=2021 -o $@ $<

build/test: src/test.rs build/libnan.rlib
	rustc --extern nan=build/libnan.rlib -o $@ $<
