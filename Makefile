DESTDIR =
PREFIX  = /usr

override define compdir
ifndef $(1)
$(1) := $$(or $$(shell pkg-config --variable=completionsdir $(2) 2>/dev/null),$(3))
endif
endef

$(eval $(call compdir,BASHDIR,bash-completion,$(PREFIX)/etc/bash_completion.d))
ZSHDIR  = /usr/share/zsh/vendor-completions

all: target/release/sharexin
build: target/release/sharexin

target/release/sharexin:
	cargo build --release

install: install-sharexin install-man

install-sharexin: target/release/sharexin
	install -m755 -- target/release/sharexin "$(DESTDIR)$(PREFIX)/bin/"

install-man:
	install -dm755 -- "$(DESTDIR)$(PREFIX)/bin/" "$(DESTDIR)$(PREFIX)/share/man/man1/"
	install -m644  -- man/sharexin.1 "$(DESTDIR)$(PREFIX)/share/man/man1/"

install-bash-completions:
	install -m644 -- shell/completions.bash "$(DESTDIR)$(BASHDIR)/sharexin"

install-zsh-completions:
	install -m644 -- shell/completions.zsh "$(DESTDIR)$(ZSHDIR)/_sharexin"

uninstall:
	-rm -f -- "$(DESTDIR)$(PREFIX)/share/man/man1/sharexin.1"
	-rm -f -- "$(DESTDIR)$(PREFIX)/bin/sharexin"
	-rm -f -- "$(DESTDIR)$(BASHDIR)/sharexin"
	-rm -f -- "$(DESTDIR)$(ZSHDIR)/_sharexin"

clean:
	cargo clean

preview-man:
	man man/sharexin.1

help:
	@echo 'Available make targets:'
	@echo '  all         - build sharexin (default)'
	@echo '  build       - build sharexin'
	@echo '  clean       - run `cargo clean`'
	@echo '  install     - build and install sharexin and manpage'
	@echo '  install-sharexin - build and install sharexin'
	@echo '  install-man - install the manpage'
	@echo '  uninstall   - uninstall zsh, manpage, and completions'
	@echo '  preview-man - preview the manpage without installing'
	@echo '  help        - print this help'
	@echo
	@echo '  install-bash-completions - install bash completions into $$BASHDIR'
	@echo '  install-zsh-completions  - install zsh completions into $$ZSHDIR'
	@echo
	@echo 'Variables:'
	@echo '  DESTDIR  - A path that'\''s prepended to installation paths (default: "")'
	@echo '  PREFIX   - The installation prefix for everything except zsh completions (default: /usr/local)'
	@echo '  BASHDIR  - The directory to install bash completions in (default: $$PREFIX/etc/bash_completion.d)'
	@echo '  ZSHDIR   - The directory to install zsh completions in (default: /usr/share/zsh/vendor-completions)'

.PHONY: all build target/release/sharexin install-sharexin install-man preview-man \
	install-bash-completions install-zsh-completions \
	clean uninstall help
