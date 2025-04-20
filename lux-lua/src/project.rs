use std::path::PathBuf;

use lux_lib::project::Project;
use mlua::{ExternalResult, Lua, Table};

pub fn project(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "current",
        lua.create_function(|_, ()| Project::current().into_lua_err())?,
    )?;

    table.set(
        "new",
        lua.create_function(|_, path: PathBuf| Project::from(path).into_lua_err())?,
    )?;

    Ok(table)
}
