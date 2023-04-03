use std::f64;

fn main() {
    println!("Zadej číslo, pro které chceš vypočítat faktoriál:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    let result = factorial(n);
    println!("Faktoriál z {} je {}.", n, result);
}

fn factorial(num: u64) -> u64{
    if num <= 1{
        return 1
    }
    else{
        return num * factorial(num-1)
    }

}

fn power(num: u64, exp: u64) -> u64{
    let mut result = 1;
    for _i in 0..exp {
        result *= num;
    }
    return result;

}

fn abs(num:i64) -> i64{
    if num <0{
        return num*-1
    }
    return num
}

fn pythagoras(a: f64, b: f64) -> f64{
    return f64::sqrt((a*a)+(b*b));
}