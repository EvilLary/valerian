pkgname="valerian-git"
_pkgname="valerian"
pkgver=0.1.c59cccf
pkgrel=1
pkgdesc="simple app to fetch cars from the internet"
arch=(any)
conflicts=('valerian')
provides=("valerian=${pkgver%%.r*}")
license=('MIT')
makedepends=('cargo')
_pkgsrc="$_pkgname"
source=("git+https://codeberg.org/EvilLary/valerian.git")
sha256sums=('SKIP')

pkgver() {
    cd $_pkgsrc
    printf "0.1.%s" "$(git rev-parse --short HEAD)"
}

build() {
    cd $_pkgsrc
    cargo build --release
}

package() {
    cd $_pkgsrc
    install -Dm755 "target/release/valerian" "$pkgdir/usr/bin/valerian"
}
