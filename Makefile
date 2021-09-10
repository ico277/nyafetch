PREFIX = /usr
CARGO = cargo

install: copy_art
	$(CARGO) install --path . --root $(PREFIX)

uninstall: remove_art
	$(CARGO) uninstall -p nyafetch --root $(PREFIX)

build:
	$(CARGO) build --release

run: copy_art
	$(CARGO) run --release

clean:
	$(CARGO) clean

copy_art:
	mkdir -p /usr/local/share/nyafetch/
	cp ./distro_art/* /usr/local/share/nyafetch/

remove_art:
	rm -rf /usr/local/share/nyafetch/
