//섞은 음식의 스코빌 지수 = 가장 맵지 않은 음식의 스코빌 지수 + (두 번째로 맵지 않은 음식의 스코빌 지수 * 2)
//scoville	K	return
//[1, 2, 3, 9, 10, 12]	k=7	return:2
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn magic_pot(scoville: &mut Vec<i32>, k: &i32) -> i32 {
    let mut operations = 0;
    let mut heap = BinaryHeap::new();

    for &s in scoville.iter() {
        heap.push(Reverse(s));
    }

    while heap.peek().map_or(false, |&Reverse(s)| s < *k) && heap.len() > 1 {
        let Reverse(smallest) = heap.pop().unwrap();
        let Reverse(second_smallest) = heap.pop().unwrap();
        let new_mix = smallest + second_smallest * 2;
        heap.push(Reverse(new_mix));
        operations += 1;
    }

    if heap.peek().map_or(false, |&Reverse(s)| s < *k) {
        -1
    } else {
        operations
    }
}

fn main() {
    let mut scoville = vec![1, 1, 1];
    let k = 10;
    let result = magic_pot(&mut scoville, &k);
    println!("{}", result);
}
