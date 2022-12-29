EXE=./target/release/brasse
prefix=/usr/local
bindir=$(prefix)/bin

.PHONY: all
all: $(EXE)

$(EXE): ./src/*.rs ./src/**/*.rs
	cargo build --release

.PHONY: install
install: $(EXE)
	install -m755 $< $(DESTDIR)$(bindir)/brasse

.PHONY: clean
clean:
	rm -rf $(EXE)
