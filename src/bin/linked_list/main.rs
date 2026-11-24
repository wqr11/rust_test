enum List {
    Node(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        Self::Nil
    }
    fn append(self, elem: u32) -> List {
        Self::Node(elem, Box::new(self))
    }
    fn as_list(self) -> Vec<u32> {
        let mut all = Vec::<u32>::new();
        let mut current = self;
        loop {
            match current {
                List::Node(id, prev) => {
                    current = *prev;
                    all.push(id);
                }
                List::Nil => {
                    break;
                }
            }
        }
        all
    }
}

fn main() {
    let mut list = List::new();

    list = list.append(2);
    list = list.append(2);
    list = list.append(2);
    list = list.append(2);

    let a: Vec<u32> = list.as_list();

    println!("{:?}", a);
}
