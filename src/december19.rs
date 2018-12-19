#[macro_use]
extern crate maplit;
use std::collections::HashMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use OpCode::*;

type Instr = (OpCode, i32, i32, i32);

fn execute(instr: Instr, reg_bank: &mut [i32]) -> &mut [i32] {
    macro_rules! r {
        ( $x:expr ) => {
            reg_bank[$x as usize]
        };
    }

    let (op_code, i1, i2, rr) = instr;
    reg_bank[rr as usize] = match op_code {
        Addr => r!(i1) + r!(i2),
        Addi => r!(i1) + i2,
        Mulr => r!(i1) * r!(i2),
        Muli => r!(i1) * i2,
        Banr => r!(i1) & r!(i2),
        Bani => r!(i1) & i2,
        Borr => r!(i1) | r!(i2),
        Bori => r!(i1) | i2,
        Setr => r!(i1),
        Seti => i1,
        Gtir => {
            if i1 > r!(i2) {
                1
            } else {
                0
            }
        }
        Gtri => {
            if r!(i1) > i2 {
                1
            } else {
                0
            }
        }
        Gtrr => {
            if r!(i1) > r!(i2) {
                1
            } else {
                0
            }
        }
        Eqir => {
            if i1 == r!(i2) {
                1
            } else {
                0
            }
        }
        Eqri => {
            if r!(i1) == i2 {
                1
            } else {
                0
            }
        }
        Eqrr => {
            if r!(i1) == r!(i2) {
                1
            } else {
                0
            }
        }
    };
    reg_bank
}

fn parse(lines: &[String]) -> Result<(usize, Vec<Instr>)> {
    let ip = lines
        .get(0)
        .and_then(|s| s.split(" ").nth(1))
        .ok_or("Malformed input")?
        .parse::<usize>()?;
    let mut instructions = Vec::new();
    let str_to_op: HashMap<&str, OpCode> = hashmap! {
        "addr" => Addr,
        "addi" => Addi,
        "mulr" => Mulr,
        "muli" => Muli,
        "banr" => Banr,
        "bani" => Bani,
        "borr" => Borr,
        "bori" => Bori,
        "setr" => Setr,
        "seti" => Seti,
        "gtir" => Gtir,
        "gtri" => Gtri,
        "gtrr" => Gtrr,
        "eqir" => Eqir,
        "eqri" => Eqri,
        "eqrr" => Eqrr
    };

    for line in &lines[1..] {
        let split = line.split(' ').collect::<Vec<_>>();
        let args = split[1..]
            .iter()
            .map(|s| Ok(s.parse::<i32>()?))
            .collect::<Result<Vec<i32>>>()?;
        let instr: Instr = (
            *str_to_op.get(split[0]).ok_or("Malformed input")?,
            args[0],
            args[1],
            args[2],
        );
        instructions.push(instr);
    }

    Ok((ip, instructions))
}

fn run(ip_reg: usize, instrs: &[Instr]) -> Result<i32> {
    let mut ip = 0;
    let mut reg_bank = vec![0, 0, 0, 0, 0, 0];

    while ip >= 0 && ip < instrs.len() as i32 {
        reg_bank[ip_reg] = ip;
        execute(instrs[ip as usize], &mut reg_bank);
        ip = reg_bank[ip_reg] + 1;
    }

    Ok(reg_bank[0])
}

fn decompiled(return_number_to_factorize: bool) -> i32 {
    let mut r = vec![1, 0, 0, 0, 0, 0];

    // 17 - 20
    r[4] += 2;
    r[4] *= r[4];
    r[4] *= 19;
    r[4] *= 11;

    // 21 - 23
    r[1] += 6;
    r[1] *= 22;
    r[1] += 10;

    // 24
    r[4] += r[1];

    if r[0] > 0 {
        // 27
        r[1] = 27;

        // 28
        r[1] *= 28;

        // 29
        r[1] += 29;

        // 30
        r[1] *= 30;

        // 31
        r[1] *= 14;

        // 32
        r[1] *= 32;

        // 33
        r[4] += r[1];

        // 34
        r[0] = 0;

        // 35
    }

    if return_number_to_factorize {
        return r[4];
    }

    // 01
    r[5] = 1;

    // 02 - 16.
    while r[5] <= r[4] {
        r[2] = 1;

        // 03 - 11
        while r[2] <= r[4] {
            // 03 - 07
            if r[5] * r[2] == r[4] {
                // 7
                r[0] += r[5];
            }

            // 08
            r[2] += 1;

            // 11
        }

        // 12
        r[5] += 1
    }

    r[0]
}

fn factorize(n: i32) -> i32 {
    (1..=n).filter(|m| n % m == 0).sum()
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december19.txt")?;
    let (ip_reg, instrs) = parse(&lines)?;

    println!("Part 1: {}", run(ip_reg, &instrs)?);
    println!("Part 2: {}", factorize(decompiled(true)));

    Ok(())
}
