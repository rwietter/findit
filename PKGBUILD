pkgname=findit
pkgver=0.0.1
pkgrel=1
source=("https://codeload.github.com/rwietter/findit/tar.gz/refs/tags/$pkgver")
arch=('x86_64')
sha256sums=('83652f7ad56737fdc8b1ff97376133506a7e978b746b495fa1f08b266a688d6a')
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
