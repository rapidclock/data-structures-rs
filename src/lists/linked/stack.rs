pub struct Stack<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn push(&mut self, elem: T) {
        let old_stack = self.head.take();
        let new_node = Box::new(Node {
            elem,
            next: old_stack,
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn iter(&self) -> StackIter<T> {
        StackIter {
            next : self.head.as_ref().map(|node| node.as_ref())
        }
    }
}

impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

impl <T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            head: None,
        }
    }
}

pub struct StackIntoIter<T> {
    stack: Stack<T>
}

impl <T> Iterator for StackIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.head.take().map(|node| {
            self.stack.head = node.next;
            node.elem
        })
    }
}

impl <T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            stack: self
        }
    }
}

pub struct StackIter<'a, T> {
    next: Option<&'a Node<T>>
}

impl <'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_i32_stack() {
        let mut list = Stack::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check peek
        assert_eq!(list.peek(), Some(&5));

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let mut list2 = Stack::new();
        list2.push(1); list2.push(2); list2.push(3);
        let expected = [3, 2, 1];
        let mut idx = 0;
        for n in list2 {
            assert_eq!(n, expected[idx]);
            idx += 1;
        }
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}