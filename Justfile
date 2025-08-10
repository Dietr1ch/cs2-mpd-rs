# Run the server
run:
	cargo run


# Test code
test:
	cargo nextest run


# Format all code
format: format_rs format_nix

# Format Rust code
format_rs:
	cargo fmt

# Format Nix code
format_nix:
	fd -e 'nix' | parallel  --  nixfmt '{}'


# Lint all code
lint: lint_rs lint_toml

# Lint Rust code
lint_rs:
	cargo clippy --all-targets --all-features

# Lint Cargo.toml file
lint_toml:
	cargo-toml-lint --sort-dependencies 'strict' Cargo.toml

# Apply the linter's suggested fixes
lint_and_fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty


# Generate documentation
doc:
	cargo doc


# Run bacon jobs on new Zellij panes
zellij_bacon:
	zellij run \
	  --close-on-exit \
	  --name 'bacon format' \
	  -- \
	  direnv exec . \
	    bacon \
	      fmt

	zellij run \
	  --close-on-exit \
	  --name 'bacon clippy' \
	  -- \
	  direnv exec . \
	    bacon \
	      clippy-all

	zellij run \
	  --close-on-exit \
	  --name 'bacon nextest' \
	  -- \
	  direnv exec . \
	    bacon \
	      nextest

	zellij run \
	  --close-on-exit \
	  --name 'bacon doc' \
	  -- \
	  direnv exec . \
	    bacon \
	      doc

# Run server on a new Zellij pane
zellij_run:
	zellij run \
	  --close-on-exit \
	  --name 'bacon run-long' \
	  -- \
	  direnv exec . \
	    bacon \
	      run-long
