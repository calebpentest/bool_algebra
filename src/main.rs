use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Op {
    And,
    Or,
    Not,
    Xor,
    Nand,
    Nor,
    Xnor,
    Imp,   // A → B
}

#[derive(Parser, Debug)]
#[command(name="bool_algebra", about="Boolean algebra CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Evaluate an operation
    Eval {
        /// Operation name
        #[arg(value_enum)]
        op: Op,
        /// A value (true/false/1/0)
        a: String,
        /// B value (omit for NOT)
        b: Option<String>,
    },
    /// Print the truth table for one operation
    Table {
        #[arg(value_enum)]
        op: Op,
    },
    /// Print truth tables for all ops
    AllTables,
}

fn parse_bool(s: &str) -> Result<bool, String> {
    match s.to_ascii_lowercase().as_str() {
        "1" | "t" | "true" | "y" | "yes" => Ok(true),
        "0" | "f" | "false" | "n" | "no" => Ok(false),
        _ => Err(format!("invalid bool: {}", s)),
    }
}

fn apply(op: Op, a: bool, b: Option<bool>) -> Result<bool, String> {
    Ok(match op {
        Op::Not => !a,
        Op::And => a & b.ok_or("AND needs B")?,
        Op::Or  => a | b.ok_or("OR needs B")?,
        Op::Xor => a ^ b.ok_or("XOR needs B")?,
        Op::Nand=> !(a & b.ok_or("NAND needs B")?),
        Op::Nor => !(a | b.ok_or("NOR needs B")?),
        Op::Xnor=> !(a ^ b.ok_or("XNOR needs B")?),
        Op::Imp => (!a) | b.ok_or("IMP needs B")?, // A→B == ¬A ∨ B
    })
}

fn print_table(op: Op) {
    match op {
        Op::Not => {
            println!("A | NOT");
            println!("--------");
            for &a in &[false, true] {
                let r = apply(Op::Not, a, None).unwrap();
                println!("{:>1} | {:>3}", bit(a), bit(r));
            }
        }
        _ => {
            println!("A B | {:?}", op);
            println!("-------------");
            for &a in &[false, true] {
                for &b in &[false, true] {
                    let r = apply(op.clone(), a, Some(b)).unwrap();
                    println!("{} {} |   {}", bit(a), bit(b), bit(r));
                }
            }
        }
    }
}

fn bit(x: bool) -> u8 { if x {1} else {0} }

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Eval { op, a, b } => {
            let av = parse_bool(&a).expect("bad A");
            let bv = match (&op, b) {
                (Op::Not, _) => None,
                (_, Some(bs)) => Some(parse_bool(&bs).expect("bad B")),
                (_, None) => { eprintln!("B is required for this op"); std::process::exit(1); }
            };
            let res = apply(op, av, bv).unwrap();
            println!("{}", bit(res));
        }
        Cmd::Table { op } => print_table(op),
        Cmd::AllTables => {
            for op in [Op::Not, Op::And, Op::Or, Op::Xor, Op::Nand, Op::Nor, Op::Xnor, Op::Imp] {
                print_table(op);
                println!();
            }
        }
    }
}
