PREFIX?=/usr/
ID=io.github.ShareXin

test:
	RUST_BACKTRACE=1 cargo test

run:
	RUST_BACKTRACE=1 cargo run -- --no-fork

install: install-resources
	cargo install --path . --force --root $(DESTDIR)$(PREFIX)

install-resources:
	mkdir -p $(DESTDIR)$(PREFIX)/share/applications/
	cp data/$(ID).desktop.in $(DESTDIR)$(PREFIX)/share/applications/$(ID).desktop
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/16x16/apps/
	cp data/icons/hicolor/16x16/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/16x16/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/32x32/apps/
	cp data/icons/hicolor/32x32/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/32x32/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/64x64/apps/
	cp data/icons/hicolor/64x64/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/64x64/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/
	cp data/icons/hicolor/128x128/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/256x256/apps/
	cp data/icons/hicolor/256x256/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/256x256/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/512x512/apps/
	cp data/icons/hicolor/512x512/apps/$(ID).png $(DESTDIR)$(PREFIX)/share/icons/hicolor/512x512/apps/$(ID).png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
	cp data/icons/hicolor/scalable/apps/$(ID).svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
