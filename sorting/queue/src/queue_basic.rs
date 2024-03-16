use std::rc::Rc;

pub struct Queue<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None }
    }

    pub fn prepend(&self, elem: T) -> Queue<T> {
        Queue {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Queue<T> {
        Queue {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_new() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.head(), None)
    }
    #[test]
    fn test_queue_head() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.head(), None);
        let q = Queue::new().prepend(1);
        assert_eq!(q.head(), Some(&1));
        let q = Queue::new().prepend(3);
        assert_eq!(q.head(), Some(&3));
    }

    #[test]
    fn test_queue_tail() {
        let q = Queue::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(q.head(), Some(&3));

        let q = q.tail();
        assert_eq!(q.head(), Some(&2));

        let q = q.tail();
        assert_eq!(q.head(), Some(&1));

        let q = q.tail();
        assert_eq!(q.head(), None);

        // Make sure empty tail works
        let q = q.tail();
        assert_eq!(q.head(), None);
    }

    #[test]
    fn test_queue_iter() {
        let q = Queue::new().prepend(1).prepend(2).prepend(3);
        let mut iter = q.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
