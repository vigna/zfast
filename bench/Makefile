# Builds the drivers of the comparative benchmarks. bench/run.py invokes
# these targets on demand.
#
#   make init       fetches the competitors (git submodules)
#   make            builds all drivers into build/

CC ?= cc
CFLAGS ?= -O3 -DNDEBUG -march=native -flto -fno-stack-protector -std=gnu18 -pthread -w
WORMHOLE = wormhole

all: build/zfast-bench build/wh_bench

init:
	git submodule update --init --recursive

build:
	mkdir -p build

# The driver for the structures of the zfast crate and for the Rust baselines
build/zfast-bench: build FORCE
	cd zfast-bench && cargo build --release
	cp zfast-bench/target/release/zfast-bench $@

# The driver for Wormhole, compiled with the flags of Wormhole's own Makefile
build/wh_bench: build wh_bench.c $(WORMHOLE)/lib.c $(WORMHOLE)/kv.c $(WORMHOLE)/wh.c
	$(CC) $(CFLAGS) -I$(WORMHOLE) -o $@ wh_bench.c $(WORMHOLE)/lib.c $(WORMHOLE)/kv.c $(WORMHOLE)/wh.c -lm

clean:
	rm -rf build

FORCE:

.PHONY: all init clean FORCE
