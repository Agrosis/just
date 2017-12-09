
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
    let mut frame = runtime::StackFrame {
        operands: Vec::new()
    };

    loop {
        println!("Executing instruction {}", program[pc]);

        let opcode = program[pc];


        if opcode == Opcode::Iadd as u8 {
            let x = frame.operands.pop();
            let y = frame.operands.pop();

            match (x, y) {
                (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                    let result = x_value + y_value;

                    frame.operands.push(runtime::Operand::Int(result));
                },
                (_, _) => panic!("Invalid IADD operands.")
            }
        } else if opcode == Opcode::Iconst0 as u8 {
            frame.operands.push(runtime::Operand::Int(0));
        } else if opcode == Opcode::Iconst1 as u8 {
            frame.operands.push(runtime::Operand::Int(1));
        } else if opcode == Opcode::Iconst2 as u8 {
            frame.operands.push(runtime::Operand::Int(2));
        } else if opcode == Opcode::Iconst3 as u8 {
            frame.operands.push(runtime::Operand::Int(3));
        } else if opcode == Opcode::Imul as u8 {
            let x = frame.operands.pop();
            let y = frame.operands.pop();

            match (x, y) {
                (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                    let result = x_value * y_value;

                    frame.operands.push(runtime::Operand::Int(result));
                },
                (_, _) => panic!("Invalid IMUL operands.")
            }
        } else if opcode == Opcode::Isub as u8 {
            let x = frame.operands.pop();
            let y = frame.operands.pop();

            match (x, y) {
                (Some(runtime::Operand::Int(x_value)), Some(runtime::Operand::Int(y_value))) => {
                    let result = x_value - y_value;

                    frame.operands.push(runtime::Operand::Int(result));
                },
                (_, _) => panic!("Invalid ISUB operands.")
            }
        }

        pc += 1;
        if pc == program.len() {
            break;
        }
    }

    println!("{:?}", frame);

    println!("Completed executing program.");
}
