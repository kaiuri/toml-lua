# toml-lua (WIP)

> LuaJIT only, for now.

TOML encoding and decoding for LuaJIT through Rust's `serde` and `toml` crates.

## Installation

For the moment, clone the repo, run `bash build` from the root, grab `toml.so` and put it somewhere in your `package.path`.

## Usage

### Encode

Returns `userdata` on error.

```lua
local toml = require('toml')
local data = {
  name = 'Toml',
  age = 42,
  is_cool = true,
  likes = { 'lua', 'toml', 'neovim' },
  contact = {
    email = 'bahblablabubahbla@blabla.com'
  }
}
print(toml.encode(data))
-- age = 42
-- is_cool = true
-- likes = ["lua", "toml", "neovim"]
-- name = "Toml"
--
-- [contact]
-- email = "bahblablabubahbla@blabla.com"
```

### Decode

Returns `userdata` on error.

```lua
local decode = toml.decode([[
age = 42
is_cool = true
likes = ["lua", "toml", "neovim"]
name = "Toml"
[contact]
email = "bahblablabubahbla@blabla.com"
]])
vim.print(decode)
-- {
--   age = 42,
--   contact = {
--     email = "bahblablabubahbla@blabla.com"
--   },
--   is_cool = true,
--   likes = { "lua", "toml", "neovim" },
--   name = "Toml"
-- }
```
