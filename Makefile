PREFIX = /usr
CARGO = cargo

install:
	$(CARGO) install --path .

uninstall:
	$(CARGO) uninstall -p nyafetch

install_global: build
	cp target/release/nyafetch ${PREFIX}/bin/

uninstall_global:
	rm ${PREFIX}/bin/nyafetch

build:
	$(CARGO) build --release

clean:
	$(CARGO) clean
