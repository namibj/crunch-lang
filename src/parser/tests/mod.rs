mod proptests;

use super::*;

// TODO: Actual parser testing

#[test]
#[ignore]
fn parse_test() {
    const CODE: &str = include_str!("../../../tests/parse_test.crunch");
    const FILENAME: &str = "parse_test.crunch";

    let mut parser = Parser::new(Some(FILENAME), CODE);
    let ast = parser.parse().unwrap();

    let bytecode = crate::interpreter::Interpreter::from_interner(
        &crate::OptionBuilder::new("./tests/parse_test.crunch").build(),
        parser.interner,
    )
    .interpret(ast.0.clone())
    .unwrap();
    println!("Bytecode: {:?}", &bytecode);

    crate::Vm::default().execute(&bytecode).unwrap();
}

#[test]
fn fibonacci_iterative_test() {
    const CODE: &str = include_str!("../../../tests/fibonacci_iterative.crunch");
    const FILENAME: &str = "fibonacci_iterative.crunch";

    let mut parser = Parser::new(Some(FILENAME), CODE);
    let ast = parser.parse().unwrap();

    let bytecode = crate::interpreter::Interpreter::from_interner(
        &crate::OptionBuilder::new("./tests/fibonacci_iterative.crunch").build(),
        parser.interner,
    )
    .interpret(ast.0.clone())
    .unwrap();

    crate::Vm::default().execute(&bytecode).unwrap();
}

#[test]
fn factorial_iterative_test() {
    const CODE: &str = include_str!("../../../tests/factorial_iterative.crunch");
    const FILENAME: &str = "factorial_iterative.crunch";

    let mut parser = Parser::new(Some(FILENAME), CODE);
    let ast = parser.parse().unwrap();

    let bytecode = crate::interpreter::Interpreter::from_interner(
        &crate::OptionBuilder::new("./tests/factorial_iterative.crunch").build(),
        parser.interner,
    )
    .interpret(ast.0.clone())
    .unwrap();

    crate::Vm::default().execute(&bytecode).unwrap();
}

#[test]
fn fibonacci_recursive_test() {
    const CODE: &str = include_str!("../../../tests/fibonacci_recursive.crunch");
    const FILENAME: &str = "fibonacci_recursive.crunch";

    let mut parser = Parser::new(Some(FILENAME), CODE);
    let ast = parser.parse().unwrap();

    let bytecode = crate::interpreter::Interpreter::from_interner(
        &crate::OptionBuilder::new("./test/fibonacci_recursive.crunch").build(),
        parser.interner,
    )
    .interpret(ast.0.clone())
    .unwrap();

    crate::Vm::default().execute(&bytecode).unwrap();
}

#[test]
fn factorial_recursive_test() {
    const CODE: &str = include_str!("../../../tests/factorial_recursive.crunch");
    const FILENAME: &str = "factorial_recursive.crunch";

    let mut parser = Parser::new(Some(FILENAME), CODE);
    let ast = parser.parse().unwrap();

    let bytecode = crate::interpreter::Interpreter::from_interner(
        &crate::OptionBuilder::new("./tests/factorial_recursive.crunch").build(),
        parser.interner,
    )
    .interpret(ast.0.clone())
    .unwrap();

    crate::Vm::default().execute(&bytecode).unwrap();
}

#[test]
#[ignore]
fn ffi_test() {
    let functions = bytecode! {
        0 => {
            load "ffi_test.dll", 0;
            ldlib 0, 0;
            load "add", 1;
            load 10i32, 2;
            load 20i32, 3;
            push 2;
            push 3;
            execlib 1, 0, 2u16;
            pop 2;
            print 2;
            ret;
        }
    };

    crate::Vm::default().execute(&functions).unwrap();
}

#[allow(non_snake_case)]
mod fuzz_found {
    use super::*;

    // Note: All test names here are MD5 hashes of the panicking input preceded by an underscore

    #[test]
    fn _C6CBD54946E2A1D183EAA7D86241656F() {
        let input = "fn main()
            i +=
        end";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _6B58087CB2578DD9C74F702A574A4C91() {
        let input = "fn main()
            let i = (1 + (100 / (1 * 10293207277133";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _307818184C53B8C3412B778AF5B836F8() {
        let input = "fn main()
            println('Test\")
        end";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _9D1BE77EA5DDA279F6679AD75A7213AF() {
        let input = "fn main()
            println(factorial(1))
            println(factorial(10))
            println(factorial(20))
        end
        
        fn factorial(n: int) -> int
            let product = 1
        
            for i in 1..n
                product *= i
            end
        
            return product
        end";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _FE605BD31796CB1A1A4A882487967B90() {
        let input = "fn main()614^8154291434688Ʀ \\ (1))
            pri -30076509";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _46439F3F511989ABB1A794D5CD34F5C4() {
        let input = "\x66\x6E\x20\x6D\x61\x69\x6E\x28\x29\x0A\x20\x20\x13\x20\x65\x6D\x70\x74\x79\x0A\x65\x6E\x64";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _A5DCA4A77C31DDDD5E03730B3D962718() {
        let input = "fn main()
            println(fibonacci-473222347563415634756341563475634156994onacci(10))
            println(fibonacci(20))
        end
        
        fn fibonacci(n: int) -> int
            if n == 0
                return 0
            else if n5311160814823956516n 1
            else
                let a = 0
                let b = 1
        
                for i in 0..n
                    let c = b
                    a = c
                    b = a + c
                end
        
                return a";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _56C81E044729298317C09A2D7112A555() {
        let input = "fn majn()
            printnn(fibonacci(1))
            println(fibonacci(10))
            println(fibonacci(20))
        end

        fn fibonacci(n: int) -> int
            if n <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<4<<<<<<<<<<<<<<<<<onaccj)n - 2)
            end
        end";

        let _ = Parser::new(None, input).parse();
    }

    #[test]
    fn _AFC02FAD36D7E05E4CB9B9EB2A300FCE() {
        let input = "fn loopy()
            for i in 0. for i in 0..test()
            i in 0..test()
            i in 0..tes0()
                println(i)
            in 0..test-81524255529t082509i)";

        let _ = Parser::new(None, input).parse();
    }
}
