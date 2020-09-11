fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("called, code: {}", code);
    Ok(code)
}

fn main() {
    println!("create");
    let result: Result<i32, String> = Ok(200);
    let err: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result);
    println!("{:?}", err);
    println!("----");

    println!("match");
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }
    match err {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }
    println!("----");

    println!("if let");
    let result: Result<i32, String> = Ok(200);
    if let Ok(code) = result {
        println!("code: {}", code);
    }
    println!("----");

    println!("unwrap_or");
    let result: Result<i32, String> = Ok(200);
    let err: Result<i32, String> = Err(String::from("error"));
    println!("code: {}", result.unwrap_or(-1));
    println!("code: {}", err.unwrap_or(-1));
    println!("----");

    println!("unwrap_or_else");
    let result: Result<i32, String> = Ok(200);
    let err: Result<i32, String> = Err(String::from("error"));
    println!("code: {}", result.unwrap_or_else(|_| 500));
    println!("code: {}", err.unwrap_or_else(|_| 500));
    println!("----");

    println!("map");
    let result: Result<i32, String> = Ok(200);
    let _err: Result<i32, String> = Err(String::from("error"));
    println!("code: {:?}", result.map(|c| c + 100));
    //println!("code: {:?}", _err.map(|c| c + 100)); -> panic

    println!("and_then");
    let result: Result<i32, String> = Ok(200);
    let err: Result<i32, String> = Err(String::from("error"));
    println!(
        "code: {:?}",
        result.and_then(|c| {
            println!("called");
            Ok(c + 1000)
        })
    );

    println!(
        "code: {:?}",
        err.and_then(|c| {
            println!("called");
            Ok(c + 1000)
        })
    );
    println!("----");

    println!("syntax suger: '?'");
    let result: Result<i32, String> = Ok(200);
    let err: Result<i32, String> = Err(String::from("error"));
    println!("{:?}", error_handling(result));
    println!("{:?}", error_handling(err));
    println!("----");
}
