use mlua::prelude::*;
use serde::Serialize;

/// Decode toml string to lua table.
fn decode(lua: &mlua::Lua, src: mlua::String) -> mlua::Result<mlua::MultiValue> {
    match src.to_str() {
        Ok(s) => match toml::from_str::<toml::Value>(&s) {
            Ok(v) => lua.pack_multi(lua.to_value(&v)?),
            Err(_e) => lua.pack_multi((&LuaNil, lua.create_string("Error parsing toml string."))),
        },
        _ => lua.pack_multi((&LuaNil, lua.create_string("Invalid string."))),
    }
}

/// Encode lua table to toml string.
fn encode(lua: &mlua::Lua, v: mlua::Value) -> mlua::Result<mlua::MultiValue> {
    let mut buf = String::new();
    match v.serialize(toml::Serializer::new(&mut buf)) {
        Ok(()) => lua.pack_multi(lua.create_string(&buf)),
        Err(_) => lua.pack_multi((
            &LuaNil,
            lua.create_string("Error serializing lua table to toml string"),
        )),
    }
}

/// toml-lua module.
/// provides toml encode and decode functions.
///
/// ```lua
/// require("toml").decode("a = 1") -- {a = 1}
/// require("toml").encode({a = 1}) -- "a = 1\n"
/// ```
#[mlua::lua_module(name = "toml_lua")]
fn toml(lua: &mlua::Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("decode", lua.create_function(decode)?)?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
