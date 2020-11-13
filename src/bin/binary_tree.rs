struct Tree<T> {
    value: T,
    left: Option<Tree<T>>,
    right: Option<Tree<T>>
}

impl<T> Tree<T>

where T: Ord
{
    fn new(value: T) -> Self {
        Tree {
            value: value,
            left: None,
            right: None
        }
        .into()
    }

    fn insert(&mut self, new_value: T) -> Self {
        let node = Some(Tree::new(new_value));

        if self.value > new_value {
            self.left = node;
        } else {
            self.right = node;
        }

        return node;
    }
}

fn b_search(array: mut [i32], start: i32, end: i32, k: i32) -> [i32] {

}

fn main() {
    let mut tree = Tree::new(32).insert(2);
}

fn search() {}
