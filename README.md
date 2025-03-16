# electro-rs
An open-source reimplementation of the [Electro](https://electrotm.org/) WireGuard app for Linux, created through reverse engineering the Windows version. This project is unofficial and not affiliated with the original team.  

## Installation
1. **Install WireGuard**
   - Debian/Ubuntu:
     ```bash
     sudo apt install wireguard-tools
     ```
   - Arch Linux:
     ```bash
     sudo pacman -S wireguard-tools
     ```

2. **Install Rust & Compile**
   ```bash
   cargo install --git https://github.com/arian8j2/electro-rs
   ```
