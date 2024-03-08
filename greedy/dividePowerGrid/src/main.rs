fn dfs(node: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> usize {
    visited[node] = true;
    let mut size = 1;

    for &next in &graph[node] {
        if !visited[next] {
            size += dfs(next, visited, graph);
        }
    }

    size
}

fn solution(n: usize, wires: Vec<Vec<i32>>) -> i32 {
    let mut min_diff = n as i32;
    let mut graph = vec![vec![]; n];

    // 그래프 구성
    for wire in &wires {
        let (a, b) = (wire[0] as usize - 1, wire[1] as usize - 1);
        graph[a].push(b);
        graph[b].push(a);
    }

    for wire in &wires {
        let (a, b) = (wire[0] as usize - 1, wire[1] as usize - 1);

        // 전선 끊기
        graph[a].retain(|&x| x != b);
        graph[b].retain(|&x| x != a);

        // 두 부분 그래프의 송전탑 개수 계산
        let mut visited = vec![false; n];
        let size = dfs(a, &mut visited, &graph);

        // 송전탑 개수 차이 계산
        let diff = ((n - size) as i32 - size as i32).abs();
        if diff < min_diff {
            min_diff = diff;
        }

        // 전선 다시 연결
        graph[a].push(b);
        graph[b].push(a);
    }

    min_diff
}

fn main() {
    //[[1,2],[2,7],[3,7],[3,4],[4,5],[6,7]]
    let wires = vec![
        vec![1, 2],
        vec![2, 7],
        vec![3, 7],
        vec![3, 4],
        vec![4, 5],
        vec![6, 7],
    ];
    let n = 9;
    let result = solution(n, wires);
    println!("{}", result); // 예상 결과 출력
}
