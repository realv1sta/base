install:
	cargo install --path . --root ~/.local
	ln -sf ~/.local/bin/base ~/.local/bin/bs
