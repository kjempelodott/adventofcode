SUBDIRS=day5

all: $(basename $(wildcard *.rs)) $(SUBDIRS)

$(SUBDIRS):
	@cd $@ && cargo build --release && \
	cp target/release/$@ ../$@.bin && cd .. && \
	strip $@.bin

%: %.rs
	rustc -O $<
	strip $@

.PHONY: $(SUBDIRS)
