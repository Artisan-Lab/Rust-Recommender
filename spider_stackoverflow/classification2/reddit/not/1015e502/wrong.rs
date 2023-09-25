fn main() {
    let mut memory = vec![];
    memory[1] = 5;

    memory[memory[1]] = 10;         
    
    let address = memory[1];            // works
    memory[address] = 10;
    
 
}