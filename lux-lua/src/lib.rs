use mlua::prelude::*;

mod config;
mod loader;
mod project;

#[mlua::lua_module]
fn lux(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "loader",
        lua.create_function(|lua, ()| loader::load_loader(lua))?,
    )?;
    exports.set("config", config::config(lua)?)?;
    exports.set("project", project::project(lua)?)?;

    Ok(exports)
}
