#!/bin/sh

cargo build --release

mkdir -p ~/.local/share/todolist

sudo cp target/release/todolist /usr/local/bin/

echo "Installed successfully. You can now run 'todolist' from anywhere."
