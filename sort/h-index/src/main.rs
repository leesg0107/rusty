//citations	return
//[3, 0, 6, 1, 5]	3

fn h_index(citations: &mut Vec<i32>) -> i32 {
    citations.sort_unstable_by(|a, b| b.cmp(a)); // 내림차순으로 정렬
    let mut h = 0;
    for (i, &citation) in citations.iter().enumerate() {
        if citation >= (i as i32 + 1) {
            h = i as i32 + 1; // H-Index 업데이트
        } else {
            break; // 나머지 논문은 조건을 만족시키지 못함
        }
    }
    h // 계산된 H-Index 반환
}

fn main() {
    let mut citations = vec![3, 0, 6, 1, 5];
    let result = h_index(&mut citations);
    println!("{}", result); // 올바른 H-Index 출력
}
