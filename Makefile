install:
	pip install --upgrade pip &&\
	pip install -r ./data/requirements.txt
	@echo "Updating rust toolchain"
	rustup update stable
	rustup default stable 

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

build:
	cargo build

run:
	cargo run

release:
	cargo build --release

all: format lint test run
# generate_and_push:
# 	# Create the markdown file 
# 	python test_main.py  # Replace with the actual command to generate the markdown

# 	# Add, commit, and push the generated files to GitHub
# 	@if [ -n "$$(git status --porcelain)" ]; then \
# 		git config --local user.email "action@github.com"; \
# 		git config --local user.name "GitHub Action"; \
# 		git add .; \
# 		git commit -m "Add SQL log"; \
# 		git push; \
# 	else \
# 		echo "No changes to commit. Skipping commit and push."; \
# 	fi