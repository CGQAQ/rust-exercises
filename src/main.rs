mod list;



fn main() {
    let mut l = list::List::new();
    l.push_end("hello");
    l.push_end("world");
    l.push_end("pretty doh");
    l.pop_end();
//    l.pop_end();
//    l.pop_end();

    for i in l.iter().enumerate() {
        println!("{}: {}", i.0, i.1)
    }
}

//fn main() {
//    struct List {
//        head: Option<Node>,
//        len: usize
//    }
//    impl List{
//        fn new()->Self {
//            List{
//                head: None,
//                len: 0
//            }
//        }
//
//        fn push_end(&mut self, data:String) {
//            if self.len == 0 {
//                self.head = Some(Node {
//                    data,
//                    next: None,
//                })
//            } else {
//                let mut temp = self.head.as_mut().unwrap();
//
////                while let Some(n) = &mut temp.next {
////                    temp = n.next.as_mut().unwrap()
////                }
//                while let Some(ref mut n) = temp.next {
//                    temp = n.next.as_mut().unwrap()
//                }
//
//                temp.next = Some(Box::new(Node{
//                    data,
//                    next: None,
//                }))
//            }
//            self.len += 1;
//        }
//    }
//
//    struct Node {
//        data: String,
//        next: Option<Box<Node>>
//    }
//
//    let list = List::new();
//}


//
//use std::mem;
//
//fn main () {
//    #[derive(Debug)]
//    struct Demo {
//        s: String
//    }
//
//    let mut d: &mut Demo = &mut Demo{
//        s: String::from("hello world"),
//    };
//
//    let b = unsafe{
//        mem::replace(d, mem::zeroed())
//    };
//
//    println!("{:?}", b);
//
//    ;
//}