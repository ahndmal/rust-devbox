///
/// Merges two collections into one. 
/// 
fn sorted_merge<T: Default + Clone + PartialOrd>(sorted_l: Vec<T>, sorted_r: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = vec![Default::default(); sorted_l.len() 
     + sorted_r.len()];

    let (mut i, mut j) = (0, 0);
    let mut k = 0;
    while i < sorted_l.len() && j < sorted_r.len() {
        if sorted_l[i] <= sorted_r[j] {
            result[k] = sorted_l[i].clone();
            i += 1;
        } else {
            result[k] = sorted_r[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < sorted_l.len() {
        result[k] = sorted_l[i].clone();
        k += 1;
        i += 1;
    }

    while j < sorted_r.len() {
        result[k] = sorted_r[j].clone();
        k += 1;
        j += 1;
    }
    result
}
