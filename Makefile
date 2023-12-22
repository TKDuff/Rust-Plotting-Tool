cdirs = $(dir $(wildcard */Cargo.toml))

all: $(cdirs)

$(cdirs):
	cd $@ && cargo build

.PHONY: $(cdirs)
