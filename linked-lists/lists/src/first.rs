use std::mem;

#[derive(Debug, PartialEq)]
pub struct List {
    head: Link,
}

#[derive(Debug, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        self.head = Link::More(Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        }));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
            Link::Empty => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Link, List, Node};

    #[test]
    fn create_linked_list() {
        let list = List::new();
        assert_eq!(List { head: Link::Empty }, list)
    }

    #[test]
    fn push_linked_list() {
        let mut list = List::new();

        list.push(6);
        list.push(2);

        assert_eq!(
            List {
                head: Link::More(Box::new(Node {
                    elem: 2,
                    next: Link::More(Box::new(Node {
                        elem: 6,
                        next: Link::Empty,
                    })),
                }))
            },
            list
        );

        let first: Option<i32> = list.pop();

        assert_eq!(first, Some(2));

        assert_eq!(
            List {
                head: Link::More(Box::new(Node {
                    elem: 6,
                    next: Link::Empty,
                }))
            },
            list
        );

        let second: Option<i32> = list.pop();

        assert_eq!(second, Some(6));

        assert_eq!(
            List {
                head: Link::Empty,
            },
            list
        );

        let last: Option<i32> = list.pop();

        assert_eq!(last, None);

    }
}
