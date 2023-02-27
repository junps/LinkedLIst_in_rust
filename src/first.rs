use std::mem;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug, PartialEq, Eq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // The code below maybe also fine.
        // match &mut self.head {
        //     Link::Empty => None,
        //     Link::More(node) => {
        //         let res = node.elem;
        //         self.head = mem::replace(&mut node.next, Link::Empty);
        //         Some(res)
        //     }
        // }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push() {
        let mut l1 = List::new();
        l1.push(100);
        let expected_list = List {
            head: Link::More(Box::new(Node {
                elem: 100,
                next: Link::Empty,
            })),
        };
        assert_eq!(l1, expected_list);
    }

    #[test]
    fn pop() {
        let mut l1 = List::new();
        assert_eq!(l1.pop(), None);
        l1.push(100);
        assert_eq!(l1.pop(), Some(100));
        assert_eq!(l1, List::new());
    }

    #[test]
    fn drop() {
        // if you comment out the Drop implementation for List, this code will cause stack overflow
        // because the default drop will be done recursively.
        let mut list = List::new();
        for _ in 0..100000 {
            list.push(0);
        }
    }
}
