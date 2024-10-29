use mlua::prelude::*;
use mlua::{Lua, LuaSerdeExt, MultiValue, Result, Value};
use serde::Serialize;
// lua multi/single value return
macro_rules! ret {
    // returns a single value
    ($lua:expr, $val:expr) => {
        $lua.pack_multi($lua.to_value($val)?)
    };
    // returns multiple values
    ($lua:expr, $($val:expr),*) => {
        $lua.pack_multi(($($lua.to_value($val)?),*))
    };
}

/// decode toml string to lua table.
fn decode<'lua>(lua: &'lua Lua, src: Value<'lua>) -> Result<MultiValue<'lua>> {
    match src {
        Value::String(ref ptr) => match ptr.to_str() {
            Ok(s) => match toml::from_str::<toml::Value>(s) {
                Ok(v) => ret!(lua, &v),
                Err(_e) => ret!(lua, "Error parsing toml string"),
            },
            Err(_) => ret!(lua, "Error converting value to string"),
        },
        _ => ret!(lua, "Error converting value to string"),
    }
}
// returns either a value or nil and an error message
fn decode_file<'lua>(lua: &'lua Lua, src: Value<'lua>) -> Result<MultiValue<'lua>> {
    match src {
        Value::String(ref ptr) => match ptr.to_str() {
            Ok(path) => match std::fs::read_to_string(path) {
                Ok(file) => match toml::from_str::<toml::Value>(&file) {
                    Ok(v) => lua.pack_multi(lua.to_value(&v)),
                    Err(_e) => ret!(lua, "Error parsing toml file"),
                },
                Err(_) => ret!(lua, "Error reading file"),
            },
            Err(_) => ret!(lua, "Error converting value to string"),
        },
        _ => ret!(lua, "Error converting value to string"),
    }
}

/// encode lua table to toml string.
fn encode<'lua>(lua: &'lua Lua, v: Value<'lua>) -> Result<MultiValue<'lua>> {
    let mut buf = String::new();
    match v.serialize(toml::Serializer::new(&mut buf)) {
        Ok(()) => lua.pack_multi(lua.create_string(&buf).map(Value::String)),
        Err(_) => ret!(lua, "Error serializing lua table to toml string"),
    }
}

/// toml-lua module.
/// provides toml encode and decode functions.
///
/// ```lua
/// require("toml").decode("a = 1") -- {a = 1}
/// require("toml").encode({a = 1}) -- "a = 1\n"
/// ```
#[mlua::lua_module(name = "toml")]
fn toml(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("decode", lua.create_function(decode)?)?;
    exports.set("encode", lua.create_function(encode)?)?;
    exports.set("decode_file", lua.create_function(decode_file)?)?;
    Ok(exports)
}
