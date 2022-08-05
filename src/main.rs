use std::io;

fn get_n_fibonacci(fibo_ord: u32) -> u32 {

    if fibo_ord <= 1u32 {
        fibo_ord
    }
    else {
        
        let mut fibo_ord = fibo_ord;
        let mut fibo = 1u32;
        let mut previous = 1u32;

        while fibo_ord > 2 {

            let temp = fibo;

            fibo = fibo + previous;
            previous = temp;

            fibo_ord = fibo_ord - 1;
        }

        fibo
    }
}

fn main() {

    loop{
        
        println!("Enter fibonacci ordinal:");

        let mut fibo = String::new();

        io::stdin()
            .read_line(&mut fibo)
            .expect("Failed to read line");

        let fibo: u32 = match fibo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fibo = get_n_fibonacci(fibo);

        println!("Fibonacci value: {fibo}");
    }
}
