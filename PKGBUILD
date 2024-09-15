pkgname=abrw-2
pkgver=1.2
pkgrel=1
pkgdesc="Abrw"
arch=('x86_64')
url="https://abrw.aapelix.dev/"
license=('GPL')
depends=('gtk3' 'webkit2gtk')
source=("abrw-2.desktop")
sha256sums=('SKIP')

package() {
  mkdir -p "$pkgdir/usr/bin"
  install -Dm755 "$srcdir/../target/release/abrw-2" "$pkgdir/usr/bin/abrw-2"
  mkdir -p "$pkgdir/usr/share/applications"
  install -Dm644 "$srcdir/abrw-2.desktop" "$pkgdir/usr/share/applications/abrw-2.desktop"
}
