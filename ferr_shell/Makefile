
all: parser lexer
	cargo build --release
	cp target/x86_64-ferros/release/ferr_shell ../../FerrOS/filesystem/ferr_shell


parser: src/compute/parser.rs
	../../Samenhir/samenhir parser.sam -l rust
	mv parser.rs src/compute/parser.rs

lexer: src/compute/lexer.rs
	../../Ferrolex/ferrolex lexer.frl -l rust -o src/compute/lexer.rs