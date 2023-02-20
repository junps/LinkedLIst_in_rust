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
}
