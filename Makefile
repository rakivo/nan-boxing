libnan.rlib: nan.rs typemod.rs
	rustc --crate-type=rlib --edition=2021 -o $@ $<
