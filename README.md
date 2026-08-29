# Corrupt File Generator

**males** is a simple tool for generating corrupt (dummy) files with a user-specified
name, format, and size. It fills files with random data and, where the format is known,
prepends the correct magic bytes so the file is recognized by tools such as `file`.

## Features

- **Customizable:** Create files with a specified name, format, and size.
- **Format signatures:** Known formats are stamped with their correct magic bytes
  (`mp4`, `mkv`, `mp3`, `wav`, `jpg`/`jpeg`, `png`, `gif`, `webp`, `pdf`, `zip`,
  `docx`, `xlsx`, `rar`, `7z`).
- **Multi-interface:** Available as Bash, Python, and Rust command-line tools, plus a
  web application.
- **Cross-platform (Rust):** Uses a self-contained xorshift pseudo-random generator
  instead of `/dev/urandom`, so it builds and runs on Linux, macOS, and Windows.

## Implementations

| Interface | Language | Location |
|-----------|----------|----------|
| CLI | Rust | `males-rust/` |
| CLI | Python | `males.py` |
| CLI | Bash | `males.sh` |
| Web app | Flask + HTML/JS | `backend/`, `frontend/`, `docker-compose.yml` |
| Release binaries | Rust | GitHub Releases |

## Quick Start

The fastest way to play with the app is to grab a prebuilt binary from the
[latest release](https://github.com/khoirulanammyid/males/releases) — no Rust toolchain needed:

| Platform | Asset |
|----------|-------|
| Linux | `males-rust-<version>-linux` |
| macOS | `males-rust-<version>-macos` |
| Windows | `males-rust-<version>-windows.exe` |

Make it executable and run (Linux/macOS):

```bash
chmod +x males-rust-<version>-linux
./males-rust-<version>-linux
```

### Run in one command

Or let `curl` + `grep` grab the latest Linux binary and run it directly:

```bash
curl -sL "$(curl -s https://api.github.com/repos/khoirulanammyid/males/releases/latest | grep -oE 'https://[^"]*-linux[^"]*' | head -1)" -o males && chmod +x males && ./males
```

For macOS, swap `-linux` for `-macos` in the URL pattern:

```bash
curl -sL "$(curl -s https://api.github.com/repos/khoirulanammyid/males/releases/latest | grep -oE 'https://[^"]*-macos[^"]*' | head -1)" -o males && chmod +x males && ./males
```

> **Note for macOS:** the first run may be blocked by Gatekeeper. Fix it once with
> `xattr -d com.apple.quarantine males`, or right-click the file and choose **Open**.

## Playing with the Rust CLI

Build and run from source:

```bash
cd males-rust
cargo run --release
```

The program prompts for a file name, a format, and a size. Here is a full session:

```text
$ cargo run --release

Nama file (tanpa ekstensi): dummy
Format (mp4, mkv, mp3, wav, jpg, jpeg, png, gif, webp, pdf, zip, docx, xlsx, rar, 7z): pdf
Ukuran (contoh: 5M, 10K, 100): 5M

Mengisi data acak...
File korup 'dummy.pdf' (5242880 byte) dengan magic bytes berhasil dibuat.
```

- **Format:** any extension listed in the prompt. Unknown extensions still produce a
  file, just without magic bytes.
- **Size:** plain bytes (`100`) or suffixes in multiples of 1024 — `K` (KiB), `M`
  (MiB), `G` (GiB).

### Fun things to try

Generate files and see how tools recognize (or fail to recognize) them:

```bash
# Create a "jpg", then ask the system what it really is
printf 'photo\njpg\n10K\n' | cargo run --release
file photo.jpg          # -> JPEG image data

# Create a fake PDF
printf 'report\npdf\n50K\n' | cargo run --release
file report.pdf         # -> PDF document, version 1.4
xdg-open report.pdf     # will probably refuse to open it

# Use any random extension — no magic bytes are stamped
printf 'mystery\nabcdef\n1M\n' | cargo run --release
xxd mystery.abcdef | head
```

Because only the header is real, the files open partially or not at all — perfect for
testing upload validators, bug reporting workflows, or apps that check file types.

## Python

```bash
python males.py
```

Prompts for a name, format, and size (plain bytes):

```text
Name: dummy
Format (e.g., doc, jpg, mp4): jpg
Size (in bytes, e.g., 1024): 4096
File 'dummy.jpg' of size 4096 bytes has been created.
```

## Bash

```bash
chmod +x males.sh
./males.sh
```

Prompts for a name, format, and size (plain bytes), then fills the file from
`/dev/urandom`.

## Web App

Run the frontend and backend with Docker Compose:

```bash
docker compose up --build
```

- Frontend: <http://localhost:8080>
- Backend API: <http://localhost:5001>

Open the frontend, type a name, extension, and size, then click
**Generate & Download** to download your corrupt file. The backend serves it directly
from memory (max 500 MB).

## Conclusion

This Corrupt File Generator is a simple yet useful tool to quickly create corrupt files
for testing or experimentation. With CLI tools in Bash, Python, and Rust — plus a
browser-based web app and prebuilt release binaries — you can easily generate files
with random data in any format and size. We hope this tool is helpful for your needs,
and feel free to contribute or provide feedback to improve it further.

Thank you for using this tool, and happy coding!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.