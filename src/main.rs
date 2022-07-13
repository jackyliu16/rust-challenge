fn median(mut a: Vec<f32>) -> Option<f32> {
    // check if empty
    if a.is_empty() { return None; } 
    
    let n_elements: usize = a.len();
    
    // result sort [using partial_cmp]
    a.sort_by(|x, y| x.partial_cmp(y).unwrap());

    // a select sort 
    // for i in 0..n_elements {
    //     let mut min_index = i;
    //     for j in i..n_elements {
    //         if a[min_index] > a[j] {
    //             min_index = j;
    //         } 
    //     }
    //     let temp_value = a[i];
    //     a[i] = a[min_index];
    //     a[min_index] = temp_value;
    // } 

    let middle: usize = n_elements / 2;
    
    let mid: f32 = if n_elements % 2 == 0 {
        (a[middle-1] + a[middle]) / 2.0
    } else {
        a[middle]
    };

    Some(mid)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
