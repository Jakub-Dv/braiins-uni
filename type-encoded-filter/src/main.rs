use impl_trait_for_tuples::impl_for_tuples;
use std::marker::PhantomData;

trait Filter<T> {
    fn test(&self, t: T) -> bool;
}

struct Simple<F, T>(F, PhantomData<T>)
where
    F: Fn(T) -> bool;

impl<F, T> Filter<T> for Simple<F, T>
where
    F: Fn(T) -> bool,
    T: Copy,
{
    fn test(&self, t: T) -> bool {
        (self.0)(t)
    }
}

struct Negate<F, T>(F, PhantomData<T>)
where
    F: Filter<T>;

impl<F, T> Filter<T> for Negate<F, T>
where
    F: Filter<T>,
    T: Copy,
{
    fn test(&self, t: T) -> bool {
        !self.0.test(t)
    }
}

struct AllOf<Fil, T>((Fil, Fil, Fil, Fil), PhantomData<T>)
where
    Fil: Filter<T>;

impl<Fil, T> Filter<T> for AllOf<Fil, T>
where
    Fil: Filter<T>,
    T: Copy,
{
    fn test(&self, t: T) -> bool {
        todo!()
        // self.0.iter().all(|s| s.test(t))
    }
}

trait TestTuple<T> {
    fn test_tuple(&self, t: T) -> Vec<bool>;
}

#[impl_for_tuples(2, 10)]
#[tuple_types_custom_trait_bound(Filter<T>)]
impl<T> TestTuple<T> for Tuple
where
    T: Copy,
{
    fn test_tuple(&self, t: T) -> Vec<bool> {
        let results: Vec<bool> = Vec::new();
        for_tuples!( #( self.Tuple.test(t.Tuple); )* );
        results
    }
}

fn main() {
    // let tester = AllOf(
    //     (
    //         Negate(Simple(|x: i32| x % 2 == 0, PhantomData), PhantomData),
    //         Simple(|x: i32| x % 3 == 0, PhantomData),
    //         Simple(|x: i32| x > 10, PhantomData),
    //         Simple(|x: i32| x < 100, PhantomData),
    //     ),
    //     PhantomData,
    // );
    //
    // for i in (0..1200).filter(|x| tester.test(*x)) {
    //     println!("matches: {}", i);
    // }
}
