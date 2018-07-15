fn main() {
    let mut arr = vec![19, 5, 56, 68, 23, 3, 99];
    insertion_sort(&mut arr);
    println!("insertion: {:?}", arr);

    arr = vec![10, 30, 20, 50, 100];
    selection_sort(&mut arr);
    println!("selection: {:?}", arr);
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

fn selection_sort<T: PartialOrd + Clone>(arr: &mut Vec<T>) {
    for p_i in 0..arr.len() - 1 {
        let mut min = p_i;
        for i in min + 1..arr.len() {
            if arr[i] < arr[min] {
                min = i;
            }
        }

        let tmp = arr[min].clone();
        arr[min] = arr[p_i].clone();
        arr[p_i] = tmp;
    }
}
