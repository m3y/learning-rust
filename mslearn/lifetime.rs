fn maybe_remove_prefix<'a>(orig: &'a [i32], prefix: &[i32]) -> &'a [i32] {
    if orig.starts_with(prefix) {
        return orig.split_at(prefix.len()).1;
    } else {
        return orig;
    }
}

fn main() {
    let a = [1, 2, 3];
    let suffix;
    {
        let b = [1, 2];
        suffix = maybe_remove_prefix(&a, &b);
        //suffix = maybe_remove_prefix(&b, &a);
    }
    println!("{:?}", suffix);
}
