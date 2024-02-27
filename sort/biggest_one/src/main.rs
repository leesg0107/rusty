//numbers	return
//[6, 10, 2]	"6210"
//[3, 30, 34, 5, 9]	"9534330"

fn making_biggest_num(arr: &mut Vec<i32>) -> String {
    arr.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab) // 내림차순으로 정렬하기 위해 순서를 바꿔 비교
    });

    if arr[0] == 0 {
        // 모든 숫자가 0인 경우, "0"만 반환
        return "0".to_string();
    }

    arr.iter().map(|&num| num.to_string()).collect()
}

fn main() {
    let mut arr = vec![3, 30, 34, 5, 9];
    let result = making_biggest_num(&mut arr);
    println!("{}", result); // 출력: "6210"
}
