🌐 **English** · [Polski](Instalacja)

# Installation

Every release on the [Releases page](https://github.com/michalsarna/MillCreator/releases) holds ready-made packages and archives for each system, plus `SHA256SUMS.txt` to verify the downloads (`sha256sum -c SHA256SUMS.txt`). Releases are immutable: once published, their files cannot be changed.

| System | Installer / package | Archive |
|---|---|---|
| Windows 10/11 (64-bit) | `millcreator-vX.Y.Z-windows-x86_64-setup.exe` | `millcreator-vX.Y.Z-windows-x86_64.zip` |
| macOS, Apple silicon (M1 and later) | `millcreator-vX.Y.Z-macos-arm64.dmg` | – |
| macOS, Intel | `millcreator-vX.Y.Z-macos-x86_64.dmg` | – |
| Debian / Ubuntu and derivatives, x86_64 | `millcreator-vX.Y.Z-linux-amd64.deb` | `millcreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Debian / Ubuntu and derivatives, ARM64 | `millcreator-vX.Y.Z-linux-arm64.deb` | `millcreator-vX.Y.Z-linux-arm64.tar.gz` |
| Other Linux | – | the `.tar.gz` above |

## Windows
Run the `-setup.exe` installer (Start-menu entry and an uninstaller in *Settings → Apps*) or unpack the `.zip` anywhere and start `millcreator.exe`. The installer is not code-signed, so Windows SmartScreen may warn: *More info → Run anyway*.

## macOS
Open the `.dmg` for your Mac (*Apple menu → About This Mac* tells you whether it is Apple silicon or Intel) and drag *MillCreator* to *Applications*. The app is only ad-hoc signed, not notarized: the first time use **right-click → Open → Open**. If macOS says the app is "damaged", remove the quarantine flag once:

```sh
xattr -dr com.apple.quarantine /Applications/MillCreator.app
```

## Linux
Install the `.deb` (`sudo apt install ./millcreator-vX.Y.Z-linux-amd64.deb`) or unpack the `.tar.gz` and run `millcreator`. The program needs the usual X11 / Wayland and OpenGL libraries (`libxkbcommon-x11-0`, `libgl1`); the `.deb` lists them.

## From source
```sh
git clone https://github.com/michalsarna/MillCreator
cd MillCreator
cargo run --release -p millcreator
```
Add `--features camera` for cameras of the computer. baseCreator is fetched by Cargo from its tag.
