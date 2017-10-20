#!/bin/sh

PWD=$(pwd)
TMP=/tmp/ShareXin

mkdir -p "$TMP"
cp pkg/AUR/PKGBUILD "$TMP"
cd "$TMP"

makepkg -si
cd "$PWD"
rm -rf "$TMP"
