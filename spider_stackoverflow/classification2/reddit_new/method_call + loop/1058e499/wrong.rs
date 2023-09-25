use std::{time::{Instant},};

#[derive(Clone, Copy, Debug)]
enum Operation{
    Add(u32),
    Mult(u32),
    Square,
    Null
}

#[derive(Clone, Debug)]
struct Monkey{
    number: u32,
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    true_monkey: u32,
    false_monkey: u32,
}

impl Monkey{
    fn new(number: u32, items: Vec<u32>, operation: Operation, test: u32, true_monkey: u32, false_monkey: u32) -> Monkey{
        println!("Monkey {} constructed.", number);
        Monkey{number, items, operation, test, true_monkey, false_monkey}
    }
}

#[derive(Clone, Debug)]
struct Troupe{
    monkeys: Vec<Monkey>,
}

impl Troupe {
    fn new() -> Troupe{
        Troupe{monkeys: vec![]}
    }

    fn push_monkey(&mut self, monkey: Monkey){
        self.monkeys.push(monkey);
    }

    fn round(&mut self){
        for monkey in &mut self.monkeys{
            println!("Monkey: {}", monkey.number);

            for item in &mut monkey.items{
                println!("\tMonkey inspects an item with a worry level of {}.", item);
                let original_level = *item;
                let mut new_level = 0;
                match monkey.operation{
                        Operation::Add(x) => {
                            new_level = original_level + x;
                            println!("\t\tWorry level increases by {} to {}.", x, new_level);
                        },
                        Operation::Mult(x) => {
                            new_level = original_level * x;
                            println!("\t\tWorry level is multiplied by {} to {}.", x, new_level);                        
                        },
                        Operation::Square => {
                            new_level = original_level * original_level;
                            println!("\t\tWorry level is multiplied by itself to {}.", new_level);          
                        }
                        Operation::Null => {} 
                    }
                new_level = new_level / 3;
                println!("\t\tMonkey gets bored with item. Worry level is divided by 3 to {}.", new_level);
                if new_level + *(&mut monkey.test) == 0{
                    let i = monkey.true_monkey as usize;
                    self.monkeys[i].items.push(item.clone());              
                }
                else{
                    let i = monkey.false_monkey as usize;
                    self.monkeys[i].items.push(item.clone());
                }

            }
            
        }
        
    }
}

pub fn part_1(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut troupe = Troupe::new();
    let mut inputvec = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        inputvec.push(parts);
    }
    let mut counter = 0;
    while counter <= 3{
        let offset = 7 * counter;
        let number: u32 = inputvec[0 + offset][1].replace(":", "").parse().unwrap();
        let mut starting_items: Vec<u32> = vec![];
        for (i, num) in inputvec[1 + offset].clone().into_iter().enumerate(){
            if i != 0 && i != 1{
                starting_items.push(num.replace(",", "").parse().unwrap());
            }
        }
        let mut operation = Operation::Null;
        if inputvec[2 + offset][4] == "+"{
            operation = Operation::Add(inputvec[2 + offset][5].parse().unwrap());
        }
        if inputvec[2 + offset][4] == "*" && inputvec[2 + offset][5] != "old"{
            operation = Operation::Mult(inputvec[2 + offset][5].parse().unwrap());
        }
        else{
            operation = Operation::Square;
        }
        let test: u32 = inputvec[3 + offset][3].parse().unwrap();
        let true_monkey: u32 = inputvec[4 + offset][5].parse().unwrap();
        let false_monkey: u32 = inputvec[5 + offset][5].parse().unwrap();
        troupe.push_monkey(Monkey::new(number, starting_items, operation, test, true_monkey, false_monkey));

        counter += 1;
    }
        println!("Troupe Created.");
        troupe.round();
}

fn main() {
    let start = Instant::now();
    part_1();
    let duration = start.elapsed();
    println!("Done. This took {} mcs.", duration.as_micros());
}