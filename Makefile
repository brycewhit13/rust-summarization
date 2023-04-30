rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-release:
	cargo build --release

docker-push:
	docker build -t rust-summarization .
	docker tag rust-summarization:latest brycewhit13/rust-summarization:latest
	docker push brycewhit13/rust-summarization:latest

docker-pull:
	docker pull brycewhit13/rust-summarization

docker-run:
	docker run -d -p 8080:8080 brycewhit13/rust-summarization:latest