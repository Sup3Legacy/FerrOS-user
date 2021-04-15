all: release

fmt:
	cargo fmt

clean: fmt
	cargo clean
	rm -f test_io

release: fmt
	cargo build --release
	cp -f target/x86_64-ferros/release/test_io test_io
	cp -f test_io ../../FerrOS/src/user_program/test_io

.PHONY: all fmt clean
