pub enum Value<'a> {
    Str (&'a str),
    Bool (bool),
    I32 (i32),
    U32 (u32),
    I64 (i64),
    U64 (u64),
    F32 (f32),
    F64 (f64),
    NoOp
}
