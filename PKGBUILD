# Maintainer: Soulprestigio  <soulprestigio@gmail.com>
pkgname=gooney-emu
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimalist RV64I Instruction Set Simulator and TUI validator for silicon IP"
arch=('x86_64')
url="https://github.com/gooneymart/gooney-emu"
license=('GPL2')
depends=('glibc')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/yourusername/gooney-emu/archive/v$pkgver.tar.gz")
sha256sums=('SKIP') # Replace with actual sha256 sum of release tarball

prepare() {
    cd "$pkgname-$pkgver"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --bin gooney-tui
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/gooney-tui" "$pkgdir/usr/bin/gooney-emu"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
