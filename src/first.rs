#[derive(Debug)]
struct Node {
    elem: i32,
    next: List,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print() {
        let l1 = List { head: Link::Empty };
        let l2 = List {
            head: Link::More(Box::new(Node {
                elem: 1,
                next: List { head: Link::Empty },
            })),
        };
        println!("{:?}", l1);
        println!("{:?}", l2);
    }
}
