use std::fmt::{Debug, Pointer};

trait LinkedList<T> {
    fn len(&self) -> usize;
    fn head(&self) -> Option<&T>;
    fn head_mut(&mut self) -> Option<&mut T>;
    fn pop(self) -> (Option<T>, Self);
    fn push(self, elem: T) -> Self;
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
}

#[derive(Debug)]
struct Item<T> {
    value: T,
    rest: Box<MyType<T>>,
}

#[derive(Debug)]
struct MyType<T> {
    data: Option<Item<T>>,
}

impl<T> LinkedList<T> for MyType<T> {
    fn len(&self) -> usize {
        let mut count = 0;
        let mut node = self;
        loop {
            match &node.data {
                Some(data) => node = &data.rest,
                None => break,
            };
            count += 1;
        }
        count
    }

    fn head(&self) -> Option<&T> {
        match &self.data {
            Some(ref item) => Some(&item.value),
            None => None,
        }
    }

    fn head_mut(&mut self) -> Option<&mut T> {
        match *(&mut self.data) {
            Some(ref mut item) => Some(&mut item.value),
            None => None,
        }
    }

    fn pop(self) -> (Option<T>, Self) {
        match self.data {
            Some(item) => (Some(item.value), *item.rest),
            None => (None, self),
        }
    }

    fn push(self, elem: T) -> Self {
        MyType {
            data: Some(Item {
                value: elem,
                rest: Box::new(self),
            }),
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        let mut node = self;
        for _ in 0..index {
            match &node.data {
                Some(item) => node = &item.rest,
                None => return None,
            };
        }
        match &node.data {
            Some(item) => Some(&item.value),
            None => None,
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut node = self;
        for _ in 0..index {
            match &mut node.data {
                Some(item) => node = &mut item.rest,
                None => return None,
            }
        }
        match *(&mut node.data) {
            Some(ref mut item) => Some(&mut item.value),
            None => None,
        }
    }
}

fn main() {
    let empty_list: MyType<i32> = MyType { data: None };
    let empty_len = empty_list.len();

    let list = MyType {
        data: Some(Item {
            value: 1,
            rest: Box::new(MyType { data: None }),
        }),
    };

    let len_list = list.len();
    println!("{:?}", empty_list);
    println!("len = {}", empty_len);

    println!("{:?}", list);
    println!("len = {}", len_list);

    let (item, popped_list) = list.pop();
    println!("item = {:?}, popped_list = {:?}", &item, &popped_list);

    let mut pushed_list = popped_list.push(1).push(2).push(3).push(4);
    println!("pushed_list = {:?}", &pushed_list);
    println!("pushed_list len = {}", &pushed_list.len());

    let head_mut = pushed_list.head_mut();
    match head_mut {
        Some(value) => *value = 66,
        None => {}
    }

    println!("updated pushed_list = {:?}", &pushed_list);

    let first = &pushed_list.get(0);
    println!("first elem = {}", first.unwrap());

    let mut second_mutable = pushed_list.get_mut(1);
    match second_mutable {
        Some(value) => *value = 12,
        None => {}
    }

    println!("updated pushed_list = {:?}", &pushed_list);
}
