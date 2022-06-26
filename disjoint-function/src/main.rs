fn result_string_to_option_i32<E>(res: Result<String, E>) -> Option<i32> {
    res.ok()?.parse::<i32>().ok()
}

fn options_to_iterator<T: 'static>(
    a: Option<T>,
    b: Option<T>,
    c: Option<T>,
) -> Box<dyn Iterator<Item = T>> {
    Box::new([a, b, c].into_iter().flatten())
}

fn whatever<T, E>(val: Option<Result<Option<T>, E>>) -> Option<Result<T, E>> {
    val.unwrap().transpose()
}

fn whatever2<T, E>(val: Option<Result<Option<T>, E>>) -> Result<Option<T>, E> {
    val.unwrap()
}

fn whatever3<T, E>(val: Option<Result<Option<T>, E>>) -> Option<T> {
    val?.ok()?
}

fn main() {
    println!("Hello, world!");
}
