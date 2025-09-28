# PKGBUILD example for a simple Rust binary
pkgname="starship"
pkgver="1.23.0+8"
pkgrel="1"
pkgdesc="A short description of your Rust application"
arch=('x86_64')         # Or other architectures as needed
url="https://github.com/echohumm/starship"
license=('MIT')         # Replace with your license
depends=('glibc')       # Add any runtime dependencies
makedepends=('rust' 'cargo') # Build dependencies

#source=("${pkgname}-${pkgver}.tar.gz::https://github.com//${pkgname}/archive/v${pkgver}.tar.gz") # Replace with your source URL
#sha256sums=('SKIP')

build() {
  cargo build --release --locked
}

package() {
  install -Dm755 "${srcdir}/../target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
