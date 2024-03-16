use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    element: i32,
    next: Link,
}

impl List {
    // NOTE: new initializes an empty List
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    // NOTE: push adds new nodes to an existing list
    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(Node {
            element,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node)
    }
    // NOTE: pop removes nodes in an existing list
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_list = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_list {
            current_list = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_single_linked_list() {
        let mut list = List::new();

        // NOTE: Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // NOTE: Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // NOTE: Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // NOTE: Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
