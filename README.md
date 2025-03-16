# electro-rs
An open-source reimplementation of the [Electro](https://electrotm.org/) WireGuard app for Linux, created through reverse engineering the Windows version. This project is unofficial and not affiliated with the original team.  
![Demo](demo.gif)

## Installation
### Prerequisites
Ensure you have **WireGuard** installed on your system. If it's not installed, use the following commands:
```bash
# Debian, Ubuntu
sudo apt install wireguard-tools

# Arch Linux
sudo pacman -S wireguard-tools
```

### Download the Latest Release (Precompiled Binary)
The easiest way to get started is by downloading the latest **precompiled binary**:
```bash
curl -LO https://github.com/Arian8j2/electro-rs/releases/latest/download/electro-rs-cli
chmod +x electro-rs-cli
```

### Build & Install from Source
If you prefer to compile it yourself, install it using `cargo`:
```bash
cargo install --git https://github.com/arian8j2/electro-rs
```
