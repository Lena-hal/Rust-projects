use std::f64;

pub fn factorial(num: u64) -> u64{
    if num <= 1{
        return 1
    }
    else{
        return num * factorial(num-1)
    }

}

pub fn power(num: u64, exp: u64) -> u64{
    let mut result = 1;
    for _i in 0..exp {
        result *= num;
    }
    return result;

}

pub fn abs(num:i64) -> i64{
    if num <0{
        return num*-1
    }
    return num
}

pub fn pythagoras(a: f64, b: f64) -> f64{
    return f64::sqrt((a*a)+(b*b));
}

pub fn multiplicate(a: f64, b: f64) -> f64{
    return a*b;
}

pub fn add(a: f64, b: f64) -> f64{
    return a+b;
}

pub fn subtract(a: f64, b: f64) -> f64{
    return a-b;
}