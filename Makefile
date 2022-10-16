
SHELL := /bin/sh -ec
TOOLS_ROOT := .
TOOLS_BIN := $(TOOLS_ROOT)/bin
GALETTE := $(TOOLS_BIN)/galette

.PHONY: generate
generate: $(GALETTE)
	for file in *.pld ; do \
	  $(GALETTE) --nofuse $${file} || { \
	  	rc=$$? ; \
		echo error in $${file} ; \
		exit $${rc} ; \
	  }; \
	done


# not phony
# depends on Makefile because we put the version in the CLI invocation
$(GALETTE): Makefile
	cargo install --root $(TOOLS_ROOT) galette --git https://github.com/rustbox/galette.git --rev 2ba00658608c4e4d1fd51d903a7799dcabe2f9cc
	touch $(GALETTE) # sometimes cargo no-ops, so keep Make in sync
