
all: parser lexer
	cargo build --release
	cp target/x86_64-ferros/release/ferr_shell ../../FerrOS/filesystem/ferr_shell


parser:
	../../Samenhir/samenhir parser.sam -l rust -o src/compute/parser.rs

lexer:
	../../Ferrolex/ferrolex lexer.frl -l rust -o src/compute/lexer.rs