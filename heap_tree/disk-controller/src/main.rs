/*
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn scheduling(jobs: &mut Vec<Vec<i32>>) -> i32 {
    // 요청 시간에 따라 작업을 정렬합니다.
    jobs.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut heap = BinaryHeap::new();
    let mut time = 0; // 현재 시간
    let mut total_time = 0; // 모든 작업의 완료 시간의 합
    let mut index = 0; // 처리할 다음 작업의 인덱스

    while index < jobs.len() || !heap.is_empty() {
        // 현재 시간에 시작할 수 있는 모든 작업을 힙에 추가합니다
        while index < jobs.len() && jobs[index][0] <= time {
            heap.push(Reverse((jobs[index][1], jobs[index][0])));
            index += 1;
        }

        if let Some(Reverse((duration, start))) = heap.pop() {
            // 가장 짧은 작업을 처리합니다.
            time = std::cmp::max(time, start) + duration; // 작업의 시작 시간이 현재 시간보다 늦을 수 있으므로, max를 사용합니다.
            total_time += time - start; // 요청부터 종료까지 걸린 시간을 더합니다.
        } else {
            // 처리할 작업이 없으면, 다음 작업의 시작 시간으로 시간을 옮깁니다.
            time = jobs[index][0];
        }
    }

    total_time / jobs.len() as i32 // 평균 완료 시간을 계산하여 반환합니다.
}

fn main() {
    let mut jobs = vec![vec![0, 3], vec![1, 9], vec![2, 6]];
    let result = scheduling(&mut jobs);
    println!("{}", result); // 평균 완료 시간을 출력합니다.
}
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn add_jobs_to_heap(
    jobs: &Vec<Vec<i32>>,
    heap: &mut BinaryHeap<Reverse<(i32, i32)>>,
    current_time: i32,
    index: &mut usize,
) {
    while *index < jobs.len() && jobs[*index][0] <= current_time {
        heap.push(Reverse((jobs[*index][1], jobs[*index][0])));
        *index += 1;
    }
}

fn process_next_job(
    heap: &mut BinaryHeap<Reverse<(i32, i32)>>,
    current_time: &mut i32,
    total_time: &mut i32,
) -> bool {
    if let Some(Reverse((duration, start))) = heap.pop() {
        *current_time = std::cmp::max(*current_time, start) + duration;
        *total_time += *current_time - start;
        true
    } else {
        false
    }
}

fn scheduling(jobs: &mut Vec<Vec<i32>>) -> i32 {
    jobs.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut heap = BinaryHeap::new();
    let mut current_time = 0;
    let mut total_time = 0;
    let mut index = 0;

    while index < jobs.len() || !heap.is_empty() {
        if !heap.is_empty() || index < jobs.len() && jobs[index][0] <= current_time {
            add_jobs_to_heap(jobs, &mut heap, current_time, &mut index);
            while process_next_job(&mut heap, &mut current_time, &mut total_time) {}
        } else if index < jobs.len() {
            // 시간을 다음 작업의 시작 시간으로 옮깁니다.
            current_time = jobs[index][0];
            add_jobs_to_heap(jobs, &mut heap, current_time, &mut index);
        }
    }

    total_time / jobs.len() as i32
}

fn main() {
    let mut jobs = vec![vec![0, 9], vec![1, 3], vec![2, 6]];
    let result = scheduling(&mut jobs);
    println!("{}", result);
}

/*
    flow of logic
    - 요청시간에 따라 우선 작업을 추가한다.
    - 작업시간에 따라
*/
