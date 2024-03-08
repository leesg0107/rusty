fn explore_dungeons(
    k: i32,
    dungeons: &Vec<Vec<i32>>,
    index: usize,
    explored: &mut Vec<bool>,
) -> i32 {
    if index == dungeons.len() {
        return 0;
    }

    let mut max_explored = 0;

    for i in 0..dungeons.len() {
        if !explored[i] && dungeons[i][0] <= k {
            explored[i] = true;
            // 탐험 후 남은 피로도로 다음 던전 탐험 시도
            let explored_count =
                1 + explore_dungeons(k - dungeons[i][1], dungeons, index + 1, explored);
            max_explored = max_explored.max(explored_count);
            explored[i] = false; // 백트래킹으로 상태 복원
        }
    }

    max_explored
}

fn adventure(k: i32, dungeons: &Vec<Vec<i32>>) -> i32 {
    let mut explored = vec![false; dungeons.len()];
    explore_dungeons(k, dungeons, 0, &mut explored)
}

fn main() {
    let k = 80;
    let dungeons = vec![vec![80, 20], vec![50, 40], vec![30, 10]];
    let result = adventure(k, &dungeons);
    println!("Maximum dungeons explored: {}", result);
}
