use super::{AllocId, CachedValue, GcStr, Index, Register, RegisterValue, RuntimeValue, Value, Vm};

/// A type alias for results that could be a [`RuntimeError`]
pub type Result<T> = std::result::Result<T, RuntimeError>;

/// A Crunch Runtime Error
// TODO: Make this more detailed
#[derive(Debug, Clone, Eq)]
pub struct RuntimeError {
    /// The type of error
    pub ty: RuntimeErrorTy,
    /// The error message
    pub message: String,
}

impl RuntimeError {
    /// Prints the formatted error to stdout
    // TODO: Make this fancy, and more detailed
    pub fn emit(&self) {
        println!("[Crunch Runtime Error: {:?}] {}", self.ty, self.message);
    }
}

impl PartialEq for RuntimeError {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

/// The type of [`RuntimeError`] that occurred
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeErrorTy {
    /// An error in the [`GC`]
    GcError,
    /// The user attempted to divide by zero
    DivideByZero,
    /// The two types are incompatible in the requested operation
    IncompatibleTypes,
    /// The program is missing a main function
    MissingMain,
    /// The requested variable is null
    NullVar,
    /// Thrown when an illegal instruction is executed
    IllegalInstruction,
    InvalidJump,
    MissingValue,
    MissingString,
    InvalidString,
    FileError,
    BytecodeError,
}

/// Instructions for the VM
// TODO: Document all Instructions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    /// Load a Value directly into a register
    Load(Value, Register),
    /// Cache a Value into the GC and load it
    Cache(u32, Value, Register),
    // TODO: Have a direct load to a register
    CompToReg(Register),
    OpToReg(Register),
    Save(u32, Register),
    DropReg(Register),
    Drop(u32),

    Add(Register, Register),
    Sub(Register, Register),
    Mult(Register, Register),
    Div(Register, Register),

    Print(Register),

    Jump(i32),
    JumpComp(i32),
    JumpPoint(u32),

    And(Register, Register),
    Or(Register, Register),
    Xor(Register, Register),
    Not(Register),

    Eq(Register, Register),
    NotEq(Register, Register),
    GreaterThan(Register, Register),
    LessThan(Register, Register),

    Collect,
    // TODO: Flesh this instruction out
    Return,
    Halt,
    // TODO: Use a `Value::ParamBuf` instead
    Syscall(
        u8,
        Register,
        Register,
        Register,
        Register,
        Register,
        Register,
    ),

    // TODO: Handle FFI with the following instructions
    // LoadLib(&'static str), // Loads a dynamic library
    // CallLib(&'static str, input: Value::ParamBuf, output: Value::ParamBuf),

    // An illegal instruction
    Illegal,
    NoOp,
}

impl Instruction {
    /// The execution of each instruction
    // TODO: Document this bad boy
    #[inline]
    pub fn execute(&self, mut vm: &mut Vm) -> Result<()> {
        match self {
            Instruction::Load(val, reg) => {
                trace!("Loading val into {}", reg);

                let val = RuntimeValue::Register(match val {
                    Value::Bool(b) => RegisterValue::Bool(*b),
                    Value::Int(i) => RegisterValue::Int(*i),
                    Value::Pointer(p) => RegisterValue::Pointer(*p),
                    Value::String(s) => {
                        // Hella unsafe buckaroo
                        RegisterValue::String(unsafe { std::mem::transmute(s.as_str()) })
                    }
                    Value::None => unreachable!(), // Is None really unreachable?
                });

                vm.registers[**reg as usize] = val;

                vm.index += Index(1);
            }
            Instruction::Cache(heap_loc, val, reg) => {
                trace!("Loading value onto heap at {}, Val: {:?}", heap_loc, val);

                let reg_val = RuntimeValue::Cached(if let Value::String(string) = val {
                    let string = GcStr::new(string, &mut vm.gc)?;

                    CachedValue::String(string)
                } else {
                    let (alloc_val, alloc_id) = vm
                        .gc
                        .allocate_id(std::mem::size_of::<Value>(), *heap_loc as usize)?;

                    unsafe {
                        vm.gc.write(alloc_id, val.to_owned(), Some(&alloc_val))?;
                    }

                    vm.gc.add_root(alloc_val);

                    debug_assert_eq!(*heap_loc as usize, *alloc_id);

                    match val {
                        Value::Bool(_) => CachedValue::Bool(alloc_id),
                        Value::Int(_) => CachedValue::Int(alloc_id),
                        Value::Pointer(_) => CachedValue::Pointer(alloc_id),
                        Value::String(_) | Value::None => unreachable!(), // Is None really unreachable?
                    }
                });

                vm.registers[**reg as usize] = reg_val;

                vm.index += Index(1);
            }
            Instruction::CompToReg(reg) => {
                trace!("Loading previous comparison into {}", reg);

                vm.registers[**reg as usize] =
                    RuntimeValue::Register(RegisterValue::Bool(vm.prev_comp));
                vm.index += Index(1);
            }
            Instruction::OpToReg(reg) => {
                trace!("Loading previous operation into {}", reg);
                trace!("Previous Operation Value: {:?}", &vm.prev_op);

                std::mem::swap(&mut vm.registers[**reg as usize], &mut vm.prev_op);

                vm.index += Index(1);
            }
            Instruction::Save(heap_loc, reg) => {
                trace!("Saving register {} to {}", reg, heap_loc);

                unsafe {
                    vm.gc.write(
                        *heap_loc as usize,
                        vm.registers[**reg as usize].clone(),
                        None,
                    )?;
                }

                vm.index += Index(1);
            }
            Instruction::DropReg(reg) => {
                trace!("Clearing register {}", reg);

                vm.registers[**reg as usize] = RuntimeValue::None;
                vm.index += Index(1);
            }
            Instruction::Drop(id) => {
                trace!("Dropping {:?}", id);
                vm.gc.remove_root(*id as usize)?;
                vm.index += Index(1);
            }

            Instruction::Add(left, right) => {
                vm.prev_op = (*vm).get(*left).add((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Sub(left, right) => {
                vm.prev_op = (*vm).get(*left).sub((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Mult(left, right) => {
                vm.prev_op = (*vm).get(*left).mult((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Div(left, right) => {
                vm.prev_op = (*vm).get(*left).div((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }

            Instruction::Print(reg) => {
                trace!("Printing reg {:?}", reg);

                if let Err(_err) = write!(vm.stdout, "{}", vm.get(*reg).clone()) {
                    panic!("Handle this sometime");
                }

                vm.index += Index(1);
            }

            Instruction::Jump(index) => {
                trace!("Jumping by offset {}", index);

                println!("Jumping by {} on index {}", index, vm.index);

                let index = if index.is_negative() {
                    let (index, overflowed) = vm.index.overflowing_sub(index.abs() as u32);

                    if overflowed {
                        return Err(RuntimeError {
                            ty: RuntimeErrorTy::InvalidJump,
                            message: "Jump overflowed".to_string(),
                        });
                    }

                    index + 1
                } else {
                    *vm.index + *index as u32 + 1
                };

                vm.index = Index(index);
            }
            Instruction::JumpComp(index) => {
                trace!(
                    "Comparison Jump: Prev Comp is {}, jump amount is {}",
                    vm.prev_comp,
                    index
                );

                if vm.prev_comp {
                    vm.index = Index((*vm.index as i32 + *index + 1) as u32);
                } else {
                    vm.index += Index(1);
                }
            }

            Instruction::And(left, right) => {
                vm.prev_op = (*vm).get(*left).bit_and((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Or(left, right) => {
                vm.prev_op = (*vm).get(*left).bit_or((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Xor(left, right) => {
                vm.prev_op = (*vm).get(*left).bit_xor((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::Not(reg) => {
                vm.prev_op = (*vm).get(*reg).not(&vm.gc)?;
                vm.index += Index(1);
            }

            Instruction::Eq(left, right) => {
                vm.prev_comp = (*vm).get(*left).eq((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::NotEq(left, right) => {
                vm.prev_comp = !(*vm).get(*left).eq((*vm).get(*right), &vm.gc)?;
                vm.index += Index(1);
            }
            Instruction::GreaterThan(left, right) => {
                vm.prev_comp = (*vm).get(*left).cmp((*vm).get(*right), &vm.gc)?
                    == Some(std::cmp::Ordering::Greater);
                vm.index += Index(1);
            }
            Instruction::LessThan(left, right) => {
                vm.prev_comp = (*vm).get(*left).cmp((*vm).get(*right), &vm.gc)?
                    == Some(std::cmp::Ordering::Less);
                vm.index += Index(1);
            }

            Instruction::Collect => {
                trace!("Forcing a GC collect");

                vm.gc.collect()?;
                vm.index += Index(1);
            }
            Instruction::Return => {
                vm.returning = true;

                if let Some(context) = vm.snapshots.pop() {
                    vm.index = context.0;
                    vm.registers = context.2;

                    if let Some(index) = context.1 {
                        vm.returning = false;

                        while !vm.returning {
                            vm.functions[*index as usize][*vm.index as usize]
                                .clone()
                                .execute(&mut vm)?;
                        }
                    } else {
                        trace!("Returning to main");
                    }
                } else {
                    if let Some(location) = vm.return_stack.pop() {
                        vm.index = location;
                    } else {
                        vm.finished_execution = true;
                    }
                }

                vm.index += Index(1);
            }
            Instruction::Halt => {
                vm.finished_execution = true;
            }
            Instruction::Syscall(offset, output, param_1, param_2, param_3, param_4, param_5) => {
                let p = [param_1, param_2, param_3, param_4, param_5];
                let func = (*super::syscall::SYSCALL_TABLE)[*offset as usize];
                let func: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize =
                    unsafe { std::mem::transmute(func) };

                let mut params = [0_usize; 5];
                for index in 0..5 {
                    match vm.registers[**p[index] as usize] {
                        RuntimeValue::Cached(CachedValue::Pointer(p)) => {
                            let i = vm.gc.fetch::<usize, AllocId>(p)?;
                            params[index] = i;
                        }
                        RuntimeValue::Register(RegisterValue::Pointer(p)) => params[index] = p,
                        RuntimeValue::Cached(CachedValue::Int(i)) => {
                            let i = vm.gc.fetch::<i32, AllocId>(i)?;
                            params[index] = i as usize;
                        }
                        RuntimeValue::Register(RegisterValue::Int(i)) => params[index] = i as usize,
                        _ => {}
                    }
                }

                let result = unsafe { func(params[0], params[1], params[2], params[3], params[4]) };

                vm.registers[**output as usize] =
                    RuntimeValue::Register(RegisterValue::Pointer(result));

                vm.index += Index(1);
            }

            Instruction::NoOp => {
                vm.index += Index(1);
            }

            Instruction::Illegal | Instruction::JumpPoint(_) => {
                return Err(RuntimeError {
                    ty: RuntimeErrorTy::IllegalInstruction,
                    message: "Illegal Instruction".to_string(),
                })
            }
        }

        Ok(())
    }

    /// Turns the instruction into a string representation, for disassembly purposes
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Load(_, _) => "ld",
            Self::Cache(_, _, _) => "cache",
            Self::CompToReg(_) => "compr",
            Self::OpToReg(_) => "opr",
            Self::Save(_, _) => "save",
            Self::DropReg(_) => "dropr",
            Self::Drop(_) => "drop",

            Self::Add(_, _) => "add",
            Self::Sub(_, _) => "sub",
            Self::Mult(_, _) => "mul",
            Self::Div(_, _) => "div",

            Self::Print(_) => "print",

            Self::Jump(_) => "jmp",
            Self::JumpComp(_) => "jmpcmp",
            Self::JumpPoint(_) => "jmppt",

            Self::And(_, _) => "and",
            Self::Or(_, _) => "or",
            Self::Xor(_, _) => "xor",
            Self::Not(_) => "not",

            Self::Eq(_, _) => "eq",
            Self::NotEq(_, _) => "neq",
            Self::GreaterThan(_, _) => "grt",
            Self::LessThan(_, _) => "let",

            Self::Collect => "coll",
            Self::Return => "ret",
            Self::Halt => "halt",
            Self::Syscall(_, _, _, _, _, _, _) => "sysc",

            Self::Illegal => "illegal",
            Self::NoOp => "nop",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;

    #[test]
    fn variable_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./variable_ops").build(),
            Box::new(stdout()),
        );

        let values = {
            use rand::Rng;
            use std::thread;

            let mut rng = rand::thread_rng();
            let (num_ints, num_strs) = (rng.gen_range(200, 1000), rng.gen_range(300, 1500));
            let mut vec = Vec::with_capacity(num_ints + num_strs);

            let ints = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut vec = Vec::with_capacity(num_ints);

                for _ in 0..num_ints {
                    vec.push(Value::Int(rng.gen_range(0, i32::max_value())));
                }

                vec
            });

            let first_strings = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut vec = Vec::with_capacity(num_strs / 2);

                for _ in 0..num_strs / 2 {
                    let len = rng.gen_range(10, 200);
                    let mut string = String::with_capacity(len);

                    for _ in 0..len {
                        string.push(rand::random::<char>());
                    }

                    vec.push(Value::String(string));
                }

                vec
            });

            let second_strings = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut vec = Vec::with_capacity(num_strs / 2);

                for _ in num_strs / 2..num_strs {
                    let len = rng.gen_range(10, 200);
                    let mut string = String::with_capacity(len);

                    for _ in 0..len {
                        string.push(rand::random::<char>());
                    }

                    vec.push(Value::String(string));
                }

                vec
            });

            vec.push(Value::Bool(true));
            vec.push(Value::Bool(false));

            vec.extend_from_slice(&ints.join().unwrap());
            vec.extend_from_slice(&first_strings.join().unwrap());
            vec.extend_from_slice(&second_strings.join().unwrap());

            vec
        };

        for (id, val) in values.clone().into_iter().enumerate() {
            let cache = Instruction::Cache(id as u32, val.clone(), 0.into());
            cache.execute(&mut vm).unwrap();
            assert!(vm.gc.contains(id));
            assert_eq!(vm.gc.fetch(id), Ok(val));
        }

        // Do a VM Reset
        vm.gc = crate::Gc::new(&crate::OptionBuilder::new("./variable_ops").build());

        for (_id, val) in values.clone().into_iter().enumerate() {
            let load = Instruction::Load(val.clone(), 0.into());

            load.execute(&mut vm).unwrap();
            assert!(vm.registers[0]
                .eq(&RuntimeValue::from_val(val), &vm.gc)
                .ok()
                .unwrap());
        }

        // Do a VM Reset
        vm.gc = crate::Gc::new(&crate::OptionBuilder::new("./variable_ops").build());

        let comp_to_reg = Instruction::CompToReg(0.into());

        vm.prev_comp = true;
        comp_to_reg.execute(&mut vm).unwrap();
        assert!(vm.registers[0]
            .eq(&RuntimeValue::Register(RegisterValue::Bool(true)), &vm.gc)
            .ok()
            .unwrap());

        vm.prev_comp = false;
        comp_to_reg.execute(&mut vm).unwrap();
        assert!(vm.registers[0]
            .eq(&RuntimeValue::Register(RegisterValue::Bool(false)), &vm.gc)
            .ok()
            .unwrap());

        let op_to_reg = Instruction::OpToReg(2.into());
        for val in values.clone() {
            vm.prev_op = RuntimeValue::from_val(val.clone());
            op_to_reg.execute(&mut vm).unwrap();
            assert!(vm.registers[2]
                .eq(&RuntimeValue::from_val(val), &vm.gc)
                .ok()
                .unwrap());
        }

        for (id, val) in values.clone().into_iter().enumerate() {
            let (alloc_val, alloc_id) =
                vm.gc.allocate_id(std::mem::size_of::<Value>(), id).unwrap();
            unsafe {
                vm.gc
                    .write(alloc_id, val.clone(), Some(&alloc_val))
                    .unwrap();
            }
            vm.gc.add_root(alloc_val);

            vm.registers[0] = RuntimeValue::from_val(val.clone());
            let save = Instruction::Save(id as u32, 0.into());
            save.execute(&mut vm).unwrap();
            assert!(vm.gc.contains(id));
            assert_eq!(vm.gc.fetch(id), Ok(val));
        }

        let drop_reg = Instruction::DropReg(0.into());
        drop_reg.execute(&mut vm).unwrap();
        assert!(vm.registers[0]
            .eq(&RuntimeValue::from_val(Value::None), &vm.gc)
            .ok()
            .unwrap());

        // Do a VM Reset
        vm.gc = crate::Gc::new(&crate::OptionBuilder::new("./variable_ops").build());

        for (id, val) in values.clone().into_iter().enumerate() {
            let drop = Instruction::Drop(id as u32);

            let (alloc_val, alloc_id) =
                vm.gc.allocate_id(std::mem::size_of::<Value>(), id).unwrap();
            unsafe {
                vm.gc
                    .write(alloc_id, val.clone(), Some(&alloc_val))
                    .unwrap();
            }
            vm.gc.add_root(alloc_val);

            assert!(vm.gc.contains(id));
            assert_eq!(vm.gc.fetch(id), Ok(val.clone()));
            drop.execute(&mut vm).unwrap();
            vm.gc.collect().unwrap();
            assert!(!vm.gc.contains(id));
            assert!(vm.gc.fetch::<Value, usize>(id).is_err());
        }
    }

    #[test]
    fn math_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./math_ops").build(),
            Box::new(stdout()),
        );

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(50));
        vm.registers[1] = RuntimeValue::Register(RegisterValue::Int(10));

        let add = Instruction::Add(0.into(), 1.into());
        add.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(50 + 10)), &vm.gc)
            .ok()
            .unwrap());

        let sub = Instruction::Sub(0.into(), 1.into());
        sub.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(50 - 10)), &vm.gc)
            .ok()
            .unwrap());

        let mult = Instruction::Mult(0.into(), 1.into());
        mult.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(50 * 10)), &vm.gc)
            .ok()
            .unwrap());

        let div = Instruction::Div(0.into(), 1.into());
        div.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(50 / 10)), &vm.gc)
            .ok()
            .unwrap());
    }

    #[test]
    fn print_op() {
        use std::mem;

        let print = Instruction::Print(0.into());
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./print_op").build(),
            Box::new(Vec::<u8>::new()),
        );

        vm.registers[0] = RuntimeValue::Register(RegisterValue::String("Test"));
        print.execute(&mut vm).unwrap();

        // Have to do some monkeying with stdout because of Vm's drop implementation
        let mut stdout: Box<dyn std::io::Write + 'static> = Box::new(Vec::<u8>::new());
        mem::swap(&mut vm.stdout, &mut stdout);
        assert_eq!(unsafe { *(Box::into_raw(stdout) as *const &str) }, "Test");

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(10));
        print.execute(&mut vm).unwrap();

        let mut stdout: Box<dyn std::io::Write + 'static> = Box::new(Vec::<u8>::new());
        mem::swap(&mut vm.stdout, &mut stdout);
        assert_eq!(unsafe { *(Box::into_raw(stdout) as *const &str) }, "10");

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Bool(true));
        print.execute(&mut vm).unwrap();

        let mut stdout: Box<dyn std::io::Write + 'static> = Box::new(Vec::<u8>::new());
        mem::swap(&mut vm.stdout, &mut stdout);
        assert_eq!(unsafe { *(Box::into_raw(stdout) as *const &str) }, "true");

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Bool(false));
        print.execute(&mut vm).unwrap();

        let mut stdout: Box<dyn std::io::Write + 'static> = Box::new(std::io::stdout()); //  Load stdout into vm.stdout for the printing portion
        mem::swap(&mut vm.stdout, &mut stdout);
        assert_eq!(unsafe { *(Box::into_raw(stdout) as *const &str) }, "false");

        // Test that writing to stdout works too, can only verify that it does, not that it is correct

        vm.registers[0] = RuntimeValue::Register(RegisterValue::String("Test"));
        print.execute(&mut vm).unwrap();

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(10));
        print.execute(&mut vm).unwrap();

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Bool(true));
        print.execute(&mut vm).unwrap();

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Bool(false));
        print.execute(&mut vm).unwrap();
    }

    #[test]
    fn jump_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./jump_ops").build(),
            Box::new(stdout()),
        );

        // Each executed instruction increments the index by one, so take that into account

        let jump = Instruction::Jump(10);
        jump.execute(&mut vm).unwrap();
        assert_eq!(vm.index, 11.into());
        let jump = Instruction::Jump(-10);
        jump.execute(&mut vm).unwrap();
        assert_eq!(vm.index, 2.into());

        let jump_comp = Instruction::JumpComp(10);
        jump_comp.execute(&mut vm).unwrap();
        assert_eq!(vm.index, 3.into());
        vm.prev_comp = true;
        jump_comp.execute(&mut vm).unwrap();
        assert_eq!(vm.index, 14.into());
        let jump_comp = Instruction::JumpComp(-10);
        jump_comp.execute(&mut vm).unwrap();
        assert_eq!(vm.index, 5.into());
    }

    #[test]
    fn bitwise_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./bitwise_ops").build(),
            Box::new(stdout()),
        );

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(10));
        vm.registers[1] = RuntimeValue::Register(RegisterValue::Int(10));

        let and = Instruction::And(0.into(), 1.into());
        and.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(10 & 10)), &vm.gc)
            .ok()
            .unwrap());

        let or = Instruction::Or(0.into(), 1.into());
        or.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(10 | 10)), &vm.gc)
            .ok()
            .unwrap());

        let xor = Instruction::Xor(0.into(), 1.into());
        xor.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::from_val(Value::Int(10 ^ 10)), &vm.gc)
            .ok()
            .unwrap());

        let not = Instruction::Not(0.into());
        not.execute(&mut vm).unwrap();
        assert!(vm
            .prev_op
            .eq(&RuntimeValue::Register(RegisterValue::Int(!10)), &vm.gc)
            .ok()
            .unwrap());
    }

    #[test]
    fn eq_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./eq_ops").build(),
            Box::new(stdout()),
        );

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(10));
        vm.registers[1] = RuntimeValue::Register(RegisterValue::Int(10));

        let eq = Instruction::Eq(0.into(), 1.into());
        eq.execute(&mut vm).unwrap();
        assert_eq!(vm.prev_comp, true);

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(20));

        let not_eq = Instruction::NotEq(0.into(), 1.into());
        not_eq.execute(&mut vm).unwrap();
        assert_eq!(vm.prev_comp, true);

        let greater_than = Instruction::GreaterThan(0.into(), 1.into());
        greater_than.execute(&mut vm).unwrap();
        assert_eq!(vm.prev_comp, true);

        vm.registers[0] = RuntimeValue::Register(RegisterValue::Int(0));

        let less_than = Instruction::LessThan(0.into(), 1.into());
        less_than.execute(&mut vm).unwrap();
        assert_eq!(vm.prev_comp, true);
    }

    #[test]
    fn misc_ops() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./misc_ops").build(),
            Box::new(stdout()),
        );

        let collect = Instruction::Collect;

        let (discard, discard_id) = vm.gc.allocate(std::mem::size_of::<Value>()).unwrap();
        unsafe {
            vm.gc
                .write(discard_id, Value::Int(10), Some(&discard))
                .unwrap();
        }
        assert!(vm.gc.contains(discard_id));
        assert!(vm.gc.fetch(discard_id) == Ok(Value::Int(10)));
        collect.execute(&mut vm).unwrap();
        assert!(!vm.gc.contains(discard_id));

        // TODO: Find way to test these
        // Return,
        // Halt,
    }

    #[test]
    fn illegal_op() {
        let mut vm = Vm::new(
            Vec::new(),
            &crate::OptionBuilder::new("./illegal_ops").build(),
            Box::new(stdout()),
        );

        let illegal = Instruction::Illegal;
        assert_eq!(
            illegal.execute(&mut vm).err().unwrap().ty,
            RuntimeErrorTy::IllegalInstruction
        );
    }
}
