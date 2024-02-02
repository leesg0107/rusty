fn main() {
    let mut arr = vec![6, 8, 3, 5, 1, 65, 7, 8, 2, 35, 95, 435, 66];

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", arr);
}
