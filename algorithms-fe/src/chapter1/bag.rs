use std::marker::PhantomData;


#[derive(Debug, Clone)]
pub struct Bag<Item> {
    phantom: PhantomData<Item>,
}

impl<Item> Bag<Item> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }

    pub fn add(&mut self, _item: Item) {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

impl<T> From<Vec<T>> for Bag<T> {
    fn from(_values: Vec<T>) -> Self {
        todo!()
    }
}

impl<Item> Iterator for Bag<Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[test]
fn test_bag() {
    // let mut numbers = Bag::<i32>::from(vec![100, 99, 101, 120, 98, 107, 109, 81, 101, 90]);
    let mut numbers = Bag::<f64>::new();

    vec![100f64, 99.0, 101.0, 120.0, 98.0, 107.0, 109.0, 81.0, 101.0, 90.0].into_iter().for_each(|value|
        numbers.add(value)
    );

    let n = numbers.size();

    let mut sum = 0.0;

    for number in numbers.clone() {
        sum += number;
    }

    let mean = sum / n as f64;

    sum = 0.0;

    for number in numbers {
        sum += (number  - mean) * (number - mean);
    }
    let std_value = (sum / (n - 1) as f64).sqrt();

    println!("mean: {}", mean);
    println!("Std dev: {}", std_value);
}
