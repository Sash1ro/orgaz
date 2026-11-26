#!/usr/bin/env sh
set -eu

REPO="${ORGAZ_REPO:-Sash1ro/file-organizer}"
VERSION="${ORGAZ_VERSION:-latest}"
INSTALL_DIR="${ORGAZ_INSTALL_DIR:-$HOME/.local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) OS_TARGET="apple-darwin" ;;
  Linux)  OS_TARGET="unknown-linux-gnu" ;;
  *)
    echo "Unsupported OS: $OS" >&2
    exit 1
    ;;
esac

case "$ARCH" in
  arm64|aarch64) ARCH_TARGET="aarch64" ;;
  x86_64|amd64)  ARCH_TARGET="x86_64" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

TARGET="${ARCH_TARGET}-${OS_TARGET}"
ARCHIVE="orgaz-${TARGET}.tar.gz"

if [ "$VERSION" = "latest" ]; then
  URL="https://github.com/${REPO}/releases/latest/download/${ARCHIVE}"
else
  case "$VERSION" in
    v*) TAG="$VERSION" ;;
    *)  TAG="v${VERSION}" ;;
  esac
  URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE}"
fi

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT INT TERM

echo "Downloading ${URL}..."
curl -fsSL "$URL" -o "${TMP_DIR}/${ARCHIVE}"
tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "$TMP_DIR"

mkdir -p "$INSTALL_DIR"
install -m 755 "${TMP_DIR}/orgaz" "${INSTALL_DIR}/orgaz"
echo "Installed orgaz to ${INSTALL_DIR}/orgaz"

case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    echo "orgaz is already in your PATH. Run: orgaz --help"
    ;;
  *)
    SHELL_NAME="$(basename "${SHELL:-sh}")"
    case "$SHELL_NAME" in
      zsh)
        PROFILE="$HOME/.zshrc"
        ;;
      bash)
        if [ -f "$HOME/.bashrc" ]; then
          PROFILE="$HOME/.bashrc"
        else
          PROFILE="$HOME/.bash_profile"
        fi
        ;;
      *)
        PROFILE="$HOME/.profile"
        ;;
    esac

    LINE="export PATH=\"${INSTALL_DIR}:\$PATH\""
    if [ ! -f "$PROFILE" ] || ! grep -Fq "$INSTALL_DIR" "$PROFILE"; then
      printf '\n# Added by orgaz installer\n%s\n' "$LINE" >> "$PROFILE"
      echo "Added ${INSTALL_DIR} to PATH in ${PROFILE}"
    fi
    echo "Restart your terminal or run: source ${PROFILE}"
    ;;
esac
