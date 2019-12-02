mod decode;
mod encode;

pub use decode::decode_program;
pub use encode::encode_program;

/// The length of an encoded instruction, in bytes
pub const INSTRUCTION_LENGTH: usize = 8;

/// An array containing all Instruction byte headers, for verification purposes
#[cfg_attr(rustfmt, rustfmt::skip)]
pub const INSTRUCTION_BYTES: [u8; 27] = [
    0x00, 0x01, 0x02, 0x03, 0x04,
    0x05, 0x06, 0x07, 0x08, 0x09,
    0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    0x0F, 0x10, 0x11, 0x12, 0x13,
    0x14, 0x15, 0x16, 0x17, 0x18,
    0x19, 0x1A,
];

/// Disassembles bytecode into a human-readable format
// TODO: Refractor and test
pub fn disassemble(bytes: &[u8]) -> String {
    use super::{Instruction, Value, NUMBER_REGISTERS};
    use std::{collections::HashMap, fmt::Write};

    let functions = {
        let (main, functions) = decode_program(bytes).unwrap();
        let mut funcs = vec![main];
        funcs.extend_from_slice(&functions);
        funcs
    };

    let mut output = String::new();
    for (index, function) in functions.into_iter().enumerate() {
        if index == 0 {
            write!(&mut output, "=> Main Function\n").unwrap();
        } else {
            write!(&mut output, "=> Function {}\n", index).unwrap();
        }

        let mut registers: [Value; NUMBER_REGISTERS] = array_init::array_init(|_| Value::None);
        let mut heap: HashMap<u32, Value> = HashMap::new();

        for (instruction_index, instruction) in function.into_iter().enumerate() {
            match &instruction {
                Instruction::Load(val, reg) => {
                    registers[**reg as usize] = val.clone();
                }
                Instruction::Cache(heap_loc, val, reg) => {
                    heap.insert(*heap_loc, val.clone());
                    registers[**reg as usize] = val.clone();
                }
                Instruction::Drop(heap_loc) => {
                    heap.remove(&heap_loc);
                }
                Instruction::DropReg(reg) => {
                    registers[**reg as usize] = Value::None;
                }
                Instruction::Save(heap_loc, reg) => {
                    heap.insert(*heap_loc, registers[**reg as usize].clone());
                    registers[**reg as usize] = Value::None;
                }
                _ => {}
            }

            let param = {
                use super::Instruction::*;

                match &instruction {
                    Load(val, reg) => format!("{}, {:?}", reg, val),
                    Save(heap, reg) => format!("{:p}, {}", *heap as *const u8, reg),
                    Cache(heap, ref val, reg) => {
                        format!("{:p}, {}, {}", *heap as *const u8, val, reg)
                    }
                    CompToReg(reg) | OpToReg(reg) | DropReg(reg) => format!("{}", reg),
                    Drop(reg) => format!("{}", reg),

                    Print(reg) => {
                        if registers[**reg as usize] != Value::None {
                            format!("{}: {}", reg, registers[**reg as usize].to_string())
                        } else {
                            format!("{}", reg)
                        }
                    }

                    Jump(abs) | JumpComp(abs) => format!("{:04}", abs),

                    Not(reg) => format!(
                        "{}",
                        if registers[**reg as usize] != Value::None {
                            format!("{}: {}", reg, registers[**reg as usize].to_string())
                        } else {
                            format!("{}", reg)
                        }
                    ),

                    Add(left, right)
                    | Sub(left, right)
                    | Mult(left, right)
                    | Div(left, right)
                    | And(left, right)
                    | Or(left, right)
                    | Xor(left, right)
                    | Eq(left, right)
                    | NotEq(left, right)
                    | GreaterThan(left, right)
                    | LessThan(left, right) => format!(
                        "{}, {}",
                        if registers[**left as usize] != Value::None {
                            format!("{}: {:?}", left, registers[**left as usize].to_string())
                        } else {
                            format!("{}", left)
                        },
                        if registers[**right as usize] != Value::None {
                            format!("{}: {:?}", right, registers[**right as usize].to_string())
                        } else {
                            format!("{}", right)
                        }
                    ),

                    Syscall(offset, output, param1, param2, param3, param4, param5) => format!(
                        "0x{:X} ({}, {}, {}, {}, {}) -> {}",
                        crate::syscall::SYSCALL_TABLE[*offset as usize],
                        if registers[**param1 as usize] != Value::None {
                            format!("{}: {:?}", param1, registers[**param1 as usize].to_string())
                        } else {
                            format!("{}", param1)
                        },
                        if registers[**param2 as usize] != Value::None {
                            format!("{}: {:?}", param2, registers[**param2 as usize].to_string())
                        } else {
                            format!("{}", param2)
                        },
                        if registers[**param3 as usize] != Value::None {
                            format!("{}: {:?}", param3, registers[**param3 as usize].to_string())
                        } else {
                            format!("{}", param3)
                        },
                        if registers[**param4 as usize] != Value::None {
                            format!("{}: {:?}", param4, registers[**param4 as usize].to_string())
                        } else {
                            format!("{}", param4)
                        },
                        if registers[**param5 as usize] != Value::None {
                            format!("{}: {:?}", param5, registers[**param5 as usize].to_string())
                        } else {
                            format!("{}", param5)
                        },
                        output,
                    ),

                    Return | Halt | Illegal | Collect | JumpPoint(_) | NoOp => String::new(),
                }
            };

            write!(
                &mut output,
                "  {:04}: {} {}\n",
                instruction_index,
                instruction.to_str(),
                param,
            )
            .unwrap();
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_test() {
        use crate::{Instruction::*, Value};
        use std::io::Write;

        simple_logger::init().unwrap();
        color_backtrace::install();

        let (instructions, functions) = (
            vec![
                Cache(0, Value::Int(10), 0.into()),
                Print(0.into()),
                Cache(1, Value::Int(5), 1.into()),
                Print(1.into()),
                Div(0.into(), 1.into()),
                OpToReg(3.into()),
                Print(3.into()),
                Drop(0),
                Drop(1),
                Collect,
                Cache(0, Value::Pointer(0), 0.into()),
                Print(0.into()),
                Syscall(
                    0,
                    1.into(),
                    0.into(),
                    1.into(),
                    1.into(),
                    1.into(),
                    1.into(),
                ),
                Halt,
            ],
            Vec::new(),
        );

        let encoded_program = encode_program(instructions.clone(), functions.clone());
        let decoded_program = decode_program(&encoded_program).unwrap();

        let mut file = std::fs::File::create("./examples/hello_world.crunched").unwrap();
        file.write_all(&encoded_program).unwrap();

        println!("{}", disassemble(&encoded_program));

        assert_eq!((instructions, functions), decoded_program);
        let mut crunch = crate::Crunch::from((
            decoded_program.0,
            decoded_program.1,
            crate::OptionBuilder::new("./byte_test").build(),
        ));
        crunch.execute();
    }
}
