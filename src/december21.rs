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

type Instr = (OpCode, i64, i64, i64);

fn execute(instr: Instr, reg_bank: &mut [i64]) -> &mut [i64] {
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
        Gtir => (i1 > r!(i2)).into(),
        Gtri => (r!(i1) > i2).into(),
        Gtrr => (r!(i1) > r!(i2)).into(),
        Eqir => (i1 == r!(i2)).into(),
        Eqri => (r!(i1) == i2).into(),
        Eqrr => (r!(i1) == r!(i2)).into(),
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
            .map(|s| Ok(s.parse::<i64>()?))
            .collect::<Result<Vec<i64>>>()?;
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

fn run(ip_reg: usize, instrs: &[Instr], r0: i64, stop: bool) -> Result<i64> {
    let mut ip = 0;
    let mut reg_bank = vec![r0, 0, 0, 0, 0, 0];

    while ip >= 0 && ip < instrs.len() as i64 {
        reg_bank[ip_reg] = ip;
        if ip == 28 && stop {
            break;
        }

        execute(instrs[ip as usize], &mut reg_bank);
        ip = reg_bank[ip_reg] + 1;
    }

    Ok(reg_bank[5])
}

fn simplified(r0: i64) -> i64 {
    let mut r = vec![r0, 0, 0, 0, 0, 0];

    // 00 - 04
    while r[5] != 72 {
        r[5] = 123;
        r[5] &= 456;
    }

    let mut prev = 0;

    // 05
    r[5] = 0;

    let mut set = hashset! {};

    // 06 - 30
    loop {
        // 06
        r[4] = r[5] | 0b10000000000000000;

        // 07
        r[5] = 15466939; //0b111011000000000110111011;

        // 08 - 27
        while r[4] > 0 {
            // 08
            r[3] = r[4] & 0b11111111; // 255

            // 09
            r[5] += r[3];

            // 10
            r[5] &= 0b111111111111111111111111;

            // 11
            r[5] *= 65899; //0b10000000101101011;

            // 12
            r[5] &= 0b111111111111111111111111;

            // 17 - 25
            r[4] /= 256;
        }

        if set.contains(&r[5]) {
            return prev;
        } else {
            prev = r[5];
            set.insert(r[5]);
        }

        // 28 - 30
        if r[5] == r[0] {
            break;
        }
    }

    return -1;
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december21.txt")?;
    let (ip_reg, instrs) = parse(&lines)?;

    println!("Part 1: {}", run(ip_reg, &instrs, 0, true)?);
    println!("Part 2: {}", simplified(0));

    Ok(())
}
