macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! vec {
    ( $x:ty ) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    assert_eq!(25, five_times!(2 + 3));

    let x = vec![0];
    assert_eq!(0, x[0]);
    println!("{:?}", x);

    let y = vec![0, 1];
    assert_eq!(0, y[0]);
    assert_eq!(1, y[1]);
    println!("{:?}", y);

    let z = vec![i32];
    assert!(z.is_empty());
    println!("{:?}", z);
}
