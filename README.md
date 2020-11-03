# rlua-userdata-derive

This crate lets you `#[derive(UserData)]` on structs.
The derived implementation reigsters the `Index` and `NewIndex` metamethods
on selected fields, so Lua code can use the fields.

## Basic Usage

```rust
#[derive(UserData)]
struct Exampler {
    /// Annotate fields with #[userdata] for default values
    /// (allowing read & write)
    field: i32,
    /// make it read-only
    #[userdata(read)]
    read_only: f64,
    /// Change the name
    #[userdata(rename = "exciting_name")]
    boring_name: bool,
}
```

Tuple structs are accessed by their indices (barring overwriting names).

```rust
#[derive(UserData)]
struct Tupler (
    u8,
    u16,
    #[userdata(rename = "jeremy")]
    bool,
    u32
);
```

In Lua, `tupler[1]` is a `u8`, `tupler[2]` is a `u16`,
and `tupler[4]` is a `u32`. `tupler[3]` does not exist; you have to use `tupler.jeremy`.

Note these are 1-indexed.

## Cloning

The Index method (reading a value) ALWAYS CLONES THE FIELD. If you don't want that, you can
use my [handy-dandy PR for rlua!](https://github.com/gamma-delta/rlua-arcmux-userdata)
It lets `Arc<Mutex<T>> where T: 'static + UserData + Default` implement UserData
and pass through all the metamethods.


## License

MIT