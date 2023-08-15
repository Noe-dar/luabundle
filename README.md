# Luabundle
Powerful CLI tool designed to bundling Lua code into a single file

## Installation

### Obtain a Prebuilt Binary
Download the prebuilt binary compatible with your platform from the [releases page](https://github.com/Noe-dar/luabundle/releases)

### Build yourself

```sh
git clone github.com/Noe-dar/luabundle
cd luabundle
cargo build --release
```

The resulting binary will be located in `target/release`
## Usage

``` 
Usage: luabundle <input> [-o <output>]

Positional Arguments:
  input             path to the input file

Options:
  -o, --output      name of the output file
  --help            display usage information
```
For example, you can bundle the `input.lua` file along with its dependencies using the following command:
```sh
luabundle -o bundle.lua input.lua
```