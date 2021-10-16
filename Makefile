.POSIX:

VERSION=0.1.0
PREFIX = /usr/local
MANPREFIX = $(PREFIX)/share/man

all: pr

pr:
	cargo build --release -vv
	cp target/release/pr ./

clean:
	cargo clean -vv
	rm pr

install: pr
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/release/tdr $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/tdr

uninstall:
	rm -r $(DESTDIR)$(PREFIX)/bin/tdr

.PHONY: all clean install uninstall
