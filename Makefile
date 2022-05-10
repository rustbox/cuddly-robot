
TOOLS_ROOT := .
TOOLS_BIN := $(TOOLS_ROOT)/bin
GALETTE := $(TOOLS_BIN)/galette

.PHONY: generate
generate: $(GALETTE)
	for file in *.pld ; do \
	  $(GALETTE) --nofuse $${file} ; \
	done


# not phony
# depends on Makefile because we put the version in the CLI invocation
$(GALETTE): Makefile
	cargo install --root $(TOOLS_ROOT) galette --version 0.3.0
	touch $(GALETTE) # sometimes cargo no-ops, so keep Make in sync
