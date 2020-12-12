#!/bin/bash

cargo build --release

./build-stdlib.sh

sudo cp target/release/lilac /usr/local/bin

sudo cp share/lila.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/lila.lang /usr/share/gtksourceview-3.0/language-specs/

if [[ -f target/liblila.so ]] ; then
    sudo cp target/libdash.so /usr/local/lib
    sudo ldconfig
fi

if [[ -d target/std ]] ; then
    if [[ -d /usr/local/lib/lila ]] ; then
        sudo rm -r /usr/local/lib/lila
    fi
    
    sudo mkdir -p /usr/local/lib/lila
    sudo cp -r target/std /usr/local/lib/lila
fi

echo "Done"
