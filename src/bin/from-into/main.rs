use std::convert::Into;

struct MYStruct {
    some_data: String,
}

trait SomeMethods {
    fn do_smth() -> () {}
}

impl SomeMethods for MYStruct {}

impl<T> Into<T> for i32
where
    T: SomeMethods,
{
    fn into<T>(self: i32) -> T {
        T {
            some_data: self.to_string(),
        }
    }
}

fn main() {
    let s: MYStruct = 32i32.into();
}
