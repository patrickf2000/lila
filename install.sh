#!/bin/bash

cargo build --release

./build-stdlib.sh

sudo cp target/release/lilac /usr/bin

sudo cp share/lila.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/lila.lang /usr/share/gtksourceview-3.0/language-specs/

if [[ -f target/liblila.so ]] ; then
    sudo cp target/liblila.so /usr/lib
    sudo ldconfig
fi

if [[ -d target/std ]] ; then
    if [[ -d /usr/lib/lila ]] ; then
        sudo rm -r /usr/lib/lila
    fi
    
    sudo mkdir -p /usr/lib/lila
    sudo cp -r target/std /usr/lib/lila
fi

if [[ -f target/lrt.o ]] ; then
    sudo mkdir -p /usr/lib/lila
    sudo cp target/lrt.o /usr/lib/lila
fi

echo "Done"
