use lux_lib::config::ConfigBuilder;
use mlua::{ExternalResult, Lua, Table};

pub fn config(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "default",
        lua.create_function(|_, ()| ConfigBuilder::default().build().into_lua_err())?,
    )?;

    table.set(
        "builder",
        lua.create_function(|_, ()| Ok(ConfigBuilder::default()))?,
    )?;

    Ok(table)
}
