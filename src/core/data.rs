
pub type QuadTree<T> = Option<Box<QuadTreeNode<T>>>;

pub enum QuadTreeNode<T> {
    Leaf(T),
    Branch {
        top_left: QuadTree<T>,
        top_right: QuadTree<T>,
        bottom_left: QuadTree<T>,
        bottom_right: QuadTree<T>,
    },
}

