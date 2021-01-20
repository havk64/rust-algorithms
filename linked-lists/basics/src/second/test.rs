use super::{List, Node};

#[test]
fn create_linked_list() {
    let list: List<i32> = List::new();
    assert_eq!(List { head: None }, list)
}

#[test]
fn test_linked_list_i32() {
    let mut list: List<i32> = List::new();

    list.push(6);
    assert_eq!(Some(&6), list.peek());
    list.push(2);
    assert_eq!(Some(&2), list.peek());

    assert_eq!(
        List {
            head: Some(Box::new(Node {
                elem: 2,
                next: Some(Box::new(Node {
                    elem: 6,
                    next: None,
                })),
            }))
        },
        list
    );

    let first: Option<i32> = list.pop();

    assert_eq!(first, Some(2));

    assert_eq!(
        List {
            head: Some(Box::new(Node {
                elem: 6,
                next: None,
            }))
        },
        list
    );

    let second: Option<i32> = list.pop();

    assert_eq!(second, Some(6));

    assert_eq!(List { head: None }, list);

    let last: Option<i32> = list.pop();

    assert_eq!(last, None);
}

#[test]
fn test_create_linked_list_string() {
    let list: List<String> = List::<String>::new();
    assert_eq!(List { head: None}, list)
}

#[test]
fn test_push_linked_list_string() {
    let mut list: List<String> = List::<String>::new();
    assert_eq!(List { head: None}, list);

    list.push("Hello".to_string());
    let peek = list.peek();
    assert_eq!(Some(&"Hello".to_string()), peek);
    assert_eq!(
        List {
            head: Some(Box::new(Node {
                elem: "Hello".to_string(),
                next: None,
            })),
        },
        list
    )
}