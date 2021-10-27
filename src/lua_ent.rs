
use mlua::{ UserData, UserDataMethods,UserDataFields};
use crate::Ent;
pub struct LuaEnt {
    pub x: f32,
    pub y: f32,
    pub vel_x:f32,
    pub vel_y:f32
}

impl UserData for LuaEnt {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_x", |_,this, ()| 
            Ok(this.x)
        );
        methods.add_method("get_y", |_,this, ()| Ok(this.y));
        
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_set("x", |_, this,x:f32| { Ok(this.x=x)});
        
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_set("y", |_, this,y:f32| { Ok(this.y=y)});

        fields.add_field_method_get("vel_x", |_, this| Ok(this.vel_x));
        fields.add_field_method_set("vel_x", |_, this,vel_x:f32| { Ok(this.vel_x=vel_x)});
        fields.add_field_method_get("vel_y", |_, this| Ok(this.vel_y));
        fields.add_field_method_set("vel_y", |_, this,vel_y:f32| { Ok(this.vel_y=vel_y)});
    }
}
 // pub fn new() -> LuaEnt {
    //     return LuaEnt { x: 10., y: 12. };
    // }
//methods.add_method("add_x", |_, this, ()| Ok(Self.ent.set_x(10.)));

        // methods.add_meta_function(MetaMethod::Add, |_, (vec1, vec2): (Vec2, Vec2)| {
        //     Ok(Vec2(vec1.0 + vec2.0, vec1.1 + vec2.1))
        // });

impl Clone for LuaEnt {
    fn clone(&self) -> LuaEnt {
        LuaEnt {
            x: self.x,
            y: self.y,
            vel_x:self.vel_x,
            vel_y:self.vel_y
        }
    }
}

impl LuaEnt{
    fn new(ent:Ent)->LuaEnt{
        LuaEnt{
        x: ent.pos.x,
            y: ent.pos.y,
            vel_x: ent.vel.x,
            vel_y:ent.vel.y
        }
    }
}

// impl<T: IAnimalData> Animal<T> {
// impl<'b> UserData for LuaEnt<'b> {
//     fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_method("add_x", |_, this, ()| Ok(Self.ent.set_x(10.)));

//         methods.add_async_function(
//             "read",
//             |lua, (this, size): (AnyUserData, usize)| async move {
//                 let mut this = this.borrow_mut::<Self>()?;
//                 let mut buf = vec![0; size];
//                 let n = this.0.read(&mut buf).await?;
//                 buf.truncate(n);
//                 lua.create_string(&buf)
//             },
//         );

//         methods.add_async_function(
//             "write",
//             |_, (this, data): (AnyUserData, LuaString)| async move {
//                 let mut this = this.borrow_mut::<Self>()?;
//                 let n = this.0.write(&data.as_bytes()).await?;
//                 Ok(n)
//             },
//         );

//         methods.add_async_function("close", |_, this: AnyUserData| async move {
//             let mut this = this.borrow_mut::<Self>()?;
//             this.0.shutdown().await?;
//             Ok(())
//         });
//     }
// }