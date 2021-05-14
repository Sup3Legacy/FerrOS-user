PARSER=../../Samenhir/samenhir
LEXER=../../Ferrolex/ferrolex

all: parser lexer release

parser:
	$(PARSER) parser.sam -l rust
	mv parser.rs src/compute/parser.rs

lexer:
	$(LEXER) lexer.frl -l rust
	mv lexer.rs src/compute/lexer.rs

fmt:
	cargo fmt

clean: fmt
	cargo clean
	rm -f ferr_shell

release: fmt
	cargo build --release
	cp -f target/x86_64-ferros/release/ferr_shell ../../FerrOS/filesystem/ferr_shell

.PHONY: all fmt clean
