pub struct Tree<T> {
    pub item: T,
    pub freq: i64,
    //children: Vec<Option<Tree<T>>>,
    pub left: Option<Box<Tree<T>>>,
    pub right: Option<Box<Tree<T>>>,
}
impl<T> Tree<T> {
    pub fn new(item: T, freq: i64) -> Tree<T> {
        Tree::<T> {
            item,
            freq,
            left: None,
            right: None,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    pub fn add_left(&mut self, child: Tree<T>) {
        self.left = Some(Box::new(child));
    }
    pub fn add_right(&mut self, child: Tree<T>) {
        self.right = Some(Box::new(child));
    }
}

pub struct Heap {
    pub data: Vec<Tree<u8>>,
}
impl Heap {
    pub fn new() -> Heap {
        Heap {
            data: vec!(),
        }
    }
    pub fn enqueue(&mut self, item: Tree<u8>) {
        if self.data.len() == 0 {
            self.data.push(item);
            return;
        }
        let mut i = self.data.len();
        let mut p = ((i + 1) >> 1) - 1;
        self.data.push(item);

        while p > 0 {
            if self.data[i].freq >= self.data[p].freq {
                break;
            }
            else {
                self.data.swap(i, p);
                i = p;
                p = ((i + 1) >> 1) - 1;
            }
        }
        if self.data[i].freq < self.data[p].freq {
            self.data.swap(i, p);
        }
    }
    pub fn dequeue(&mut self) -> Result<Tree<u8>, &'static str> {
        if self.data.len() == 0 {
            return Err("Trying to dequeue an empty heap");
        }
        let node = self.data.swap_remove(0);

        self.min_heapfy(0);

        return Ok(node);
    }
    fn min_heapfy(&mut self, root: usize) {
        let left = (root << 1) + 1;
        let right = left + 1;

        if left >= self.data.len() {
            return;
        }

        if right >= self.data.len() {
            if self.data[root].freq > self.data[left].freq {
                self.data.swap(root, left);
                self.min_heapfy(left);
            }
        }else {
            if self.data[root].freq > self.data[left].freq {
                if self.data[left].freq <= self.data[right].freq {
                    self.data.swap(root, left);
                    self.min_heapfy(left);
                } else {
                    self.data.swap(root, right);
                    self.min_heapfy(right);
                }
            } else {
                if self.data[root].freq > self.data[right].freq {
                    self.data.swap(root, right);
                    self.min_heapfy(right);
                }
            }
        }
    }
}

