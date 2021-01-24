use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(
            Rc::new(
                Node {
                    elem,
                    next: self.head.take(),
                }
            )
        )
    }
    pub fn append(&self, elem: T) -> List<T> {
        List { 
            head: Some(
                Rc::new(
                    Node {
                        elem,
                        next: self.head.clone(),
                    }
                )
            )
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node|
            node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node|
            &node.elem
        )
    }
}
