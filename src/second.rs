use std::mem;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq, Eq)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
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
            head: Some(Box::new(Node {
                elem: 100,
                next: None,
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
