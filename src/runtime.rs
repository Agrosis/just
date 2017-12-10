
struct Heap {

}

#[derive(Debug)]
pub struct StackFrame {
    operands: Vec<Operand>,
    local_variables: Vec<u32>
}

impl StackFrame {

    pub fn new() -> StackFrame {
        StackFrame {
            operands: Vec::new(),
            local_variables: Vec::new()
        }
    }

    pub fn push_operand(&mut self, operand: Operand) -> () {
        self.operands.push(operand)
    }

    pub fn pop_operand(&mut self) -> Option<Operand> {
        self.operands.pop()
    }

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
