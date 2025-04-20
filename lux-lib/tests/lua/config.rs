#[cfg(test)]
mod tests {
    use lux_lib::config::ConfigBuilder;
    use mlua::prelude::*;

    #[test]
    fn lua_api_test_config() {
        let lua = Lua::new();

        lua.globals()
            .set("lux_config", ConfigBuilder::new().unwrap().build().unwrap())
            .unwrap();

        lua.load(
            r#"
            local config = lux_config
            local default = config.default()
            assert(default, "default config should not be nil")
            "#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn lua_api_test_config_builder() {
        let lua = Lua::new();
        let tree = assert_fs::TempDir::new().unwrap();
        let cache = assert_fs::TempDir::new().unwrap();
        let data = assert_fs::TempDir::new().unwrap();

        lua.globals()
            .set("lux_config", ConfigBuilder::new().unwrap().build().unwrap())
            .unwrap();
        lua.globals().set("tree", tree.path()).unwrap();
        lua.globals().set("cache", cache.path()).unwrap();
        lua.globals().set("data", data.path()).unwrap();

        lua.load(
            r#"
            local config = lux_config
            local default = config.builder()
                :dev(true)
                :server("https://example.com")
                :extra_servers({"https://example.com", "https://example2.com"})
                :only_sources("example")
                :namespace("example")
                :lua_dir("lua")
                :lua_version("5.1")
                :tree(temp)
                :luarocks_tree(tree)
                :no_project(true)
                :verbose(true)
                :timeout(10)
                :cache_dir(cache)
                :data_dir(data)
                -- :entrypoint_layout("rockspec")
                :build()

            assert(default, "default config should not be nil")
            assert(#default:enabled_dev_servers() > 0, "enabled_dev_servers should not be empty")
            assert(default:server() == "https://example.com/", "server should be https://example.com")
            assert(#default:extra_servers() == 2, "extra_servers should have 2 elements")
            assert(default:extra_servers()[1] == "https://example.com/", "first extra server should be https://example.com")
            assert(default:extra_servers()[2] == "https://example2.com/", "second extra server should be https://example2.com")
            assert(default:only_sources() == "example", "only_sources should be https://example.com")
            assert(default:namespace() == "example", "namespace should be example")
            assert(default:lua_dir() == "lua", "lua_dir should be lua")
            assert(default:lua_version() == "5.1", "lua_version should be 5.1")
            assert(default:tree("5.1"), "tree should be not nil")
            assert(default:luarocks_tree(), "luarocks_tree should be not nil")
            assert(default:no_project(), "no_project should be true")
            assert(default:verbose(), "verbose should be true")
            assert(default:timeout() == 10, "timeout should be 10")
            assert(default:cache_dir() == cache, "cache_dir should be /cache")
            assert(default:data_dir() == data, "data_dir should be /data")
            -- assert(default.entrypoint_layout() == ...)
            "#,
        )
        .exec()
        .unwrap();
    }
}
