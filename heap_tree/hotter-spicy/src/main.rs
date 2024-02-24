//scoville	K	return
//[1, 2, 3, 9, 10, 12]	7	2
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn magic_pot(scoville: &mut Vec<i32>, k: &i32) -> i32 {
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    for &s in scoville.iter() {
        heap.push(Reverse(s));
    }
    while heap
        .peek()
        .map_or(false, |&Reverse(s)| s < *k && heap.len() > 1)
    {
        let s1 = heap.pop().unwrap().0;
        let s2 = heap.pop().unwrap().0;
        heap.push(Reverse(s1 + s2 * 2));
        ans += 1;
    }

    if heap.peek().map_or(false, |&Reverse(s)| s < *k) {
        -1
    } else {
        ans
    }
}

fn main() {
    let mut scoville = vec![1, 1, 1];
    let K = 7;
    let result = magic_pot(&mut scoville, &K);
    println!("{}", result);
}
