use std::collections::BTreeMap;

fn btreemap(strings: Vec<&str>) -> Vec<i32> {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();

    for op in strings {
        let parts: Vec<&str> = op.split_whitespace().collect();
        let command = parts[0];
        let value = parts[1].parse::<i32>().unwrap();

        match command {
            "I" => *map.entry(value).or_insert(0) += 1,
            "D" => {
                if !map.is_empty() {
                    let key = match value {
                        1 => *map.keys().next_back().unwrap(),
                        -1 => *map.keys().next().unwrap(),
                        _ => 0,
                    };
                    if let Some(count) = map.get_mut(&key) {
                        *count -= 1;
                        if *count <= 0 {
                            map.remove(&key);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    if map.is_empty() {
        vec![0, 0]
    } else {
        let min = *map.keys().next().unwrap();
        let max = *map.keys().next_back().unwrap();
        vec![max, min]
    }
}

fn main() {
    let operations = vec!["I 16", "I -5643", "D -1", "D 1", "D 1", "I 123", "D -1"];
    let result = btreemap(operations);
    println!("Result: {:?}", result); // 예상 출력: [0, 0] 또는 다른 최솟값과 최댓값
}
