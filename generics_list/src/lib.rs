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
        List{head:None}
    }
    
    pub fn push(&mut self, value: T) {
        match self.head {
            None => {
                self.head = Some(Node{value, next: None});
            },
            _ => {
                self.head = Some(Node {
                    value,
                    next: self.head.take().map(Box::new),
                });
            }
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
    
    pub fn pop(&mut self) {
    if self.head.is_none() {
        return;
    }

    if self.head.as_ref().unwrap().next.is_none() {
        self.head = None;
        return;
    }

    let mut current = self.head.as_mut().unwrap();
    while current.next.as_ref().unwrap().next.is_some() {
        current = current.next.as_deref_mut().unwrap();
    }
    current.next = None;
}
}

