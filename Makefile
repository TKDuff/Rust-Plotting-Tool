cdirs = $(dir $(wildcard */Cargo.toml))

all: strategy_pattern/
all: writer/

allall: $(cdirs)

$(cdirs):
	cd $@ && cargo build

.PHONY: $(cdirs)
