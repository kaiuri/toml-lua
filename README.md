# toml-lua (WIP)

> Note: For the moment this is for usage from within Neovim only.

TOML encoding and decoding for LuaJIT through Rust's `serde` and `toml` crates.

## Installation

### Neovim

- `lazy.nvim`:

```lua
{
  [1] = 'kaiuri/toml-lua',
  build = 'bash build'
  lazy = false -- ??
}
```

### Elsewhere

For the moment, clone the repo, run `bash build` from the root and grab
`./lua/toml.so` and put somewhere in your `package.path`.

## Usage

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
local decode = toml.decode([[
age = 42 # aha, nope 😆
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
