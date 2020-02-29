use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mut stop = false;
    let mut pc = 0;

    while !stop {
        let line = program[pc];
//        println!("execute line {} at {}", line, pc);

        if line.starts_with("mov") {
            operator_mov(line, &mut registers);
            pc += 1;
        } else if line.starts_with("inc") {
            operator_inc(line, &mut registers);
            pc += 1;
        } else if line.starts_with("dec") {
            operator_dec(line, &mut registers);
            pc += 1;
        } else if line.starts_with("jnz") {
            let (matched, step) = operator_jnz(line, &registers);
            if matched {
                if step < 0 {
                    pc -= (-step) as usize;
                } else {
                    pc += (step) as usize;
                }
            } else {
                pc += 1;
            }
        }


//        show_registers(&registers);

        if pc >= program.len() {
            stop = true;
        }
    }


    registers
}

fn operator_mov(program: &str, registers: &mut HashMap<String, i64>) {
    let mut split_whitespace = program.split_whitespace();
    split_whitespace.next();
    let register = split_whitespace.next().unwrap().to_string();

    let value = split_whitespace.next().unwrap();
    match value.parse::<i64>() {
        Ok(v) => {
            registers.insert(register, v);
        }
        Err(e) => {
            let v = registers.get(value).unwrap();
            registers.insert(register, *v);
        }
    }
}

fn operator_inc(program: &str, registers: &mut HashMap<String, i64>) {
    let mut split_whitespace = program.split_whitespace();
    split_whitespace.next();
    let register = split_whitespace.next().unwrap();
    let v = registers.get(register).unwrap();
    registers.insert(register.to_string(), v + 1);
}

fn operator_dec(program: &str, registers: &mut HashMap<String, i64>) {
    let mut split_whitespace = program.split_whitespace();
    split_whitespace.next();
    let register = split_whitespace.next().unwrap();
    let v = registers.get(register).unwrap();
    registers.insert(register.to_string(), v - 1);
}

fn operator_jnz(program: &str, registers: &HashMap<String, i64>) -> (bool, i64) {
    let mut split_whitespace = program.split_whitespace();
    split_whitespace.next();

    let mut v = 0 as i64;
    let register = split_whitespace.next().unwrap();
    match register.parse::<i64>() {
        Ok(va) => {
            v = va;
        }
        Err(_) => {
            v = *registers.get(register).unwrap();
        }
    }

    if v == 0 {
        return (false, 0);
    }
    return (true, split_whitespace.next().unwrap().parse::<i64>().unwrap());
}

fn show_registers(registers: &HashMap<String, i64>) {
    println!("debug info.......");
    for (register, value) in registers {
        println!("register: {}, value {}", register, value)
    }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
