EXE=./brasse
COMP_DIR=./autocomplete
prefix=/usr/local
bindir=$(prefix)/bin
datadir=$(prefix)/share

.PHONY: all
all: $(EXE) completions

$(EXE): main.go ./**/*.go
	go build .

.PHONY: completions
completions: $(COMP_DIR)/brasse.bash $(COMP_DIR)/brasse.fish $(COMP_DIR)/_brasse

mk_autocomp_dir=@mkdir -p $(COMP_DIR)

autocomplete/brasse.bash: $(EXE)
	$(mk_autocomp_dir)
	$(EXE) completion bash > $@

autocomplete/brasse.fish: $(EXE)
	$(mk_autocomp_dir)
	$(EXE) completion fish > $@

autocomplete/_brasse: $(EXE)
	$(mk_autocomp_dir)
	$(EXE) completion zsh > $@

.PHONY: install
install: $(EXE)
	install -m755 $< $(DESTDIR)$(bindir)/brasse
	install -m644 $(COMP_DIR)/brasse.bash $(DESTDIR)$(datadir)/bash-completion/completions/brasse
	install -m644 $(COMP_DIR)/brasse.fish $(DESTDIR)$(datadir)/fish/vendor_completions.d/brasse.fish
	install -m644 $(COMP_DIR)/_brasse $(DESTDIR)$(datadir)/zsh/site-functions/_brasse

.PHONY: clean
clean:
	rm -rf $(EXE) $(COMP_DIR)
