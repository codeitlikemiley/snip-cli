# A CLI (snipr) Tool for Managing Neovim and VSCode Snippets

[![Rust Build and Test](https://github.com/codeitlikemiley/snipr/actions/workflows/test.yml/badge.svg)](https://github.com/codeitlikemiley/snipr/actions/workflows/test.yml)

[![release](https://github.com/codeitlikemiley/snipr/actions/workflows/release.yml/badge.svg)](https://github.com/codeitlikemiley/snipr/actions/workflows/release.yml)

Note: This is an additional tools to be used with my [Neovide Neovim](https://github.com/codeitlikemiley/nvim) rust setup.

> VSCode Snippet Compatible

## Installation

1. You can Download and Install [snipr](https://github.com/codeitlikemiley/snipr/releases) on Releases Page
Note: on MacOS you might need to go to System Preferences > Security & Privacy > General and click Open Anyway to install it

Note: on Windows you might need to Add the command to ENV PATH

or Install it using Cargo

```sh
cargo install snipr
```



2. Build it from source
### Clone
```sh
git clone htps://github.com/codeitlikemiley/snipr.git
cd snipr
```

### For MacOS
```sh
./provision.sh
```

### For Linux
```sh
cargo build --release
mv ./target/release/snipr /usr/local/bin/snipr
chmod +x /usr/local/bin/snipr
```

### For Windows
```sh
cargo build --release

# Replace 'YourUsername' with your actual username
Move-Item .\target\release\snipr.exe C:\Users\YourUsername\bin\snipr.exe

# Again, replace 'YourUsername' with your actual username
$env:Path += ";C:\Users\YourUsername\bin"
```

## Managing Multiple Snippets File for Different Languages

You can use the command `snipr config <path>` to change the Snippets File on Runtime.

Note: This is quite useful when you want to manage different Snippets File for different languages.

## Location of  Config Path

The `DEFAULT_CONFIG_PATH` is `~/.config/nvim/snippets/rust/rust.json` as set on `src/constants.rs`  and is used during the build process,

but you can change it by setting the `SNIP_CONFIG_PATH` environment variable.

## Usage:
1. Help

```sh
snipr
# or
snipr --help
```

<details>
<summary>Output</summary>

```sh
snipr
/Users/uriah/.config/nvim/snippets/rust/rust.json
A CLI tool for managing Neovim LuaSnip Rust snippets

Usage: snipr <COMMAND>

Commands:
  add         Adds entry to Snippet Collection file
  rm          Removes entry from Snippet Collection file
  edit        Edits entry in Snippet Collection file
  ls          Lists all entries in Snippet Collection file
  show        Gets entry from Snippet Collection file
  search      Searches for entries in Snippet Collection file
  config      Configures the Snippet Collection file
  update-key
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
</details>

2.  Add new snippet
```sh
# help
snipr add --help
# Add Snippet
snipr add --key <key> --value <value> --description <description> -- "<snippet>"
```

3. Remove snippet

```sh
# help
snipr remove --help
# Remove Snippet
snipr remove --key <key>
```

4. List all snippets

```sh
# help
snipr ls --help
# Usage
snipr ls <LIST_OPTION | (key or prefix)>
# List all Keys
snipr ls key
# List all Prefixes
snipr ls prefix
```

<details>
<summary>Output</summary>

```sh
snipr ls key
/Users/uriah/.config/nvim/snippets/rust/rust.json
[src/main.rs:468] list_option = Key
impl_iterator
serialize_to_json_string
impl_add_trait
impl_vec_iterator
unwrap_or_else
impl_deref
impl_debug_single_field
deserialize_json_string
impl_display_single_field
import_serde_traits
impl_clone_single_field
```
</details>

5.  Update Key

```sh
# help
snipr update-key --help
# Update Key
snipr update-key  --old-key <old-key> --new-key <new-key>
```

<details>
<summary>Output</summary>

```sh
snipr update-key --old-key "Fuzz match String" --new-key "fuzzy-match-string"
/Users/uriah/.config/nvim/snippets/rust/rust.json
[src/main.rs:499] &old_key = "Fuzz match String"
[src/main.rs:499] &new_key = "fuzzy-match-string"
```

</details>

6. Update Snippet

```sh
# help
snipr edit --help
# Update snippet Value
snipr edit --key <key> --prefix <prefix> --description <description> -- "<snippet>"
```

7. Search Snippet

```sh
# help
snipr search --help
# Search Snippet
snipr search <ID | (key or prefix)> -- "<search_term>"
```

<details>
<summary>Output</summary>

```sh
snipr search key -- impl
/Users/uriah/.config/nvim/snippets/rust/rust.json
[src/main.rs:490] id = Some(
    Key,
)
[src/main.rs:490] &name = "impl"
impl_deref

impl_clone_single_field

impl_iter_range

impl_partialeq_single_field
```
</details>


8. Show Snippet

```sh
# help
snipr show --help
# Show Snippet
snipr show <key_id>
```

<details>
<summary>Output</summary>

```sh
snipr show impl_deref
/Users/uriah/.config/nvim/snippets/rust/rust.json
[src/main.rs:484] &key = "impl_deref"
+-------------+-------------------------------------------------------------+
| Key         | impl_deref                                                  |
+-------------+-------------------------------------------------------------+
| Prefix      | impl_deref                                                  |
+-------------+-------------------------------------------------------------+
| Description | Impl Deref and DerefMut traits for a custom type            |
+-------------+-------------------------------------------------------------+
| Body        | use std::ops::{Deref, DerefMut};                            |
|             |                                                             |
|             | impl<${1:T}> Deref for ${2:YourConcreteStruct}<${1:T}> {    |
|             |     type Target = ${1:T};                                   |
|             |     fn deref(&self) -> &Self::Target {                      |
|             |         &self.${3:your_field}                               |
|             |     }                                                       |
|             | }                                                           |
|             |                                                             |
|             | impl<${1:T}> DerefMut for ${2:YourConcreteStruct}<${1:T}> { |
|             |     fn deref_mut(&mut self) -> &mut Self::Target {          |
|             |         &mut self.${3:your_field}                           |
|             |     }                                                       |
|             | }                                                           |
+-------------+-------------------------------------------------------------+
````
</details>

9. Config Snippet
Note: This can be used to switch Configuration e.g. you wanna manage Python Snippet , you can do just pass in the PATH to that configuration file.

```sh
# help
snipr config --help
# Config Snippet
snipr config <path>
```
