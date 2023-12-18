# CLI Tool for Managing LuaSnip Rust Snippets

Note: This is an additional tools to be used with my [Neovide Neovim](https://github.com/codeitlikemiley/nvim) rust setup.

## Installation

1. You can Download and Install [rsnippet](https://github.com/codeitlikemiley/rsnippet/releases) on Releases Page
Note: on MacOS you might need to go to System Preferences > Security & Privacy > General and click Open Anyway to install it

Note: on Windows you might need to Add the command to ENV PATH

2. Build it from source
### Clone
```sh
git clone htps://github.com/codeitlikemiley/rsnippet.git
cd rsnippet
```

### For MacOS
```sh
./provision.sh
```

### For Linux
```sh
cargo build --release
mv ./target/release/rsnippet /usr/local/bin/rsnippet
chmod +x /usr/local/bin/rsnippet
```

### For Windows
```sh
cargo build --release

# Replace 'YourUsername' with your actual username
Move-Item .\target\release\rsnippet.exe C:\Users\YourUsername\bin\rsnippet.exe

# Again, replace 'YourUsername' with your actual username
$env:Path += ";C:\Users\YourUsername\bin"
```

