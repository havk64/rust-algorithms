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
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
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
        let mut current_link: Link = self.head.take();

        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test;
