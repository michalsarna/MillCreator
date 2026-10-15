🌐 [English](Installation) · **Polski**

# Instalacja

Każde wydanie na [stronie Releases](https://github.com/michalsarna/MillCreator/releases) zawiera gotowe pakiety i archiwa dla każdego systemu oraz `SHA256SUMS.txt` do sprawdzenia pobranych plików (`sha256sum -c SHA256SUMS.txt`). Wydania są niezmienne: po opublikowaniu ich plików nie da się zmienić.

| System | Instalator / pakiet | Archiwum |
|---|---|---|
| Windows 10/11 (64-bit) | `millcreator-vX.Y.Z-windows-x86_64-setup.exe` | `millcreator-vX.Y.Z-windows-x86_64.zip` |
| macOS, Apple silicon (M1 i nowsze) | `millcreator-vX.Y.Z-macos-arm64.dmg` | – |
| macOS, Intel | `millcreator-vX.Y.Z-macos-x86_64.dmg` | – |
| Debian / Ubuntu i pochodne, x86_64 | `millcreator-vX.Y.Z-linux-amd64.deb` | `millcreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Debian / Ubuntu i pochodne, ARM64 | `millcreator-vX.Y.Z-linux-arm64.deb` | `millcreator-vX.Y.Z-linux-arm64.tar.gz` |
| Inny Linux | – | `.tar.gz` powyżej |

## Windows
Uruchom instalator `-setup.exe` (wpis w menu Start i dezinstalator w *Ustawienia → Aplikacje*) albo rozpakuj `.zip` gdziekolwiek i uruchom `millcreator.exe`. Instalator nie jest podpisany, więc Windows SmartScreen może ostrzec: *Więcej informacji → Uruchom mimo to*.

## macOS
Otwórz `.dmg` dla swojego Maca (*Menu Apple → Ten Mac* mówi, czy to Apple silicon, czy Intel) i przeciągnij *MillCreator* do *Programów*. Aplikacja jest podpisana tylko ad hoc, bez notaryzacji: za pierwszym razem użyj **prawy przycisk → Otwórz → Otwórz**. Jeśli macOS pisze, że aplikacja jest „uszkodzona”, jednorazowo usuń flagę kwarantanny:

```sh
xattr -dr com.apple.quarantine /Applications/MillCreator.app
```

## Linux
Zainstaluj `.deb` (`sudo apt install ./millcreator-vX.Y.Z-linux-amd64.deb`) albo rozpakuj `.tar.gz` i uruchom `millcreator`. Program potrzebuje zwykłych bibliotek X11 / Wayland i OpenGL (`libxkbcommon-x11-0`, `libgl1`); pakiet `.deb` je wymienia.

## Ze źródeł
```sh
git clone https://github.com/michalsarna/MillCreator
cd MillCreator
cargo run --release -p millcreator
```
Dodaj `--features camera` dla kamer komputera. baseCreator Cargo pobiera ze znacznika (tagu).
