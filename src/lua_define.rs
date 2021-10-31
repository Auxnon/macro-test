use crate::LuaEnt;
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

        // let make_ent = lua.create_function(|_, (x, y): (f32, f32)| Ok(LuaEnt { x, y }));
        // globals.set("make_ent", make_ent.unwrap());

        // let default_func = lua
        //     .create_function(|_, e: crate::entity::LuaEnt| Ok(e))
        //     .unwrap();
        let default_func = lua.create_function(|_, e: f32| Ok(e)).unwrap();
        globals.set("default_func", default_func);
        drop(globals);

        LuaCore {
            lua,
            map: HashMap::new(),
        }
    }

    pub fn load(&self, str: String) {
        let input_path = Path::new(".")
            .join("scripts")
            .join(str.to_owned())
            .with_extension("lua");
        let st = fs::read_to_string(input_path).unwrap_or_default();
        println!("::lua:: got script {} :\n{}", str, st);
        let chunk = self.lua.load(&st);
        let globals = self.lua.globals();
        //chunk.eval()
        //let d= chunk.eval::<mlua::Chunk>();

        match chunk.eval::<mlua::Function>() {
            Ok(code) => {
                println!("::lua:: code loaded 📜{} ♥", str);
                globals.set(str, code);
            }
            Err(err) => {
                println!(
                    "::lua::  bad lua code for 📜{} !! Assigning default \"{}\"",
                    str, err
                );
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
            let res2 = globals.get::<_, Function>(str.to_owned());
            if res2.is_err() {
                println!(
                    "::lua:: failed to get lua code for 📜{} even after default func",
                    str
                );
            }
            println!(
                "::lua:: we didnt find lua code so we loaded it and returned it for 📜{}",
                str
            );
            res2.unwrap()
        } else {
            println!("::lua::we got and returned a func for {}", str);
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
