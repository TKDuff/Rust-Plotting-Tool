all: basic_threading
all: mpscTest
all: program1
all: program2

RUSTC=rustc

%: %.rs; $(RUSTC) $<

test1:
	yes | awk '{print NR, sqrt(NR)}' | ./program2
