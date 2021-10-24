use crate::Ent;
use mlua::{Function, Lua, UserData, UserDataMethods};

use std::{collections::HashMap, fs, path::Path};

pub struct LuaCore<'a> {
    lua: mlua::Lua,
    map: HashMap<String, Function<'a>>,
}

impl<'a> LuaCore<'a> {
    pub fn new() -> LuaCore<'a> {
        //let map = HashMap::new();
        let lua = Lua::new();
        let globals = lua.globals();

        let multi = lua.create_function(|_, (x, y): (f32, f32)| Ok(x * y));
        globals.set("multi", multi.unwrap());

        let multi = lua.create_function(|_, (x, y): (f32, f32)| Ok(x * y));
        globals.set("multi", multi.unwrap());

        let default_func = lua.create_function(|_, (x): (f32)| Ok(x - 0.1)).unwrap();
        globals.set("default_func", default_func);
        drop(globals);

        LuaCore {
            lua,
            map: HashMap::new(),
        }
    }

    pub fn load(&self, str: String) {
        let input_path = Path::new(".").join("entities").join("scripty.lua");
        let st = fs::read_to_string(input_path).unwrap_or_default();
        let chunk = self.lua.load(&st);
        let globals = self.lua.globals();

        match chunk.into_function() {
            Ok(code) => {
                println!("::lua:: code loaded {} ♥", str);
                globals.set(str, code);
            }
            Err(err) => {
                println!("::lua::  bad lua code {} !! Assigning default", str);
                globals.set(str, globals.get::<_, Function>("default_func").unwrap());
            }
        }
    }

    //let out = self.lua.globals().get("default_func").unwrap();
    //out

    pub fn get(&self, str: String) -> Function {
        let globals = self.lua.globals();
        //let version = globals.get::<_, String>("_VERSION").unwrap();
        let res = globals.get::<_, Function>(str.to_owned());
        if res.is_err() {
            self.load(str.to_owned());
            let res2 = globals.get::<_, Function>(str);
            if res2.is_err() {}
            res2.unwrap()
        } else {
            res.unwrap()
        }
        //let res: Function = globals.get::<_, Function>("default_func").unwrap();
    }

    // pub fn get(&'a self, str: String) -> &Function {
    //     // self.map.insert("foo".to_string(), "bar".to_string());

    //     // let result = self.load(str);
    //     // let out;
    //     // if result.is_none(){

    //     match self.map.get(&str) {
    //         Some(out) => out,
    //         None => {
    //             &self.load(str).unwrap()
    //             //&g.unwrap()
    //         }
    //     }
    //     //|e: Ent| {func.call(0)}
    //     //defau
    // }

    //pub fn entity_run(ent: &mut Ent, delta: f32) {}
}
