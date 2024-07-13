#!/bin/sh
sudo mkdir -p /mnt/pico
sudo mount -t drvfs d: /mnt/pico
elf2uf2-rs -d $1