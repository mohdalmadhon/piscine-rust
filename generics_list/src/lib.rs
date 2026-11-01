#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Node {
            value,
            next: self.head.take().map(Box::new),
        });
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed| *boxed);
        }
    }

    pub fn len(&self) -> usize {
        let mut n = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            n += 1;
            current = node.next.as_deref();
        }
        n
    }
}