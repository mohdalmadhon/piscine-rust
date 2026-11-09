pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

pub fn twice<T>(f: T) -> impl Fn(i32) -> i32
where
    T: Fn(i32) -> i32,
{
    move |x| f(f(x))
}