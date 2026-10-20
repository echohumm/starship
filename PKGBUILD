pkgname="starship"
pkgver="1.27.0+106"
pkgrel="1"
pkgdesc="echohumm fork of starship"
arch=('x86_64')
url="https://github.com/echohumm/starship"
license=('ISC')
depends=('glibc')

build() {
  cargo build --release
}

package() {
  install -Dm755 "${srcdir}/../target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
