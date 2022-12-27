use std::marker::PhantomData;



pub struct Queue<Item> {
    phantom: PhantomData<Item>,
}

impl<Item> Queue<Item> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }

    pub fn enqueue(&mut self, _item: Item) {
        todo!()
    }

    pub fn dequeue(&mut self) -> Option<Item> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
    pub fn size(&self) -> usize {
        todo!()
    }
}

impl<Item> Iterator for Queue<Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
