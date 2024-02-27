/*
function quickSort(array, low, high)
    if low < high
        // partitionIndex는 partition 함수를 통해 정렬된 피벗의 인덱스입니다.
        partitionIndex = partition(array, low, high)

        // 피벗보다 작은 부분을 재귀적으로 정렬
        quickSort(array, low, partitionIndex - 1)

        // 피벗보다 큰 부분을 재귀적으로 정렬
        quickSort(array, partitionIndex + 1, high)
end function

function partition(array, low, high)
    // 피벗을 배열의 마지막 요소로 선택
    pivot = array[high]

    i = low - 1
    for j = low to high - 1
        if array[j] < pivot
            i = i + 1
            swap array[i] with array[j]
    // 피벗을 올바른 위치로 이동
    swap array[i + 1] with array[high]
    return i + 1
end function

function swap(a, b)
    temp = a
    a = b
    b = temp
end function

*/
/*
function quickSort(array, low, high)
    if low < high
        // partitionIndex는 partition 함수를 통해 정렬된 피벗의 인덱스입니다.
        partitionIndex = partition(array, low, high)

        // 피벗보다 작은 부분을 재귀적으로 정렬
        quickSort(array, low, partitionIndex - 1)

        // 피벗보다 큰 부분을 재귀적으로 정렬
        quickSort(array, partitionIndex + 1, high)
end function
*/
fn qsort(mut array, mut low, mut high){
    if low<high{
        let mut  partitionIndex=partition(array,low,high);
        qsort(array,low,partitionIndex-1);
        qsort(array,partitionIndex+1,high);
    }
}


fn partition(mut array,mut low, mut high){
    let mut pivot=array[high];
    let mut i=low-1;

    for j in low..high-1{
        if array[j]<pivot{
            i+=1;
            array.swap(i,j);
    array.swap(i+1,high);
    i+1

        }
    }
}

fn main() {
    let mut arr = vec![6, 8, 3, 5, 1, 65, 7, 8, 2, 35, 95, 435, 66];

    qsort(&mut arr, 0, arr.len() - 1);
}
