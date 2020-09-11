fn main() {
    println!("create");
    let s = Some(10);
    println!("{:?}", s);
    // let n = None; -> panic
    let n = None as Option<i32>;
    println!("{:?}", n);
    println!("----");

    println!("match");
    let result = match s {
        Some(n) => n,
        None => -1,
    };
    println!("{:?}", result);

    let result = match n {
        Some(n) => n,
        None => -1,
    };
    println!("{:?}", result);
    println!("----");

    println!("unwrap");
    println!("{:?}", s.unwrap());
    // println!("{:?}", n.unwrap()); -> panic
    println!("----");

    println!("map");
    println!("{:?}", s.map(|n| n + 1));
    println!("{:?}", n.map(|n| n + 1));
    println!("----");

    println!("and_then");
    println!(
        "{:?}",
        s.and_then(|n| {
            if n > 0 {
                Some(n + 1)
            } else {
                None
            }
        })
    );
    println!(
        "{:?}",
        Some(-1).and_then(|n| {
            if n > 0 {
                Some(n + 1)
            } else {
                None
            }
        })
    );
    println!(
        "{:?}",
        n.and_then(|n| {
            if n > 0 {
                Some(n + 1)
            } else {
                None
            }
        })
    );
    println!("----");

    println!("andthen.map");
    println!(
        "{:?}",
        s.and_then(|n| {
            if n > 0 {
                Some(n + 1)
            } else {
                None
            }
        })
        .map(|n| n * 2)
    );
    println!("----");
}
