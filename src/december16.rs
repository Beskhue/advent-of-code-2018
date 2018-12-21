use std::collections::HashSet;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum OpCode {
    Addr, Addi, Mulr, Muli,
    Banr, Bani, Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
}
use OpCode::*;

static OP_CODES: &[OpCode] = &[
    Addr, Addi, Mulr, Muli,
    Banr, Bani, Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

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
        Gtir => (i1 > r!(i2)).into(),
        Gtri => (r!(i1) > i2).into(),
        Gtrr => (r!(i1) > r!(i2)).into(),
        Eqir => (i1 == r!(i2)).into(),
        Eqri => (r!(i1) == i2).into(),
        Eqrr => (r!(i1) == r!(i2)).into(),
    };
    reg_bank
}

fn solve_constraints(map: &mut Vec<HashSet<OpCode>>) {
    let mut mutated = true;
    while mutated {
        mutated = false;

        for (idx, set) in map.clone().iter().enumerate() {
            if set.len() == 1 {
                let op_code = set.iter().nth(0).unwrap();
                for (idx2, set2) in map.iter_mut().enumerate() {
                    if idx != idx2 && set2.remove(op_code) {
                        mutated = true;
                    }
                }
            }
        }
    }
}

fn find_op_codes(lines: &[String]) -> Result<(u32, Vec<OpCode>)> {
    let mut valid: Vec<HashSet<OpCode>> =
        vec![OP_CODES.iter().cloned().collect(); 16];
    let mut count = 0;

    for idx in (0..=lines.len()).step_by(4) {
        let before = lines[idx][9..=18]
            .split(", ")
            .map(|s| Ok(s.parse::<i32>()?))
            .collect::<Result<Vec<_>>>()?;
        let instr = lines[idx + 1]
            .split(' ')
            .map(|s| Ok(s.parse::<i32>()?))
            .collect::<Result<Vec<_>>>()?;
        let after = lines[idx + 2][9..=18]
            .split(", ")
            .map(|s| Ok(s.parse::<i32>()?))
            .collect::<Result<Vec<_>>>()?;

        let mut count_valid = 0;
        for op_code in OP_CODES {
            if after
                == execute(
                    (*op_code, instr[1], instr[2], instr[3]),
                    &mut before.clone(),
                )
            {
                count_valid += 1;
            } else {
                valid[instr[0] as usize].remove(op_code);
            }
        }

        if count_valid >= 3 {
            count += 1;
        }
    }
    solve_constraints(&mut valid);

    Ok((
        count,
        valid.iter().map(|s| *s.iter().nth(0).unwrap()).collect(),
    ))
}

fn run(lines: &[String], mapping: Vec<OpCode>) -> Result<i32> {
    let mut reg_bank = vec![0, 0, 0, 0];

    for line in lines {
        let instr = line
            .split(' ')
            .map(|s| Ok(s.parse::<i32>()?))
            .collect::<Result<Vec<_>>>()?;
        execute(
            (mapping[instr[0] as usize], instr[1], instr[2], instr[3]),
            &mut reg_bank,
        );
    }

    Ok(reg_bank[0])
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december16.txt")?;
    let (count_many_matches, mapping) = find_op_codes(&lines[..3259])?;

    println!("Part 1: {:?}", count_many_matches);
    println!("Part 2: {:?}", run(&lines[3262..], mapping)?);

    Ok(())
}
