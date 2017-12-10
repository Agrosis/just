
// 0: iconst_3
// 1: istore_1
// 2: iload_1
// 3: iconst_3
// 4: iadd
// 5: istore_2
// 6: return

mod jvm;
mod runtime;

use jvm::opcode;
use jvm::opcode::Opcode;

fn main() {
    let program: Vec<u8> = vec![
        Opcode::Iconst3 as u8,
        Opcode::Istore1 as u8,
        Opcode::Iload1 as u8,
        Opcode::Iconst3 as u8,
        Opcode::Imul as u8,
        Opcode::Istore2 as u8
    ];

    println!("JUST - JVM implementation");

    let mut pc = 0;
    let mut frame = runtime::StackFrame::new();

    loop {
        let byte = program[pc];
        println!("Executing instruction {}", byte);

        let opcode_opt = opcode::from_u8(byte);

        match opcode_opt {
            Some(opcode) => {
                match opcode {
                    Opcode::Iadd => {
                        let x = frame.pop_operand();
                        let y = frame.pop_operand();

                        match (x, y) {
                            (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                                let result = x_value + y_value;

                                frame.push_operand(runtime::Operand::Int(result));
                            },
                            (_, _) => panic!("Invalid IADD operands.")
                        }
                    },
                    Opcode::Iconst0 => {
                        frame.push_operand(runtime::Operand::Int(0));
                    },
                    Opcode::Iconst1 => {
                        frame.push_operand(runtime::Operand::Int(1));
                    },
                    Opcode::Iconst2 => {
                        frame.push_operand(runtime::Operand::Int(2));
                    },
                    Opcode::Iconst3 => {
                        frame.push_operand(runtime::Operand::Int(3));
                    },
                    Opcode::Iconst4 => {
                        frame.push_operand(runtime::Operand::Int(4));
                    },
                    Opcode::Iconst5 => {
                        frame.push_operand(runtime::Operand::Int(5));
                    },
                    Opcode::Imul => {
                        let x = frame.pop_operand();
                        let y = frame.pop_operand();

                        match (x, y) {
                            (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                                let result = x_value * y_value;

                                frame.push_operand(runtime::Operand::Int(result));
                            },
                            (_, _) => panic!("Invalid IMUL operands.")
                        }
                    },
                    Opcode::Isub => {
                        let x = frame.pop_operand();
                        let y = frame.pop_operand();

                        match (x, y) {
                            (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                                let result = x_value - y_value;

                                frame.push_operand(runtime::Operand::Int(result));
                            },
                            (_, _) => panic!("Invalid ISUB operands.")
                        }
                    },
                    _ => panic!("Unsupported opcode {:?}.", opcode)
                }
            },
            None => panic!("Invalid opcode {}.", byte)
        }

        pc += 1;
        if pc == program.len() {
            break;
        }
    }

    println!("{:?}", frame);

    println!("Completed executing program.");
}
