# Gözcü — Süreç Yöneticisi

[🇹🇷 Türkçe](#türkçe) | [🇬🇧 English](#english)

---

<a name="türkçe"></a>

## Türkçe

KDE Plasma'ya gömülü sistem izleyicisine derin menülerde aramak yerine tek tıkla
açabileceğiniz, Rust ile yazılmış hafif bir süreç yöneticisi. Koyu temalıdır.

![Gözcü](https://d2z0o16i8xm8ak.cloudfront.net/c4dd606e-2c3e-44fb-9595-cc44529a65b9/04588b70-0949-4527-aa92-f5cb20ac3bee/screenshot.png?Policy=eyJTdGF0ZW1lbnQiOlt7IlJlc291cmNlIjoiaHR0cHM6Ly9kMnowbzE2aTh4bThhay5jbG91ZGZyb250Lm5ldC9jNGRkNjA2ZS0yYzNlLTQ0ZmItOTU5NS1jYzQ0NTI5YTY1YjkvMDQ1ODhiNzAtMDk0OS00NTI3LWFhOTItZjVjYjIwYWMzYmVlL3NjcmVlbnNob3QucG5nPyoiLCJDb25kaXRpb24iOnsiRGF0ZUxlc3NUaGFuIjp7IkFXUzpFcG9jaFRpbWUiOjE3OTQzMTYwMzR9fX1dfQ__&Signature=WKvCSYpBflAHAKPrMf2qtIT2GKX79tny-~Fp2hDRA4xTPlvKCYhNVHT3punSkJ2Pd-ao110dcmATrZJU5bqFewq3GXk6BAnacDLIkBTVFo0OXOk7uEYO31V7qciAhj89GFeOvjF0mgRfc-gXWPbgemvGmHl148oJ90Nbbw4AYiLv40CBi6wcZe7WEAmZlYHDjNGkNNab3IVGPGvkPh7OSc2yzTQidznLO33YGnVwa-6AyVdxxQrSBZCJaVwN136H4F-kxFAF3gxOAieuxd~YBO-VrgDMpMW~WGG-Yt8ElVXO1M~rID4quD6fpnzHmbO6sZ2~5ItMtrBxhFuVnossHw__&Key-Pair-Id=K1BF7XGXAIMYNX)

### Özellikler

- Tüm süreçleri listeler: PID, süreç adı, kullanıcı, CPU %, bellek ve durum
- **Arama**: sürecin adı, kullanıcı adı veya PID ile anında filtreleme
- **Çift dil**: Türkçe/İngilizce — sağ üstteki dil açılır kutusundan seçilir;
  açılışta sistem diline göre seçilir, yeni diller kolayca eklenebilir
- **Sağ tık menüsü**: bir sürecin satırına sağ tıklayınca açılır:
  - `Sonlandır (TERM)` — nazik yolla kapatma, süreç önce dosyalarını kaydeder
  - `Duraklat (STOP)` — süreci askıya alır (SIGSTOP)
  - `Devam (CONT)` — duraklatılan süreci sürdürür (SIGCONT)
  - `Öldür (KILL)` — SIGKILL; direnen süreci zorla keser (kırmızı yazıyla)
  - Sağ tık aynı anda satırı da seçer; menü dışına tıklayınca kapanır
- Sıralama: CPU, bellek veya PID (düğme ile değiştirilir)
- Listeyi saniyede bir otomatik yeniler (manuel "Yenile" düğmesi de var)
- Seçili sürecin sonucunu durum satırında gösterir (başarı/hata)
- Koyu tema (fluent-dark)

### Gereksinimler

- Rust (rustup ile: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Derleme için sistem kütüphaneleri:
  - Ubuntu/Debian: `sudo apt install libfontconfig-dev pkg-config`
  - Arch/Manjaro: `sudo pacman -S fontconfig pkgconf`
- Çalışma zamanında yalnızca `libfontconfig` ve `libfreetype` gerekir (masaüstünde
  zaten bulunur).

### Derleme ve çalıştırma

```bash
cargo build --release
./target/release/gozcu
```

İsteğe bağlı: belirli bir filtreyle açılmak için:

```bash
./target/release/gozcu --filter firefox
```

### KDE menüsüne kısayol ekleme

```bash
mkdir -p ~/.local/share/applications
cp gozcu.desktop ~/.local/share/applications/
# Exec= satırındaki yolu, ikilinin bulunduğu tam yolla güncelleyin:
#   örn. Exec=/home/kullanici/gozcu/target/release/gozcu
update-desktop-database ~/.local/share/applications
```

Artık uygulama menüsünde "Gözcü" olarak görünür; görev çubuğuna da
sabitleyebilirsiniz.

### Notlar

- Başka kullanıcıya (örn. root) ait süreçleri öldürmek için uygulamanın yetkisi
  yoktur; durum satırında "yetki yok" uyarısı alırsanız uygulamayı sudo ile
  çalıştırmanız gerekir.
- Süreç durdurma/kapatma gibi eylemler geri alınamaz; SIGKILL verilen süreçte
  kaydedilmemiş veri kaybolabilir.

### Teknoloji

- [Slint](https://slint.rs) — Rust ile GUI (egui kullanılmadan, masaüstüne uygun
  yerleşik görünümlü widget seti)
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — süreç bilgisi ve sinyal
  gönderme

### Lisans

MIT.

---

<a name="english"></a>

## English

A lightweight process manager written in Rust — open it with a single click
instead of digging through KDE Plasma's built-in system monitor menus. Dark
themed.

![Gözcü](https://d2z0o16i8xm8ak.cloudfront.net/c4dd606e-2c3e-44fb-9595-cc44529a65b9/83510a47-7e62-48c4-828a-4e58f2925330/screenshot-en.png?Policy=eyJTdGF0ZW1lbnQiOlt7IlJlc291cmNlIjoiaHR0cHM6Ly9kMnowbzE2aTh4bThhay5jbG91ZGZyb250Lm5ldC9jNGRkNjA2ZS0yYzNlLTQ0ZmItOTU5NS1jYzQ0NTI5YTY1YjkvODM1MTBhNDctN2U2Mi00OGM0LTgyOGEtNGU1OGYyOTI1MzMwL3NjcmVlbnNob3QtZW4ucG5nPyoiLCJDb25kaXRpb24iOnsiRGF0ZUxlc3NUaGFuIjp7IkFXUzpFcG9jaFRpbWUiOjE3OTQzMTYwMzV9fX1dfQ__&Signature=kOvQBBB8~LDIXbuVQxsdORYn3ABeYL7Ohl3j-fEVysYaph8CzSojmBDSry2p6I7~~Mf0mV2KDwhhnxyGwXLBFXgLeAxiRI7uZY2PihK4sdLAqpt7Jk74lWe9fv7kqbPX0Wu3u~7b04eDZkw5nDsKPx0ZoPLXPHiIaOh47EL481lHS3egUHVKxlCXNjrc46xrGh~SlWEoOyhFk-~ZJIFmMc0L36OUvHtC3cHd4kM27ThYxWm1y9bNyQ1y7WBRotTzBeEnng3r1kVfpVdnY9bhtPXwgnQKxN-JvQEUC8bSpauzuXa6tPlf2w3GT3MHuoKNWVsuXexcGOMTTprejIEjNw__&Key-Pair-Id=K1BF7XGXAIMYNX)

### Features

- Lists all processes: PID, name, user, CPU %, memory and state
- **Search**: instant filtering by process name, user name or PID
- **Two languages**: Turkish/English — pick from the dropdown in the top-right
  corner; chosen automatically from the system locale at startup, new languages
  are easy to add
- **Right-click menu**: opens on a process row:
  - `Terminate (TERM)` — graceful shutdown, the process saves its files first
  - `Pause (STOP)` — suspends the process (SIGSTOP)
  - `Resume (CONT)` — continues a paused process (SIGCONT)
  - `Kill (KILL)` — SIGKILL; forcefully kills stubborn processes (in red)
  - Right-click also selects the row; the menu closes on outside clicks
- Sorting: CPU, memory or PID (toggled with a button)
- Auto-refreshes the list once per second (a manual "Refresh" button is
  available too)
- Shows the result of an action in the status line (success/error)
- Dark theme (fluent-dark)

### Requirements

- Rust (via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- System libraries for building:
  - Ubuntu/Debian: `sudo apt install libfontconfig-dev pkg-config`
  - Arch/Manjaro: `sudo pacman -S fontconfig pkgconf`
- At runtime only `libfontconfig` and `libfreetype` are needed (already present
  on any desktop system).

### Building and running

```bash
cargo build --release
./target/release/gozcu
```

Optional: start with a pre-set filter:

```bash
./target/release/gozcu --filter firefox
```

### Adding a KDE menu shortcut

```bash
mkdir -p ~/.local/share/applications
cp gozcu.desktop ~/.local/share/applications/
# Update the path in the Exec= line to the full path of the binary:
#   e.g. Exec=/home/user/gozcu/target/release/gozcu
update-desktop-database ~/.local/share/applications
```

It now appears as "Gözcü" in the application menu; you can also pin it to the
taskbar.

### Notes

- The app cannot kill processes owned by other users (e.g. root); if you get a
  "permission denied" warning in the status line, run it with sudo.
- Process termination/kill actions are irreversible; unsaved data may be lost in
  a SIGKILLed process.

### Technology

- [Slint](https://slint.rs) — GUI in Rust (no egui; a native-looking widget
  set suited to the desktop)
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — process info and
  signal sending

### License

MIT.
