use crate::{
    ent::{self, Ent},
    ent_factory::{self, EntFactory},
    LuaEnt,
};
use mlua::{Function, Lua, Scope, UserData, UserDataMethods};
use once_cell::sync::OnceCell;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::Path,
    process::exit,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct LuaCore<'a> {
    pub lua: mlua::Lua,
    map: HashMap<String, Function<'a>>,
}

impl<'a, 'scope> LuaCore<'a> {
    pub fn new() -> LuaCore<'a> {
        let lua = Lua::new();

        LuaCore {
            lua,
            map: HashMap::new(),
        }
    }

    pub fn init(
        &'a self,
        scope: &'a Scope<'a, 'scope>,
        entity_factory: &'a EntFactory,
        meshes: Rc<RefCell<Vec<Ent<'a>>>>,
    ) -> &Scope<'a, 'scope> {
        let globals = self.lua.globals();

        let multi = self.lua.create_function(|_, (x, y): (f32, f32)| Ok(x * y));
        // let t = multi.unwrap();
        globals.set("multi", multi.unwrap());

        // let closure = |_, (str, x, y): (String, f32, f32)| {
        //     let mut ent = entity_factory.create_ent(&str, self);
        //     ent.pos.x = x;
        //     ent.pos.y = y;
        //     let lua_ent = ent.to_lua();
        //     let mut m = meshes.borrow_mut();
        //     m.push(ent);
        //     println!("added ent, now sized at {}", m.len());
        //     Ok(lua_ent)
        //     //Ok(&ent.to_lua())
        // };

        // globals.set("spawn", {
        //     let m = scope.create_function(closure);
        //     m.unwrap()
        // });

        let default_func = self.lua.create_function(|_, e: f32| Ok(e)).unwrap();
        globals.set("default_func", default_func);
        drop(globals);
        scope
    }

    pub fn test(&self) {}
    pub fn spawn(&'a self, str: &String) {
        //entity_factory.create_ent(str, self);
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

pub fn scope_test<'b, 'scope>(
    scope: &Scope<'scope, 'b>,
    ent_factory: &'b EntFactory,
    lua_core: &'b LuaCore,
    meshes: Rc<RefCell<Vec<Ent<'b>>>>,
) {
    let closure = move |_, (str, x, y): (String, f32, f32)| {
        let mut ent = ent_factory.create_ent(&str, &lua_core);
        ent.pos.x = x;
        ent.pos.y = y;
        let lua_ent = ent.to_lua();
        let res = meshes.try_borrow_mut();
        if res.is_ok() {
            let mut m = res.unwrap();
            m.push(ent);
            println!("added ent, now sized at {}", m.len());
        } else {
            println!("cannot add ent, overworked!")
        }
        Ok(lua_ent)
        //Ok(&ent.to_lua())
    };

    let lua_globals = lua_core.lua.globals();
    lua_globals.set("spawn", {
        let m = scope.create_function(closure);
        m.unwrap()
    });
}
