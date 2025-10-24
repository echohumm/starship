pkgname="starship"
pkgver="1.23.0+20"
pkgrel="1"
pkgdesc="echohumm fork of starship"
arch=('x86_64')
url="https://github.com/echohumm/starship"
license=('ISC')
depends=('glibc')
makedepends=('rust' 'cargo')

build() {
  cargo build --release --locked
}

package() {
  install -Dm755 "${srcdir}/../target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
