pkgname="starship"
pkgver="1.23.0+9"
pkgrel="1"
pkgdesc="A short description of your Rust application"
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
