fn main() {
    println!("Hello, world!");

    let tests = vec![
	"login with rally".to_string(), 
	"rally health".to_string(), 
	"click to login".to_string(), 
	"to".to_string(), 
	"with".to_string(),
        ];



    struct Point {
        offset: u8,
        length: u8,
    }    

    //            0         1         2         3
    //            0123456789012345678901234567890
    let expect = "click to login with rally health";
    let login_with_rally = Point { offset: 9, length: 16 };
    let rally_health = Point { offset: 20, length: 12 };
    let click_to_login = Point { offset: 0, length: 14 };
    let to_ = Point { offset: 6, length: 2 };
    let with_ = Point { offset: 15, length: 4 };    

    tests.into_iter().for_each(|test| println!("\t{}",test));
    println!("end");
    compare("hello".to_string(), "0123456789".to_string());
}


fn compare(large: String, small: String) -> bool {
    let large_chars = large.chars(); 
    let small_chars = &mut small.chars(); 
    println!("{}", small_chars.as_str());
    for n in (0..small.len()) {
        match small_chars.nth(0) {
            Some(c) => println!("{}:{}", n, c),
            None => println!("{}:{}", n, "None")
	}
    } 
    true
}
