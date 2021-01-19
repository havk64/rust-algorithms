use std::mem;

type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
pub struct List {
    head: Link,
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        self.head = Some(Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        }));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link: Link = mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod test;
