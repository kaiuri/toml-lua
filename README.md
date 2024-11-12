# toml-lua (WIP)

Library for encoding and decoding TOML in LuaJIT through Rust's `serde` and `toml` crates.

# Installation

Don't you do it, but if you do

```sh
git clone -b main main https://github.com/kaiuri/toml-lua.git toml-lua
cd toml-lua
make all
# then put that toml_lua.so somewhere in your package.cpath
```

# Usage

```lua
--- # toml-lua (WIP), LuaJIT only, for now.
--- TOML encoding and decoding for LuaJIT through Rust's `serde` and `toml` crates.

--- ## Installation
--- For the moment, clone the repo, run `make all` and put `toml.so` somewhere
--- in your package.cpath

--- Usage
local inspect = require('inspect')
---@class toml
---@field encode fun(obj: table): string?, string?
---@field decode fun(str: string): table?, string?
---@field decode_file fun(path: string): table?, string?
local toml = require('toml_lua')
local data = {
  name = 'Toml',
  age = 42,
  is_cool = true,
  likes = { 'lua', 'toml', 'neovim' },
  contact = {
    email = 'bahblablabubahbla@blabla.com'
  }
}

local out = assert(toml.encode {
  name = 'Toml',
  age = 42,
  is_cool = true,
  likes = { 'lua', 'toml', 'neovim' },
  contact = {
    email = 'bahblablabubahbla@blabla.com'
  }
})
print(out)
--> age = 42
--> is_cool = true
--> likes = ["lua", "toml", "neovim"]
--> name = "Toml"
-->
--> [contact]
--> email = "bahblablabubahbla@blabla.com"

local decoded = assert(toml.decode([[
age = 42
is_cool = true
likes = ["lua", "toml", "neovim"]
name = "Toml"
[contact]
email = "bahblablabubahbla@blabla.com"
]]))
print(inspect(decoded))
--> {
-->   age = 42,
-->   contact = {
-->     email = "bahblablabubahbla@blabla.com"
-->   },
-->   is_cool = true,
-->   likes = { "lua", "toml", "neovim" },
-->   name = "Toml"
--> }
```
