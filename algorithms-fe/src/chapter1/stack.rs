use std::marker::PhantomData;



pub struct Stack<Item> {
    phantom: PhantomData<Item>,
}

impl<Item> Stack<Item> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }

    pub fn push(&mut self, _item: Item) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<Item> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
    pub fn size(&self) -> usize {
        todo!()
    }
}

impl<Item> Iterator for Stack<Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
