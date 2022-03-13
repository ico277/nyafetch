PREFIX = /usr
CARGO = cargo

build:
	$(CARGO) build --release

install:
	cp target/release/nyafetch $(PREFIX)/bin/nyafetch

uninstall:
	rm $(PREFIX)/bin/nyafetch

clean:
	$(CARGO) clean
