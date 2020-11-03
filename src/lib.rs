//! This crate lets you `#[derive(UserData)]` on structs.
//! The derived implementation reigsters the `Index` and `NewIndex` metamethods
//! on selected fields, so Lua code can use the fields.
//!
//! # Basic Usage
//!
//! ```
//! #[derive(UserData)]
//! struct Exampler {
//!     /// Annotate fields with #[userdata] for default values
//!     /// (allowing read & write)
//!     field: i32,
//!     /// make it read-only
//!     #[userdata(read)]
//!     read_only: f64,
//!     /// Change the name
//!     #[userdata(rename = "exciting_name")]
//!     boring_name: bool,
//! }
//! ```
//!
//! Tuple structs are accessed by their indices (barring overwriting names).
//!
//! ```
//! #[derive(UserData)]
//! struct Tupler (
//!     u8,
//!     u16,
//!     #[userdata(rename = "jeremy")]
//!     bool,
//!     u32
//! );
//! ```
//!
//! In Lua, `tupler[1]` is a `u8`, `tupler[2]` is a `u16`,
//! and `tupler[4]` is a `u32`. `tupler[3]` does not exist; you have to use `tupler.jeremy`.
//!
//! Note these are 1-indexed.
//!
//! # Cloning
//!
//! The Index method (reading a value) ALWAYS CLONES THE FIELD. If you don't want that, you can
//! use my [handy-dandy PR for rlua!](https://github.com/gamma-delta/rlua-arcmux-userdata)
//! It lets `Arc<Mutex<T>> where T: 'static + UserData + Default` implement UserData
//! and pass through all the metamethods.

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use rlua::{Lua, MetaMethod, UserData, UserDataMethods, Value};
    use rud_proc_macro::UserData;

    #[test]
    fn attributes() {
        #[derive(UserData)]
        struct Tester {
            #[userdata]
            field: u32,
            #[userdata(read)]
            read_only: String,
            #[userdata(rename = "hello", read)]
            hellont: isize,
        }
        let _s = Tester {
            field: 10,
            read_only: String::from("read only?!"),
            hellont: -44,
        };
    }

    #[test]
    fn nested() {
        #[derive(UserData)]
        struct Foo {
            #[userdata]
            bar: Arc<Mutex<Bar>>,
        }
        #[derive(UserData, Default)]
        struct Bar {
            #[userdata]
            value: i32,
        }

        let lua = Lua::new();
        lua.context(|ctx| -> rlua::Result<()> {
            let globals = ctx.globals();
            let foo = Foo {
                bar: Arc::new(Mutex::new(Bar { value: 0 })),
            };
            globals.set("foo", foo).unwrap();

            let ending_val = ctx
                .load(
                    r#"
                assert(foo.bar.value == 0)
                foo.bar.value = 10
                assert(foo.bar.value == 10)

                -- this clones the Arc
                local bar_alias = foo.bar
                for i = 1,1000 do
                    bar_alias.value = i
                    assert(foo.bar.value == i)
                end

                return foo.bar.value
            "#,
                )
                .eval::<i32>()
                .unwrap();
            assert_eq!(ending_val, 1000);

            Ok(())
        })
        .unwrap();
    }
}
