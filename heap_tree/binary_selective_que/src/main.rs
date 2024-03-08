use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MinMaxHeap {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
    count: usize,
}
impl MinMaxHeap {
    fn new() -> Self {
        MinMaxHeap {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
            count: 0,
        }
    }
    fn push(&mut self, value: i32) {
        self.min_heap.push(Reverse(value));
        self.max_heap.push(value);
        self.count += 1;
    }
    fn pop_min(&mut self) -> Option<i32> {
        if self.count > 0 {
            self.count -= 1;
            self.min_heap.pop().map(|Reverse(val)| val) //if value of self.min_heap.pop() is Some, then return val -> this is a role of map()
        } else {
            None
        }
    }
    fn pop_max(&mut self) -> Option<i32> {
        if self.count > 0 {
            self.count -= 1; // 요소를 제거할 때마다 카운트를 감소시킵니다.
            self.max_heap.pop()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

fn detective(commands: &[&str]) -> Vec<i32> {
    let mut minmaxheap = MinMaxHeap::new();
    for &command in commands {
        let mut iter = command.split_whitespace();
        let action = iter.next().unwrap();
        let number = iter.next().unwrap().parse::<i32>().unwrap();
        match action {
            "I" => minmaxheap.push(number),
            "D" if number == 1 => {
                minmaxheap.pop_max();
            }
            "D" if number == -1 => {
                minmaxheap.pop_min();
            }
            _ => (),
        }
    }

    if minmaxheap.is_empty() {
        vec![0, 0] // when all elements are removed,
    } else {
        let max = minmaxheap.pop_max().unwrap_or(0);
        let min = minmaxheap.pop_min().unwrap_or(0);
        vec![max, min]
    }
}
//->문제점, min heap and max heap are not synchronized.
fn main() {
    let commands = ["I 16", "I -5643", "D -1", "D 1", "D 1", "I 123", "D -1"];
    let result = detective(&commands);
    println!("Result: {:?}", result); // 예상 출력: Result: [0, 0]
}
