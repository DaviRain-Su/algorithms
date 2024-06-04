use rand::Rng;

#[derive(Debug)]
pub struct MyVec<'a, T> {
    pub data: &'a mut [T],
}

impl<'a, T> MyVec<'a, T>
where
    T: Clone,
{
    pub fn new(data: &'a mut [T]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /* 随机访问元素 */
    pub fn random_access(&self) -> T {
        // 在区间 [0, nums.len()) 中随机抽取一个数字
        let random_index = rand::thread_rng().gen_range(0..self.len());
        // 获取并返回随机元素
        let random_num = &self.data[random_index];
        random_num.clone()
    }

    /* 在数组的索引 index 处插入元素 num */
    pub fn insert(&mut self, num: T, index: usize) {
        // 把索引 index 以及之后的所有元素向后移动一位
        for i in (index + 1..self.len()).rev() {
            self.data[i] = self.data[i - 1].clone();
        }
        // 将 num 赋给 index 处的元素
        self.data[index] = num;
    }

    /* 删除索引 index 处的元素 */
    pub fn remove(&mut self, index: usize) {
        // 把索引 index 之后的所有元素向前移动一位
        for i in index..self.len() - 1 {
            self.data[i] = self.data[i + 1].clone();
        }
    }

    /* 遍历数组 */
    pub fn traverse<F>(&mut self, func: F)
    where
        T: Clone,
        F: FnMut(T),
    {
        self.data.iter_mut().map(|v| v.clone()).for_each(func)
    }
}
