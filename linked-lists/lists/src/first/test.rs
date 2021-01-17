use super::{Link, List, Node};

#[test]
fn create_linked_list() {
    let list: List = List::new();
    assert_eq!(List { head: Link::Empty }, list)
}

#[test]
fn push_linked_list() {
    let mut list: List = List::new();

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
