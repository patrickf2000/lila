#!/bin/bash

cargo build --release

./build-stdlib.sh

sudo cp target/release/lilac /usr/bin

sudo cp share/lila.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/lila.lang /usr/share/gtksourceview-3.0/language-specs/

if [[ -d /usr/lib/lila ]] ; then
    sudo rm -r /usr/lib/lila
fi

# Install core library
if [[ -f target/liblila_core.a ]] ; then
    sudo cp target/liblila_core.a /usr/lib
    sudo ldconfig
fi

# Install standard library
if [[ -f target/liblila.so ]] ; then
    sudo cp target/liblila.so /usr/lib
    sudo ldconfig
fi

# Install core library headers
if [[ -d target/core ]] ; then
    sudo mkdir -p /usr/lib/lila
    sudo cp -r target/core /usr/lib/lila
fi

# Install standard library headers
if [[ -d target/std ]] ; then
    sudo mkdir -p /usr/lib/lila
    sudo cp -r target/std /usr/lib/lila
fi

# Install start files
if [[ -f target/lrt.o ]] ; then
    sudo mkdir -p /usr/lib/lila
    sudo cp target/lrt.o /usr/lib/lila
fi

echo "Done"
