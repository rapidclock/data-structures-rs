use std::rc::Rc;

pub struct PStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl <T> Default for PStack<T> {
    fn default() -> Self {
        Self {
            head: None
        }
    }
}


#[cfg(test)]
mod test {
    use super::PStack;

    #[test]
    fn basics() {
        let list = PStack::new();
        // assert_eq!(list.head(), None);
        //
        // let list = list.prepend(1).prepend(2).prepend(3);
        // assert_eq!(list.head(), Some(&3));
        //
        // let list = list.tail();
        // assert_eq!(list.head(), Some(&2));
        //
        // let list = list.tail();
        // assert_eq!(list.head(), Some(&1));
        //
        // let list = list.tail();
        // assert_eq!(list.head(), None);
        //
        // // Make sure empty tail works
        // let list = list.tail();
        // assert_eq!(list.head(), None);
    }
}
