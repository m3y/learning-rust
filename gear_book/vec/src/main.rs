fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("----");

    println!("Vec can be accessed with '[]'");
    println!("{}", v1[2]);
    println!("----");

    println!("push");
    v1.push(6);
    println!("{:?}", v1);
    println!("----");

    println!("pop");
    let v: Option<i32> = v1.pop();
    println!("{}", v.unwrap());
    println!("{:?}", v1);
    println!("----");
}
