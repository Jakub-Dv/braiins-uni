use std::env::args;

trait Pipe {
    /// take Self by ownership, returning T
    fn pipe<F, T>(self, apply: F) -> T
    where
        F: Fn(Self) -> T,
        Self: Sized;
}

trait PipeMut {
    /// take Self by &mut, returning T
    fn pipe_mut<F, T>(&mut self, apply: F) -> T
    where
        F: FnMut(&mut Self) -> T;

    /// take Self by &mut, ignoring the result of the closure, returning &self
    fn pipe_ignore_mut<F, T>(&mut self, apply: F) -> &mut Self
    where
        F: Fn(&mut Self) -> T;
}

trait PipeRef {
    /// take Self by &, returning T
    fn pipe_ref<F, T>(&self, apply: F) -> T
    where
        F: Fn(&Self) -> T;

    /// take Self by &, returning &self
    fn pipe_ignore<F, T>(&self, apply: F) -> &Self
    where
        F: Fn(&Self) -> T;
}

impl<X> Pipe for Option<X> {
    fn pipe<F, T>(self, apply: F) -> T
    where
        F: Fn(Self) -> T,
    {
        apply(self)
    }
}

impl PipeRef for String {
    fn pipe_ref<F, T>(&self, _apply: F) -> T
    where
        F: Fn(&Self) -> T,
    {
        todo!()
    }

    fn pipe_ignore<F, T>(&self, apply: F) -> &Self
    where
        F: Fn(&Self) -> T,
    {
        apply(&self);
        self
    }
}

impl Pipe for &String {
    fn pipe<F, T>(self, apply: F) -> T
    where
        F: Fn(Self) -> T,
        Self: Sized,
    {
        apply(self)
    }
}

impl PipeMut for usize {
    fn pipe_mut<F, T>(&mut self, _apply: F) -> T
    where
        F: FnMut(&mut Self) -> T,
    {
        todo!()
    }

    fn pipe_ignore_mut<F, T>(&mut self, apply: F) -> &mut Self
    where
        F: Fn(&mut Self) -> T,
    {
        apply(self);
        self
    }
}

impl Pipe for &mut usize {
    fn pipe<F, T>(self, apply: F) -> T
    where
        F: Fn(Self) -> T,
        Self: Sized,
    {
        apply(self)
    }
}

impl PipeRef for usize {
    fn pipe_ref<F, T>(&self, _apply: F) -> T
    where
        F: Fn(&Self) -> T,
    {
        todo!()
    }

    fn pipe_ignore<F, T>(&self, apply: F) -> &Self
    where
        F: Fn(&Self) -> T,
    {
        apply(self);
        self
    }
}

fn main() {
    args()
        .skip(1)
        .next()
        .pipe(|o| o.unwrap())
        .pipe_ignore(|s| println!("received string: {}", s))
        .pipe(|s| s.chars().map(|c| c as u8 as usize).sum::<usize>())
        .pipe_ignore_mut(|s| println!("magic number: {}", s))
        .pipe(|s| *s + 1)
        .pipe_ignore(|s| println!("incremented: {}", s));
}
