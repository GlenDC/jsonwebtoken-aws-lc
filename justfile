fmt:
	cargo fmt --all

sort:
	cargo sort --grouped

lint: fmt sort

check:
	cargo check --all --all-targets

clippy:
	cargo clippy --all --all-targets

clippy-fix:
	cargo clippy --fix

typos:
	typos -w

doc:
	RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc --features= --no-deps

hack:
	cargo hack check --each-feature --no-dev-deps

test:
	cargo test

test-examples:
	cargo test --examples

qa: lint check clippy doc test test-examples
