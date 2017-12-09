
struct Heap {

}

#[derive(Debug)]
pub struct StackFrame {
    pub operands: Vec<Operand>
}

impl StackFrame {

}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Operand {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    Boolean(bool)
}
