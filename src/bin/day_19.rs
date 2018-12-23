use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let state: State = data.parse().unwrap();
    println!("part_1: {}", part_1(&state));
    // NOTE: this initial number is specific to my input
    println!("part_2: {}", part_2(10551287));
}

fn part_1(state_0: &State) -> usize {
    let mut state = state_0.clone();
    while state.next() {}
    state.registers[0]
}

fn part_2(n: usize) -> usize {
    let mut sum = 0;
    let mut q = 1;
    while q * q <= n {
        if n % q == 0 {
            sum += q;
            if q * q != n {
                sum += n / q;
            }
        }
        q += 1;
    }
    sum
}

type Args = (usize, usize, usize);
type Registers = [usize; 6];
type Instruction = (Operation, Args);
type OperationFn = fn(Args, Registers) -> Registers;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Operation(&'static str, OperationFn);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct State {
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    registers: Registers,
}

impl State {
    fn next(&mut self) -> bool {
        if let Some(&(op, args)) = self
            .instructions
            .get(self.registers[self.instruction_pointer])
        {
            self.registers = op.1(args, self.registers);
            self.registers[self.instruction_pointer] += 1;
            true
        } else {
            false
        }
    }
}

fn parse_operation(data: &str) -> Result<Operation, &'static str> {
    match data {
        "addi" => Ok(Operation("addi", addi)),
        "addr" => Ok(Operation("addr", addr)),
        "muli" => Ok(Operation("muli", muli)),
        "mulr" => Ok(Operation("mulr", mulr)),
        "bani" => Ok(Operation("bani", bani)),
        "banr" => Ok(Operation("banr", banr)),
        "bori" => Ok(Operation("bori", bori)),
        "borr" => Ok(Operation("borr", borr)),
        "seti" => Ok(Operation("seti", seti)),
        "setr" => Ok(Operation("setr", setr)),
        "gtir" => Ok(Operation("gtir", gtir)),
        "gtri" => Ok(Operation("gtri", gtri)),
        "gtrr" => Ok(Operation("gtrr", gtrr)),
        "eqir" => Ok(Operation("eqir", eqir)),
        "eqri" => Ok(Operation("eqri", eqri)),
        "eqrr" => Ok(Operation("eqrr", eqrr)),
        _ => Err("Invalid operation"),
    }
}

// This function is not used in the final solution, but I used it to deparse the input for eyeballing
fn deparse_instruction(line: usize, ip: usize, (op, (a, b, c)): Instruction) -> String {
    let r = |i| {
        if i == ip {
            line.to_string()
        } else {
            format!("r[{}]", i)
        }
    };
    let lhs = |i| {
        if i == ip {
            "goto 1 +".to_string()
        } else {
            format!("r[{}] =", i)
        }
    };
    match op.0 {
        "addi" => format!("{} {} + {};", lhs(c), r(a), b),
        "addr" => format!("{} {} + {};", lhs(c), r(a), r(b)),
        "muli" => format!("{} {} * {};", lhs(c), r(a), b),
        "mulr" => format!("{} {} * {};", lhs(c), r(a), r(b)),
        "bani" => format!("{} {} & {};", lhs(c), r(a), b),
        "banr" => format!("{} {} & {};", lhs(c), r(a), r(b)),
        "bori" => format!("{} {} | {};", lhs(c), r(a), b),
        "borr" => format!("{} {} | {};", lhs(c), r(a), r(b)),
        "seti" => format!("{} {};", lhs(c), a),
        "setr" => format!("{} {};", lhs(c), r(a)),
        "gtir" => format!("{} ({} > {});", lhs(c), a, r(b)),
        "gtri" => format!("{} ({} > {});", lhs(c), r(a), b),
        "gtrr" => format!("{} ({} > {});", lhs(c), r(a), r(b)),
        "eqir" => format!("{} ({} == {});", lhs(c), a, r(b)),
        "eqri" => format!("{} ({} == {});", lhs(c), r(a), b),
        "eqrr" => format!("{} ({} == {});", lhs(c), r(a), r(b)),
        _ => panic!("invalid operation"),
    }
}

impl std::str::FromStr for State {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut state = State::default();
        let mut lines = data.lines();
        let ip_s = lines.next().ok_or("missing instruction pointer line")?;
        state.instruction_pointer = ip_s[4..]
            .parse()
            .map_err(|_| "invalid instruction pointer")?;
        for line in lines {
            let line_parts: Vec<_> = line.split(" ").collect();
            let op = parse_operation(line_parts.get(0).ok_or("no operation in line")?)?;
            let args: Args = (
                line_parts
                    .get(1)
                    .ok_or("arg 0 missing")?
                    .parse()
                    .map_err(|_| "invalid arg")?,
                line_parts
                    .get(2)
                    .ok_or("arg 1 missing")?
                    .parse()
                    .map_err(|_| "invalid arg")?,
                line_parts
                    .get(3)
                    .ok_or("arg 2 missing")?
                    .parse()
                    .map_err(|_| "invalid arg")?,
            );
            state.instructions.push((op, args));
        }
        Ok(state)
    }
}

fn addi((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = r[a] + b;
    out
}

fn addr((a, b, c): Args, r: Registers) -> Registers {
    addi((a, r[b], c), r)
}

fn muli((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = r[a] * b;
    out
}

fn mulr((a, b, c): Args, r: Registers) -> Registers {
    muli((a, r[b], c), r)
}

fn bani((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = r[a] & b;
    out
}

fn banr((a, b, c): Args, r: Registers) -> Registers {
    bani((a, r[b], c), r)
}

fn bori((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = r[a] | b;
    out
}

fn borr((a, b, c): Args, r: Registers) -> Registers {
    bori((a, r[b], c), r)
}

fn seti((a, _, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = a;
    out
}

fn setr((a, b, c): Args, r: Registers) -> Registers {
    seti((r[a], b, c), r)
}

fn gtir((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = if a > r[b] { 1 } else { 0 };
    out
}

fn gtri((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = if r[a] > b { 1 } else { 0 };
    out
}

fn gtrr((a, b, c): Args, r: Registers) -> Registers {
    gtri((a, r[b], c), r)
}

fn eqir((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = if a == r[b] { 1 } else { 0 };
    out
}

fn eqri((a, b, c): Args, r: Registers) -> Registers {
    let mut out = r.clone();
    out[c] = if r[a] == b { 1 } else { 0 };
    out
}

fn eqrr((a, b, c): Args, r: Registers) -> Registers {
    eqri((a, r[b], c), r)
}

#[cfg(test)]
mod tests {
    const DATA: &'static str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_part_1() {
        use part_1;
        use State;
        let state: State = DATA.parse().unwrap();
        assert_eq!(7, part_1(&state));
    }
}
