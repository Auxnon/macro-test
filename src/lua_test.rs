use mlua::{Function, Lua, UserData, UserDataMethods};
use std::{fs, path::Path};

struct Testo(f32);
impl UserData for Testo {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        //methods.add_method("remote_addr", |_lua, req, ()| Ok((req.0).to_string()));
        //methods.add_method("method", |_lua, req, ()| Ok((req.1).method().to_string()));

        // methods.add_method("magnitude", |_, vec, ()| {
        //     let mag_squared = vec.0 * vec.0 + vec.1 * vec.1;
        //     Ok(mag_squared.sqrt())
        // });
        methods.add_method("go", |_, v, ()| {
            //let mag_squared = vec.0 * vec.0 + vec.1 * vec.1;

            Ok(v.0 * 10.)
        });
    }
}
pub fn test_lua() {
    let lua = Lua::new();
    // let testo = Testo(5.);
    //testo.
    let globals = lua.globals();
    let func = lua.create_function(|_, (x, y): (f32, f32)| Ok(x * y));
    if func.is_err() {
        return;
    }
    globals.set("go", func.unwrap());

    // let go_method = lua.create_function(|lua, ()| {
    //     let t = lua.create_table()?;
    //     t.set(1, 1)?;
    //     t.set(2, 2)?;
    //     Ok(t)
    // }).unwrap();

    //lua.add_method("get")
    let input_path = Path::new(".").join("entities").join("scripty.lua");
    let st = fs::read_to_string(input_path).unwrap_or_default();
    let code = lua.load(&st);
    //code.
    match code.eval::<i32>() {
        Ok(o) => println!("lua loaded and got {}", o),
        Err(e) => println!("uh oh stinky {}", e),
    }
}
