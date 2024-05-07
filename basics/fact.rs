fn factorial(n: u64) -> u64{
    if n == 0 {
        return 1;
    }else{
        return n * factorial(n - 1);
    }
}

fn main(){
    let y = factorial(5);
    println!("{}",y);
}

