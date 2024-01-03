pkgname=findit
pkgver=1.0.0
pkgrel=1
source=("https://codeload.github.com/rwietter/findit/tar.gz/refs/tags/$pkgver")
arch=('x86_64')
sha256sums=('463fc8af46ef8ff07d0989d0a67022acca5a2c80965ad3d440d0120c206d6710')
license=('MIT')
depends=('rust')
# validpgpkeys=('')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 target/release/findit "$pkgdir/usr/bin/findit"
}
