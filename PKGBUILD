# Maintainer: TheBitStick <thebitstick@tfwno.gf>

pkgname=sharexin
pkgver=0.6.1
pkgrel=0
pkgdesc="ShareX for Unix-like systems"
url="https://github.com/ShareXin/ShareXin"
makedepends=('rust' 'cargo' 'curl' 'gtk3' 'gdk-pixbuf2' 'cairo' 'glib2' 'openssl' 'dbus' 'xcb-util' 'gcc' 'cmake')
arch=('x86_64')
license=('MIT')
source=("$pkgname-$pkgver.tar.gz::https://crates.io/api/v1/crates/$pkgname/$pkgver/download")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  LDFLAGS="-static" cargo build --release
}

package() {
  cd "$pkgname-$pkgver"
  strip "target/release/$pkgname"
  strip -s "target/release/$pkgname"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/sharexin"
}
