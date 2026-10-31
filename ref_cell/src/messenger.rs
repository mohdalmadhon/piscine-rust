use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let count = Rc::strong_count(value);

        if count > self.max {
            self.messages
                .borrow_mut()
                .push(String::from("Error: You can't go over your quota!"));
            return;
        }

        *self.value.borrow_mut() = count;

        let percentage = self.percentage(count);
        if percentage > 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percentage
            ));
        }
    }

    pub fn peek<T>(&self, value: &Rc<T>) {
        let percentage = self.percentage(Rc::strong_count(value));
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }

    fn percentage(&self, count: usize) -> usize {
        count * 100 / self.max
    }
}