#!/bin/bash

cargo build --release

sudo cp target/release/dashc /usr/local/bin

echo "Done"
