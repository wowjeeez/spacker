use std::sync::Mutex;
use lazy_static::lazy_static;
use rlua::{Function, Table};
use crate::rt::Context;
use crate::rt::tablegen::populate_lua_tbl;
use colored::Colorize;

lazy_static! {
    static ref ENTRIES: Mutex<Vec<String>> = Mutex::new(vec![]);
}
impl Context {
    pub fn load_directives(&self) {
        self.override_print();
        let static_funcs = include_str!("ignored.lua");
        self.lua.context(|cx| {
            cx.load(static_funcs).exec().unwrap(); //my lua file is guaranteed to work
            let mock_creator: Function = cx.globals().get("__CREATE_MOCK_FROM_TBL").unwrap();
            let mut tbl = cx.create_table().unwrap();
            populate_lua_tbl(&mut tbl, vec!["fx_version", "author", "game",
                                                    "description", "authors", "dependency",
                                                    "resource_manifest_version", "export",
                                                    "server_export", "this_is_a_map", "server_only",
                                                    "provide", "clr_disable_task_scheduler", "lua54"]);
            cx.globals().set("__FUNCS", tbl).unwrap();
            mock_creator.call::<_, ()>(()).unwrap();
            self.load_interested_funcs();
            self.lua.context(|cx| { //super smart logic right here to allow calling arbitrary metadata keys
                cx.load(r#"local proxiedMeta = {}
for k,v in pairs(_G) do
    proxiedMeta[k] = v
end

_G = setmetatable(_G, {
    __index = function(tbl, key)
        if proxiedMeta[key] then
            return proxiedMeta[key]
        end
        print('Call to arbitrary key, creating mocked function:', key)
        proxiedMeta[key] = function() return function() end end
        return proxiedMeta[key]
    end
})"#).exec().unwrap();
            })
        })
    }
    pub fn get_files(&self) -> Vec<String> {
        let vec = ENTRIES.lock().unwrap().to_vec();
        vec
    }

    fn load_interested_funcs(&self) {
        self.lua.context(|cx| {
            let push_tbl = cx.create_function(|_, tbl: Table| {
                let mut lck = ENTRIES.lock().unwrap();
                for pair in tbl.pairs::<usize, String>() {
                    let (_, file) = pair.unwrap();
                    lck.push(file);
                }
                rlua::Result::Ok(())
            }).unwrap();
            cx.globals().set("__PUSH_TBL", push_tbl).unwrap();
            let extractor: Function = cx.globals().get("__CREATE_EXTRACTOR_FROM_TBL").unwrap();
            let mut tbl = cx.create_table().unwrap();
            populate_lua_tbl(&mut tbl, vec!["shared_script", "shared_scripts", "client_script",
                                                    "client_scripts", "server_script", "server_scripts",
                                                    "ui_page", "files", "data_files", "data_file",
                                                    "file", "loadscreen", "before_level_meta", "after_level_meta", "replace_level_meta", "spacker_keep"]);
            cx.globals().set("__COLLECTORS", tbl).unwrap();
            extractor.call::<_, ()>(()).unwrap();
        });
    }

    fn override_print(&self) {
        self.lua.context(|cx| {
            let print = cx.create_function(|_, args: String| {
                println!("{}", format!("[LUA]: {}", args).blue());
                rlua::Result::Ok(())
            }).unwrap();
            cx.globals().set("__PRINT", print).unwrap();
        })
    }
}