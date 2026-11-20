# Builds the drivers of the comparative benchmarks. bench/run.py invokes
# these targets on demand.
#
#   make init       fetches the competitors (git submodules)
#   make            builds all drivers into build/
#
# The code of "B-Trees Are Back" (Mueller, Benson, Leis, SIGMOD 2025), which
# provides an adaptive B-tree and the version of HOT it is compared with, is not a submodule because its repository contains several hundred
# megabytes of experimental data: the parts we need (about 2 MB) are fetched on
# demand at a fixed commit into btree-cpp/. Note that the repository has no
# licence file: we do not redistribute any of its code.
#
# ART needs the headers and the library of TBB.

# Wormhole is compiled with the compiler and the flags of its own Makefile:
# with clang it is significantly faster than with gcc
WH_CC ?= clang
CFLAGS ?= -O3 -DNDEBUG -march=native -flto -fno-stack-protector -std=gnu18 -pthread -w
# A clang built from source performs link-time optimization only through lld
WH_LDFLAGS ?= $(shell $(WH_CC) -fuse-ld=lld -Wl,--version 2>/dev/null | grep -q LLD && echo -fuse-ld=lld)
WORMHOLE = wormhole

# As in the Makefile of btree-cpp, the B-tree and ART are compiled with
# clang++ (the code uses __builtin_assume) and the adapter of HOT with g++
BT_CXX ?= clang++
HOT_CXX ?= g++
ART_CXX ?= g++
CXXFLAGS ?= -O3 -DNDEBUG -march=native -std=c++17 -w
BTREECPP = btree-cpp
BTREECPP_URL = https://github.com/m-mueller678/btree-cpp
BTREECPP_COMMIT = 774b9fd9ce70222cee328ac5395b290d7822e587
# The configurations of btree-cpp we build (files in its named-configs)
BT_CONFIGS = adapt2 hot
# The size in bytes of the nodes of the B-tree: the default of btree-cpp is
# 4096, but on our data larger nodes are faster
BT_PAGE_SIZE ?= 16384
# (ART.cpp defines symbols needed by all configurations)
BT_SOURCES = $(addprefix $(BTREECPP)/btree/,anynode.cpp btree2020.cpp debug.cpp dense.cpp hash.cpp) \
	$(BTREECPP)/in-memory-structures/ART/ART.cpp
ART = ARTSynchronized
# Evaluated when the recipe runs, that is, after the code has been fetched
HOT_INCLUDES = $(shell find $(BTREECPP)/in-memory-structures/hot -type d -name include | sed 's/^/-I/')

all: build/zfast-bench build/wh_bench build/wh_bench-256 build/art_bench $(addprefix build/bt_bench-,$(BT_CONFIGS))

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
	$(WH_CC) $(CFLAGS) $(WH_LDFLAGS) -I$(WORMHOLE) -o $@ wh_bench.c $(WORMHOLE)/lib.c $(WORMHOLE)/kv.c $(WORMHOLE)/wh.c -lm

# Wormhole with 256 keys per leaf instead of 128 (a compile-time constant of
# wh.c), which is faster on most operations on our data
build/wh_bench-256: build wh_bench.c $(WORMHOLE)/lib.c $(WORMHOLE)/kv.c $(WORMHOLE)/wh.c
	rm -rf build/wormhole-256 && cp -r $(WORMHOLE) build/wormhole-256
	sed -i.bak 's|^#define WH_KPN ((128u))|#define WH_KPN ((256u))|' build/wormhole-256/wh.c
	grep -q '^#define WH_KPN ((256u))' build/wormhole-256/wh.c
	$(WH_CC) $(CFLAGS) $(WH_LDFLAGS) -Ibuild/wormhole-256 -o $@ wh_bench.c build/wormhole-256/lib.c build/wormhole-256/kv.c build/wormhole-256/wh.c -lm

# The driver for ART (with optimistic lock coupling); its sources lack an
# include that recent versions of GCC require
build/art_bench: build art_bench.cpp harness.hpp $(ART)/OptimisticLockCoupling/Tree.cpp
	$(ART_CXX) $(CXXFLAGS) -include cstdint -I$(ART) -o $@ art_bench.cpp $(ART)/OptimisticLockCoupling/Tree.cpp -ltbb -pthread

# The partial, sparse clone of btree-cpp
$(BTREECPP)/.fetched:
	git clone --filter=blob:none --no-checkout $(BTREECPP_URL) $(BTREECPP)
	cd $(BTREECPP) && git sparse-checkout set --no-cone /btree/ /in-memory-structures/ART/ /in-memory-structures/hot/ /named-configs/ /tlx_wrapper/TlxWrapper.h \
		&& git checkout -q $(BTREECPP_COMMIT)
	touch $@

# The driver for the structures of btree-cpp: one binary per configuration
# (the adapter of HOT is compiled as position-independent code because clang
# links position-independent executables by default, and a GCC built from
# source does not generate such code by default)
build/hot_adapter.o: build $(BTREECPP)/.fetched
	$(HOT_CXX) $(CXXFLAGS) -fPIE -c $(HOT_INCLUDES) -o $@ $(BTREECPP)/btree/hot_adapter.cpp

build/bt_bench-hot: build bt_bench.cpp harness.hpp build/hot_adapter.o $(BTREECPP)/.fetched
	$(BT_CXX) $(CXXFLAGS) -I$(BTREECPP) -include $(BTREECPP)/named-configs/hot.hpp -DNAMED_CONFIG=\"hot\" -DMETHOD=\"hot\" \
		-o $@ bt_bench.cpp $(BT_SOURCES) build/hot_adapter.o

build/bt_bench-%: build bt_bench.cpp harness.hpp $(BTREECPP)/.fetched
	$(BT_CXX) $(CXXFLAGS) -DPS_L=$(BT_PAGE_SIZE) -DPS_I=$(BT_PAGE_SIZE) -I$(BTREECPP) -include $(BTREECPP)/named-configs/$*.hpp -DNAMED_CONFIG=\"$*\" -DMETHOD=\"$*\" \
		-o $@ bt_bench.cpp $(BT_SOURCES)

clean:
	rm -rf build

FORCE:

.PHONY: all init clean FORCE
