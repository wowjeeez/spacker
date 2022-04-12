use rlua::{Lua};
use colored::Colorize;
pub struct Context {
    pub lua: Lua
}

impl Context {
    pub fn new() -> Context {
        Context {lua: Lua::new()}
    }

    pub fn load_str(&self, file_str: String) {
        self.lua.context(|cx| {
            let load_res = cx.load(&file_str).set_name("fxmanifest.lua");
            if load_res.is_ok() {
                let call_res = load_res.unwrap().exec();
                if call_res.is_ok() {
                    println!("{}", "fxmanifest.lua executed successfully!".green());
                } else {
                    let err = call_res.err().unwrap().to_string();
                    println!("{}", format!("Failed to execute fxmanifest.lua due to: \n{}", err).red());
                }
            } else {
                let err = load_res.err().unwrap().to_string();
                println!("{}", format!("Failed to load fxmanifest.lua into runtime due to: {}", err).red());
                std::process::exit(1);
            }
        })
    }
}