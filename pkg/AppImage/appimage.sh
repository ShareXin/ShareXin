#!/bin/bash

version=$(cat version)
ARCH=$(uname -m)

echo $version
echo $ARCH

git pull
LDFLAGS="-static" cargo build --release
mkdir -p sharexin.AppDir/usr/bin/
mkdir -p sharexin.AppDir/usr/lib/x86_64-linux-gnu
mkdir -p sharexin.AppDir/lib/x86_64-linux-gnu
cp target/release/sharexin sharexin.AppDir/usr/bin/sharexin
mkdir releases/$version
cp pkg/AppImage/AppRun sharexin.AppDir/
cp pkg/AppImage/sharexin.desktop sharexin.AppDir/
if [ ! -f appimagetool-x86_64.AppImage ]; then
	curl https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage --output appimagetool-x86_64.AppImage
fi
ldd target/release/sharexin | grep "=> /" | awk '{print $3}' | xargs -I '{}' cp -v '{}' sharexin.AppDir/usr/lib/x86_64-linux-gnu
./appimagetool-x86_64.AppImage -n sharexin.AppDir releases/$version/sharexin-$version-$ARCH.AppImage
