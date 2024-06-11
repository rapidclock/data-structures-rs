use std::rc::Rc;

pub struct PStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> Default for PStack<T> {
    fn default() -> Self {
        Self {
            head: None
        }
    }
}

impl <T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prepend(&self, elem: T) -> PStack<T> {
        PStack {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> PStack<T> {
        PStack {
            head : self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn iter(&self) -> PStackIter<T> {
        PStackIter {
            cur: self.head.as_deref()
        }
    }
}

pub struct PStackIter<'a, T> {
    cur : Option<&'a Node<T>>,
}

impl <'a, T> Iterator for PStackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|node| {
            self.cur = node.next.as_deref();
            &node.elem
        })
    }
}




#[cfg(test)]
mod test {
    use super::PStack;

    #[test]
    fn basics() {
        let list = PStack::new();
        assert_eq!(list.head(), None);
        
        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = PStack::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
