use shared::interpreter::OpCode0;

pub fn length() -> Vec<OpCode0> {
    use OpCode0::*;
    vec![Pushx, Square, Pushy, Square, Add, Sqrt]
}

pub fn disk(r: f32) -> Vec<OpCode0> {
    use OpCode0::*;
    let mut vec = length();
    vec.extend(vec![Push(r), Sub]);
    vec
}

pub fn sdf() -> Vec<OpCode0> {
    disk(0.3)
}
