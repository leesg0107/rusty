/*
prices	return
[1, 2, 3, 2, 3]	[4, 3, 1, 1, 0]
*/

fn time_doesnt_decrease(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..arr.len() {
        println!("i: {}", i);
        let mut time = 0;
        if i == arr.len() - 1 {
            result.push(0);
            break;
        } // the last element of i should be 0
        for j in i + 1..arr.len() {
            // no matter i increase or nah, j is guarenteed 1. but i think this problem has error
            println!("j: {}", j); // because you can see 3 of arr[2] is gonna be 2, and it's not a 1 second but close one.
            time = j - i;
            if arr[i] > arr[j] {
                break;
            }
        }
        result.push(time as i32);
    }
    result
}

/*
fn time_doesnt_decrease(prices: &[i32]) -> Vec<i32> {
    let mut result = vec![0; prices.len()];
    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            result[i] += 1;
            if prices[i] > prices[j] {
                break;
            }
        }
    }
    result
}
*/
fn main() {
    let arr = vec![1, 2, 3, 2, 3];
    let result = time_doesnt_decrease(arr);
    println!("{:?}", result); // 예상 출력: [4, 3, 1, 1, 0]
}
