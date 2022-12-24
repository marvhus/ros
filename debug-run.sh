#!/bin/bash
set -xe

qemu-system-x86_64 -drive format=raw,file=target/x86_64-marvhus-ros/debug/bootimage-ros.bin
