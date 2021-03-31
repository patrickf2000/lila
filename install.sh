#!/bin/bash

cargo build --release

./build-stdlib.sh

sudo cp target/release/idac /usr/bin

sudo cp share/ida.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/ida.lang /usr/share/gtksourceview-3.0/language-specs/
sudo cp share/ida4.lang /usr/share/gtksourceview-4/language-specs/ida.lang

if [[ -d /usr/lib/ida ]] ; then
    sudo rm -r /usr/lib/ida
fi

# Install core library
if [[ -f target/libida_core.a ]] ; then
    sudo cp target/libida_core.a /usr/lib
    sudo ldconfig
fi

# Install standard library
if [[ -f target/libida.so ]] ; then
    sudo cp target/libida.so /usr/lib
    sudo ldconfig
fi

# Install core library headers
if [[ -d target/core ]] ; then
    sudo mkdir -p /usr/lib/ida
    sudo cp -r target/core /usr/lib/ida
fi

# Install standard library headers
if [[ -d target/std ]] ; then
    sudo mkdir -p /usr/lib/ida
    sudo cp -r target/std /usr/lib/ida
fi

# Install start files
if [[ -f target/irt.o ]] ; then
    sudo mkdir -p /usr/lib/ida
    sudo cp target/irt.o /usr/lib/ida
fi

echo "Done"
