/*
prices	return
[1, 2, 3, 2, 3]	[4, 3, 1, 1, 0]
*/

fn time_doesnt_decrease(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..arr.len() {
        let mut time = 0;
        if i == arr.len() - 1 {
            result.push(0);
            break;
        } // the last element of i should be 0
        for j in i + 1..arr.len() {
            time = j - i;
            if arr[i] > arr[j] {
                break;
            }
        }
        result.push(time as i32);
    }
    result
}

/*   -->i use reference to solve this problem '&'
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
