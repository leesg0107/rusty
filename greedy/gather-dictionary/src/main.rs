fn solution(word: &str) -> i32 {
    let vowels = vec!['A', 'E', 'I', 'O', 'U'];
    let mut result = 0;

    // 각 알파벳 위치에서의 가중치를 저장
    let weights = [781, 156, 31, 6, 1]; // 5자리 단어의 각 자리수별 조합 수

    for (i, ch) in word.chars().enumerate() {
        let index = vowels.iter().position(|&v| v == ch).unwrap() as i32;
        result += index * weights[i] + 1; // 현재 위치의 알파벳이 기여하는 순서 값 계산
    }

    result
}

fn main() {
    let examples = vec!["AAAAE", "AAAE", "I", "EIO"];
    for word in examples {
        println!("{}: {}", word, solution(word));
    }
}
