all: release

fmt:
	cargo fmt

doc: fmt
	cargo doc --document-private-items --open

clean: fmt
	cargo clean
	rm -f lisp

release: fmt
	cargo build --release
	cp -f target/x86_64-ferros/release/lisp lisp
	cp -f lisp ../../FerrOS/src/user_program/lisp

.PHONY: all fmt doc clean
