
pub struct Link {
    head: List
}

enum List {
    Empty,
    More(Box<Node>)
}
struct Node {
    value:i32,
    next:List
}