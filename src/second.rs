#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
