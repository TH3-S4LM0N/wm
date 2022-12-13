#!/usr/bin/env bash

cargo build --release
sudo cp ./penrose-cfg.desktop /usr/share/xsessions
sudo install -s ./target/release/penrose-wm /bin