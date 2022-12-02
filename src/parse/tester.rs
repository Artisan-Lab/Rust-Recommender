fn main() {
    let my_array = vec![61, 14, 71, 23, 42, 8, 13, 66];
    
    let max = themax(my_array);
    let _min = themin(my_array);
    println!("Max value is {}.", max);
}

fn themax(array: Vec<i32>) -> i32 {
    let mut max = array[0];
    for val in array {
        if max < val {
            max = val;
        }
        else{
            max = val;
        }
    }
    max
}

fn themin(array: Vec<i32>) -> i32 {
    let mut min = array[0];
    for val in array {
        if min > val {
            min = val;
        }
    }
    min
}
