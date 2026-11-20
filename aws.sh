#!/bin/bash

# —————————— Install deps ——————————

sudo dnf -y install clang g++ boost boost-devel tbb-devel xxhash-devel jemalloc git cmake ninja-build wget patch tmux htop parallel perf java-25 maven maven-antrun-plugin ant ivy gcc14 gcc14-c++ texinfo ncurses-devel perl perl-open


# —————————— Install GCC ——————————

export GCCVER=16.2.0

build_gcc() {
	cd ~
	wget https://mirrors.ocf.berkeley.edu/gnu/gcc/gcc-$GCCVER/gcc-$GCCVER.tar.gz
	tar zxf gcc-$GCCVER.tar.gz
	cd gcc-$GCCVER
	./contrib/download_prerequisites
	mkdir build
	cd build
	../configure --enable-languages=c,c++ --prefix=/usr/local --disable-multilib --disable-bootstrap
	make -j$(nproc)
	sudo make install
	cd x86_64-pc-linux-gnu/libstdc++-v3/
	make -j$(nproc)
	sudo make install
	cd
	echo "export LIBRARY_PATH=/usr/local/lib64" >>~/.bashrc
	echo "export LD_LIBRARY_PATH=/usr/local/lib64" >>~/.bashrc
}


# —————————— Install CLang ——————————

# Wormhole is fastest when compiled with clang, and the B-tree of btree-cpp
# needs it, so we build it, too (a release, rather than the current sources).

export LLVMVER=23.1.6

build_clang() {
	cd ~
	git clone --depth=1 --branch llvmorg-$LLVMVER https://github.com/llvm/llvm-project.git
	cd llvm-project
	mkdir build
	cd build
	cmake -DLLVM_ENABLE_PROJECTS=clang -DCMAKE_BUILD_TYPE=Release -DLLVM_TARGETS_TO_BUILD=host -G Ninja ../llvm
	ninja
	sudo ninja install
	cd
}

# The rest of the setup does not need the new compilers, so it runs while they
# are being built.

build_gcc >~/gcc-build.log 2>&1 &
GCC_PID=$!
echo "Building GCC (log in ~/gcc-build.log)"

build_clang >~/clang-build.log 2>&1 &
CLANG_PID=$!
echo "Building CLang (log in ~/clang-build.log)"


# —————————— Install Rust ——————————

cd ~
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s  -- -y
. "$HOME/.cargo/env"

# —————————— Clone and initialize repos ——————————

git clone --recursive https://github.com/vigna/zfast.git

# —————————— Install ne ——————————

git clone https://github.com/vigna/ne.git

cd ne
make -j$(nproc)
cd ..
mkdir bin
mv ne/src/ne bin
export PATH="$HOME/bin:$PATH"


# —————————— Install lifekeeper ——————————

curl -fsSL https://vigna.di.unimi.it/lk.sh | sh
cat >>~/.bashrc <<EOF
if [ "\$PS1" ]; then
    lifekeeper & disown
fi
EOF


# —————————— Wait for the compilers ——————————

wait $GCC_PID && echo "GCC done" || { echo "GCC build failed: see ~/gcc-build.log"; exit 1; }
wait $CLANG_PID && echo "CLang done" || { echo "CLang build failed: see ~/clang-build.log"; exit 1; }
export LIBRARY_PATH=/usr/local/lib64
export LD_LIBRARY_PATH=/usr/local/lib64

# The new compilers are installed in /usr/local/bin, which precedes /usr/bin
# in the path: the drivers of the competitors (make -C zfast/bench) must be
# built after this point.
