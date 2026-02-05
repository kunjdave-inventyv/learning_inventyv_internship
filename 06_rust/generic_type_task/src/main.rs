fn sum<T, U>(a: T, b: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    a.into() + b.into()
}

fn main() {
    let value = sum( -2 , 2.1);
    println!("{}" , value);
}
