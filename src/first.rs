
pub struct List {
    head:Link
}

enum Link {
    Empty,
    More(Box<Node>)
}
struct Node {
    value:i32,
    next:List
}

impl List {
    pub fn new () -> Self {
       List{
        head:Link::Empty
       } 
    }
    pub fn push (&mut self,elem:i32) {
        
    }     
}
