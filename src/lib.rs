/// A linked list implemented using a `Vec`
pub struct VecLinkedList<T> {
    data: Vec<Option<(T, usize, usize)>>,
    head: Option<usize>,
    available: Vec<usize>,
    len: usize,
}

impl<T> VecLinkedList<T> {
    /// Constructs a new, empty `VecLinkedList<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_linked_list::VecLinkedList;
    /// let list = VecLinkedList::<i32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            head: None,
            available: Vec::new(),
            len: 0,
        }
    }
    /// Same as `new`, but with an initial capacity to reduce reallocation
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_linked_list::VecLinkedList;
    /// let list = VecLinkedList::<i32>::with_capacity(8);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: None,
            available: Vec::new(),
            len: 0,
        }
    }
    ///
    pub fn get_next_node(&self, node: usize) -> usize {
        if let Some(x) = self.data.get(node) {
            if let Some(x) = x.as_ref() {
                x.2
            } else {
                panic!("Node: {} points to None", node)
            }
        } else {
            panic!("Node: {} points outside buffer", node)
        }
    }
    ///
    pub fn get_prev_node(&self, node: usize) -> usize {
        if let Some(x) = self.data.get(node) {
            if let Some(x) = x.as_ref() {
                x.1
            } else {
                panic!("Node: {} points to None", node)
            }
        } else {
            panic!("Node: {} points outside buffer", node)
        }
    }
    /// Returns the index of the node `offset` nodes to the right of `node`.
    /// Negative numbers go left.
    ///
    /// # panics
    ///
    /// panics if `node` points to None or outside the internal Vec
    ///
    /// # Examples
    pub fn offset(&self, mut node: usize, mut offset: isize) -> usize {
        if offset < 0 {
            while offset < 0 {
                node = self.get_prev_node(node);
                offset += 1;
            }
            node
        } else if offset > 0 {
            while offset > 0 {
                node = self.get_next_node(node);
                offset -= 1;
            }
            node
        } else {
            node
        }
    }
    /// Returns the node before `self.head`
    pub fn head(&self) -> Option<usize> {
        self.head
    }
    ///
    pub fn set_head(&mut self, nhead: usize) {
        self.head = Some(nhead);
    }
    /// Returns the node before `self.head`
    pub fn tail(&self) -> Option<usize> {
        self.head.and_then(|i| Some(self.offset(i, -1)))
    }
    /// Get the amount of elements in the list
    pub fn len(&self) -> usize {
        self.len
    }
    ///
    pub fn get(&self, node: usize) -> Option<&T> {
        self.data
            .get(node)
            .and_then(|x| x.as_ref().and_then(|(x, _, _)| Some(x)))
    }
    ///
    pub fn get_mut(&mut self, node: usize) -> Option<&mut T> {
        self.data
            .get_mut(node)
            .and_then(|x| x.as_mut().and_then(|(x, _, _)| Some(x)))
    }
    //
    fn setprev(&mut self, node: usize, prev: usize) {
        if let Some(x) = self.data.get_mut(node) {
            if let Some(x) = x.as_mut() {
                x.1 = prev;
            }
        }
    }
    //
    fn setnext(&mut self, node: usize, next: usize) {
        if let Some(x) = self.data.get_mut(node) {
            if let Some(x) = x.as_mut() {
                x.2 = next;
            }
        }
    }
    /// Inserts val right after `node`. Returns index of new node.
    ///
    /// # panics
    ///
    /// panics if `node > len`
    pub fn insert(&mut self, node: usize, val: T) -> usize {
        self.len += 1;
        let next = self.offset(node, 1);
        let prev = node;
        let n_node = (val, prev, next);
        //
        let ind = if let Some(ind) = self.available.pop() {
            self.data[ind] = Some(n_node);
            ind
        } else {
            let ind = self.data.len();
            self.data.push(Some(n_node));
            ind
        };
        //
        self.setprev(next, ind);
        self.setnext(prev, ind);
        ind
    }
    /// Pushes `val` to the list. Returns the index in the internal vec
    /// to where it was put.
    pub fn push(&mut self, val: T) -> usize {
        if let Some(tail) = self.tail() {
            self.insert(tail, val)
        } else {
            self.len = 1;
            self.data.push(Some((val, 0, 0)));
            self.head = Some(0);
            0
        }
    }
    /// Removes `node`, returning the value of the node
    pub fn remove(&mut self, node: usize) -> T {
        if self.len == 1 {
            self.head = None;
        } else {
            self.head = Some(self.get_next_node(node));
        }
        //
        self.len -= 1;
        {
            let prev = self.offset(node, -1);
            let next = self.offset(node, 1);
            //
            self.setnext(prev, next);
            self.setprev(next, prev);
        }
        //
        let val = self.data[node].take().unwrap().0;
        self.available.push(node);
        //
        val
    }
    ///
    pub fn iter(&self) -> VecLinkedListIter<T> {
        VecLinkedListIter {
            v: self,
            curnode: self.head.unwrap_or(0),
            amt: 0,
        }
    }
    ///
    pub fn into_iter(self) -> VecLinkedListIntoIter<T> {
        let head = self.head;
        VecLinkedListIntoIter {
            v: self,
            curnode: head.unwrap_or(0),
        }
    }
    ///
    pub fn iter_with_start(&self, start: usize) -> VecLinkedListIter<T> {
        VecLinkedListIter {
            v: self,
            curnode: start,
            amt: 0,
        }
    }
    ///
    pub fn into_iter_with_start(
        self,
        start: usize,
    ) -> VecLinkedListIntoIter<T> {
        VecLinkedListIntoIter {
            v: self,
            curnode: start,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* -> ")?;
        for i in self.iter() {
            write!(f, "{:?} -> ", i)?;
        }
        write!(f, "*")
    }
}

impl<T> std::iter::FromIterator<T> for VecLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Self::new();
        for val in iter {
            v.push(val);
        }
        v
    }
}

pub struct VecLinkedListIter<'a, T> {
    v: &'a VecLinkedList<T>,
    curnode: usize,
    amt: usize,
}

impl<'a, T> Iterator for VecLinkedListIter<'a, T> {
    type Item = &'a T;
    //
    fn next(&mut self) -> Option<&'a T> {
        if self.amt < self.v.len() {
            let thisnode = self.curnode;
            self.curnode = self.v.offset(self.curnode, 1);
            self.amt += 1;
            //
            self.v.get(thisnode)
        } else {
            None
        }
    }
}

pub struct VecLinkedListIntoIter<T> {
    v: VecLinkedList<T>,
    curnode: usize,
}

impl<T> Iterator for VecLinkedListIntoIter<T> {
    type Item = T;
    //
    fn next(&mut self) -> Option<T> {
        if self.v.len() > 0 {
            let nextnode = self.v.offset(self.curnode, 1);
            let val = self.v.remove(self.curnode);
            self.curnode = nextnode;
            //
            Some(val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VecLinkedList;
    #[test]
    fn test1() {
        let mut v = VecLinkedList::new();
        //
        println!("{:?}", v);
        //
        v.push("Hei1");
        //
        println!("{:?}", v);
        //
        v.push("Hei2");
        let node = v.push("Hei3");
        v.push("Hei4");
        //
        println!("{:?}", v);
        //
        v.remove(node);
        //
        println!("{:?}", v);
    }
    #[test]
    fn test2() {
        let mut v: VecLinkedList<_> = "Marcus".chars().collect();
        //
        v.remove(2);
        //
        println!("{:?}", v);
        //
        println!("{:?}", v.data);
        //
        v.insert(3, 'H');
        //
        println!("{:?}", v);
        //
        println!("{:?}", v.data);
        //
        let v: Vec<_> = v.into_iter_with_start(3).collect();
        //
        println!("{:?}", v);
    }
}
