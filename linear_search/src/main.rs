fn main() {

    let target = 5;
    let array = [1, 3, 5, 7, 9, 11, 13, 15, 17];

    println!("{}", linear_search(target, array));
}

fn linear_search(target: i32, &array[..]: Vec<i32>) -> i32 {

    for i in 0..array.len() {
        if array[i] == target {
            return i;
        }
    }
    return -1;
}