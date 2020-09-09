///// 渡したものを返してもらう
//fn main() {
//    let mut important_data = "Hello, World!".to_string();
//    important_data = calc_data(important_data);
//
//    println!("{}", important_data);
//}
//
//fn calc_data(data: String) -> String {
//    println!("{}", data);
//    data
//}

/// 借用を使う
fn main() {
    let important_data = "Hello, World!".to_string();
    calc_data(&important_data);

    println!("{}", &important_data);
}

fn calc_data(data: &String) {
    println!("{}", data);
}
