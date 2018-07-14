fn main() {
    let mut arr = vec![19, 5, 56, 68, 23, 3, 99];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

fn insertion_sort<T: PartialOrd + Clone>(arr: &mut Vec<T>) {
    for i in 1..arr.len() {
        let key = arr[i].clone();

        let mut j = i;
        while j > 0 && key < arr[j - 1] {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}
