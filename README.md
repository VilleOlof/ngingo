# Nginxgo

A minimal, lightweight way to manage Nginx proxy configurations.  

## Installation

```bash
git clone https://github.com/VilleOlof/nginxgo.git
cd nginxgo
cargo build --release

# Symlink the binary to a directory in your PATH
sudo ln -s $(pwd)/target/release/nginxgo /usr/local/bin/nginxgo
```

## Usage

```bash
> nginxgo
# And then follow the instructions
```

*Do note that symlinks only work on Unix-like systems.*  
*And the auto-restart uses `systemctl` to restart the Nginx service.*