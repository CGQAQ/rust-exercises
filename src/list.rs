use std::mem;

pub struct List<T> {
    pub head: Option<Node<T>>,
    len: usize
}

impl <T> List<T> {
    pub fn new () -> Self {
        List {
            head: None,
            len: 0,
        }
    }

    pub fn push_end(&mut self, data: T) {
        if self.len == 0 {
            self.head = Some(Node {
                data,
                next: None,
            })
        } else {
            let mut temp: &mut Node<T> = self.head.as_mut().unwrap();

            while let Some(ref mut n) = temp.next {
                temp = &mut *n;
            }

            temp.next = Some(Box::new(Node{
                data,
                next: None,
            }))
        }
        self.len += 1;
    }

    pub fn pop_end(&mut self) -> Option<T> {
        if self.len == 0 {
            return None
        }
        else {
            let mut temp: *mut Node<T> = self.head.as_mut().unwrap();
            let mut second_last: *mut Node<T> = temp as *mut Node<T>;

            unsafe {
                while let Some(ref mut n) = (*temp).next {
                    second_last = temp;
                    temp = n.as_mut() as *mut Node<T>;
                }

                (*second_last).next = None;
                return Some(mem::replace(&mut (*temp).data, mem::zeroed()))
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: match self.head {
                Some(ref n) => Some(Box::new(n)),
                None => None
            }
        }
    }
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T> {
    pub current: Option<Box<&'a Node<T>>>
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = match &self.current {
            Some(n) => Some(&n.data),
            None => None
        };
        self.current = match self.current.as_ref() {
            Some(c) => {
                match c.next{
                    Some(ref n) => Some(Box::new(&*n)),
                    None => None
                }
            },
            None => None
        };
        r
    }
}