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
| Fedora, openSUSE, x86_64 | `millcreator-vX.Y.Z-linux-x86_64.rpm` | `millcreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Fedora, openSUSE, ARM64 | `millcreator-vX.Y.Z-linux-aarch64.rpm` | `millcreator-vX.Y.Z-linux-arm64.tar.gz` |
| Other Linux | – | the `.tar.gz` above |
| FreeBSD, x86_64 | – | `millcreator-vX.Y.Z-freebsd-x86_64.tar.gz` |

## Package managers

The release is also available through package managers, which install the newest version and update it:

| System | Command |
|---|---|
| macOS (Homebrew) | `brew install --cask michalsarna/tap/millcreator` (later `brew upgrade`) |
| Windows (Scoop) | `scoop bucket add creator https://github.com/michalsarna/scoop-bucket`, then `scoop install millcreator` (later `scoop update millcreator`) |
| Windows (winget) | `winget install MichalSarna.MillCreator` once Microsoft has accepted the package into its catalogue (submitted, waiting for review) |

The Homebrew cask and the Scoop manifest follow every release by themselves. The macOS app is not notarized, so the first time see the note in the *macOS* section below. An Arch Linux (AUR) package is prepared but not published yet.

## Windows
Run the `-setup.exe` installer (Start-menu entry and an uninstaller in *Settings → Apps*) or unpack the `.zip` anywhere and start `millcreator.exe`. The installer is not code-signed, so Windows SmartScreen may warn: *More info → Run anyway*.

## macOS
Open the `.dmg` for your Mac (*Apple menu → About This Mac* tells you whether it is Apple silicon or Intel) and drag *MillCreator* to *Applications*. The app is only ad-hoc signed, not notarized: the first time use **right-click → Open → Open**. If macOS says the app is "damaged", remove the quarantine flag once:

```sh
xattr -dr com.apple.quarantine /Applications/MillCreator.app
```

## Linux
Install the `.deb` (`sudo apt install ./millcreator-vX.Y.Z-linux-amd64.deb`), on Fedora or openSUSE the `.rpm` (`sudo dnf install ./millcreator-vX.Y.Z-linux-x86_64.rpm`, openSUSE: `sudo zypper install ./...`; it needs a recent glibc, so not RHEL / Alma / Rocky 9), or unpack the `.tar.gz` and run `millcreator`. The program needs the usual X11 / Wayland and OpenGL libraries (`libxkbcommon-x11-0`, `libgl1`); the `.deb` and the `.rpm` list them.

## FreeBSD
Unpack `millcreator-vX.Y.Z-freebsd-x86_64.tar.gz` and run `millcreator`. It needs a few packages and an X11 or Wayland desktop (as root):

```sh
pkg install libudev-devd libxkbcommon mesa-libs ffmpeg
```

`ffmpeg` is for network (IP) cameras; USB cameras do not work on FreeBSD yet (the camera says so). The machine on USB is `/dev/cuaU0`: add yourself to the group `dialer` (`pw groupmod dialer -m $USER`, then log in again). Built and its tests run on FreeBSD 15.0 (x86_64 only); not yet tried on a FreeBSD desktop. To build it from source there: `pkg install rust pkgconf libudev-devd evdev-proto` first.

## From source
```sh
git clone https://github.com/michalsarna/MillCreator
cd MillCreator
cargo run --release -p millcreator
```
Add `--features camera` for cameras of the computer. baseCreator is fetched by Cargo from its tag.
