#!/bin/bash

# —————————— Install deps ——————————

sudo dnf -y install clang g++ boost boost-devel tbb-devel xxhash-devel git cmake tmux htop parallel java-25 maven maven-antrun-plugin ant ivy gcc14 gcc14-c++ texinfo ncurses-devel perl

# —————————— Install Rust ——————————

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
