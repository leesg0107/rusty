//array	                 commands	                        return
//[1, 5, 2, 6, 3, 7, 4]	[[2, 5, 3], [4, 4, 1], [1, 7, 3]]	[5, 6, 3]

fn command(arr: &Vec<i32>, commands: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for cmd in commands.iter() {
        let (i, j, k) = (cmd[0] as usize - 1, cmd[1] as usize, cmd[2] as usize - 1);
        let mut sub_arr = arr[i..j].to_vec();
        sub_arr.sort();
        result.push(sub_arr[k]);
    }
    result
}

fn main() {
    let arr = vec![1, 5, 2, 6, 3, 7, 4];
    let commands = vec![vec![2, 5, 3], vec![4, 4, 1], vec![1, 7, 3]];
    let result = command(&arr, &commands);
    println!("{:?}", result);
}
