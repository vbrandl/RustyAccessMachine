use std::error::Error;
use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

#[derive(Debug)]
enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    LDA,
    LDK,
    STA,
    INP,
    OUT,
    HLT,
    JMP,
    JEZ,
    JNE,
    JLZ,
    JLE,
    JGZ,
    JGE,
}

struct Instruction {
    operator: Operator,
    operand: i16,
}

fn read_file(path: &str) -> String {
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => return s,
    };
}

fn compile_line(line: &str) -> Option<Instruction> {
    let clean = line.trim();
    let v: Vec<&str> = clean.split(';').collect();
    let clean = v[0];
    if clean == "" {
        return None;
    }
    let v: Vec<&str> = clean.splitn(2, ' ').collect();

    let op: Operator = match v[0].to_uppercase().trim() {
        "ADD" => Operator::ADD,
        "SUB" => Operator::SUB,
        "MUL" => Operator::MUL,
        "DIV" => Operator::DIV,
        "LDA" => Operator::LDA,
        "LDK" => Operator::LDK,
        "STA" => Operator::STA,
        "INP" => Operator::INP,
        "OUT" => Operator::OUT,
        "HLT" => Operator::HLT,
        "JMP" => Operator::JMP,
        "JEZ" => Operator::JEZ,
        "JNE" => Operator::JNE,
        "JLZ" => Operator::JLZ,
        "JLE" => Operator::JLE,
        "JGZ" => Operator::JGZ,
        "JGE" => Operator::JGE,
        _     => panic!("Illegal operator {}", v[0]),
    };
    let operand: i16 = match v[1].trim().parse::<i16>() {
        Ok(n)   => n,
        Err(e)  => panic!("Invalid number {} -> {}", v[1], e),
    };

    Some(Instruction { operator: op, operand: operand })
}

fn execute(prog: Vec<Instruction>) {
    let mut pc: usize = 0;
    let mut akku: i16 = 0;
    let mut data: [i16; 256] = [0; 256];
    loop {
        match prog[pc].operator {
            Operator::ADD => akku += data[prog[pc].operand as usize],
            Operator::SUB => akku -= data[prog[pc].operand as usize],
            Operator::MUL => akku *= data[prog[pc].operand as usize],
            Operator::DIV => akku /= data[prog[pc].operand as usize],
            Operator::LDA => akku = data[prog[pc].operand as usize],
            Operator::LDK => akku = prog[pc].operand,
            Operator::STA => data[prog[pc].operand as usize] = akku,
            Operator::INP => loop {
                print!("-> ");
                io::stdout().flush().unwrap();
                let mut inp = String::new();
                io::stdin().read_line(&mut inp)
                    .expect("Failed to read line");

                let inp: i16 = match inp.trim().parse() {
                    Ok(n)  => n,
                    Err(_) => continue,
                };
                data[prog[pc].operand as usize] = inp;
                break;
            },
            Operator::OUT => println!("{}", data[prog[pc].operand as usize]),
            Operator::HLT => exit(prog[pc].operand as i32),
            Operator::JMP => { pc = prog[pc].operand as usize; continue; }
            Operator::JEZ => if akku == 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
            Operator::JNE => if akku != 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
            Operator::JLZ => if akku < 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
            Operator::JLE => if akku <= 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
            Operator::JGZ => if akku > 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
            Operator::JGE => if akku >= 0 {
                pc = prog[pc].operand as usize;
                continue;
            },
        };
        pc += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[0]);
    if args.len() < 2 {
        println!("Usage: {} filename", args[0]);
        exit(1);
    }

    let content = read_file(&args[1]);
    let content: Vec<&str> = content.split('\n').collect();
    let mut vec: Vec<Instruction> = Vec::new();
    for l in &content {
        let instr = match compile_line(l) {
            Some(x) => x,
            None    => continue,
        };
        vec.push(instr);
    }
    execute(vec);
}
