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

        let default_func = lua.create_function(|_, (x): (f32)| Ok(x)).unwrap();
        globals.set("default_func", default_func);
        drop(globals);

        LuaCore {
            lua,
            map: HashMap::new(),
        }
    }
    pub fn load(&self, str: String) -> Function {
        match self.map.get(&str) {
            Some(out) => *out,
            None => {
                let input_path = Path::new(".").join("entities").join("scripty.lua");
                let st = fs::read_to_string(input_path).unwrap_or_default();
                let chunk = self.lua.load(&st);
                let h = match chunk.into_function() {
                    Ok(code) => {
                        self.map.insert(str.clone(), code);
                        self.map.get(&str).unwrap()
                    }
                    Err(err) => {
                        println!("bad lua code {} !!", str);
                        let globals = self.lua.globals();
                        let d: Function = globals.get("default_func").unwrap()
                        
                    }
                };
                h
            }
        }
        //let out = self.lua.globals().get("default_func").unwrap();
        //out
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
