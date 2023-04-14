use std::cmp;

fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in (0..triangle.len()-1).rev() { // from penultimate (last - 1) to first
        let current_row = &mut triangle[i];
        let last_row = &triangle[i+1];
        for j in 0..current_row.len() {
            current_row[j] = cmp::min(last_row[j], last_row[j+1]) + current_row[j];
        }
    }

    triangle[0][0]
}

fn main() {
    println!("{}", minimum_total(vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));
}

// 不得不夹杂使用，与循环没有太大关系，不得不使用trangle的不可变引用在使用途中还需要一个可变引用，所以clone掉就可以