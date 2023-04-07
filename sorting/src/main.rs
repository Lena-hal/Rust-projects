fn main() {
}

pub fn bubble(arr: &mut [i64]){
    let lenght = arr.len();
    for i in 0..lenght{
        for j in 0..lenght-i-1{
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1)
            }
        }
    }
}

pub fn insert(arr: &mut [i32]){
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}
