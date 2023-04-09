fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10;
        }
    };
    println!("result = {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    let mut number = 5;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("ENDED While loop!");

    let a = [10,20,30,40,50];

    for element in a {
        println!("{element}");
    }
    
    for element in 1..=10  {
        println!("{element}");
    }
}
