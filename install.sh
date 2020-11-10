#!/bin/bash

cargo build --release

./build-stdlib.sh

sudo cp target/release/dashc /usr/local/bin

sudo cp share/dash.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/dash.lang /usr/share/gtksourceview-3.0/language-specs/

if [[ -f target/libdash.so ]] ; then
    sudo cp target/libdash.so /usr/local/lib
    sudo ldconfig
fi

if [[ -d target/std ]] ; then
    if [[ -d /usr/local/lib/dash ]] ; then
        sudo rm -r /usr/local/lib/dash
    fi
    
    sudo mkdir -p /usr/local/lib/dash
    sudo cp -r target/std /usr/local/lib/dash
fi

echo "Done"
