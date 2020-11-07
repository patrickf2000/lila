#!/bin/bash

cargo build --release

sudo cp target/release/dashc /usr/local/bin

sudo cp share/dash.lang /usr/share/gtksourceview-2.0/language-specs/
sudo cp share/dash.lang /usr/share/gtksourceview-3.0/language-specs/

echo "Done"
