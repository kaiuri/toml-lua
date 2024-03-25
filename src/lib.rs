use mlua::prelude::*;
use mlua::{Lua, LuaSerdeExt, Result, Value};
use serde::Serialize;

/// decode toml string to lua table.
fn decode<'lua>(lua: &'lua Lua, src: Value<'lua>) -> Result<Value<'lua>> {
    let s = match src {
        Value::String(ref v) => v,
        _ => {
            return lua.to_value("expected string");
        }
    };
    match toml::from_str::<toml::Value>(s.to_str()?) {
        Ok(v) => lua.to_value(&v),
        Err(e) => lua.to_value(&e.to_string()),
    }
}

/// encode lua table to toml string.
fn encode<'lua>(lua: &'lua Lua, v: Value<'lua>) -> Result<Value<'lua>> {
    let mut buf = String::new();
    match v.serialize(toml::Serializer::new(&mut buf)) {
        Ok(()) => lua.create_string(&buf).map(Value::String),
        Err(e) => lua.to_value(&e.to_string()),
    }
}

/// toml-lua module.
/// provides toml encode and decode functions.
///
/// ```lua
/// require("toml").decode("a = 1") -- {a = 1}
/// require("toml").encode({a = 1}) -- "a = 1\n"
/// ```
#[mlua::lua_module]
fn toml(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("decode", lua.create_function(decode)?)?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
