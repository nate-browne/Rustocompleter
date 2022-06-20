use std::collections::HashMap;

type HeapMap = Box<HashMap<char, Option<Box<MwtNode>>>>;
pub struct MwtNode {
    is_end: bool,
    data: String,
    rank: i32,
    children: HeapMap,
}

impl MwtNode {
    fn new() -> MwtNode {
        MwtNode {
            is_end: false,
            data: String::new(),
            rank: 0,
            children: Box::new(HashMap::new()),
        }
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    fn increment_rank(&mut self) {
        self.rank += 1;
    }

    pub fn get_rank(&self) -> i32 {
        self.rank
    }

    fn set_data(&mut self, data: String) {
        self.data = data;
    }

    fn toggle_end(&mut self) {
        self.is_end = !self.is_end;
    }

    pub fn get_end(&self) -> bool {
        self.is_end
    }

    pub fn get_children(&self) -> &HeapMap {
        &self.children
    }
}

pub struct Mwt {
    root: Box<MwtNode>,
}

impl Mwt {
    pub fn new() -> Mwt {
        Mwt {
            root: Box::new(MwtNode::new()),
        }
    }

    pub fn get_root(&self) -> &Box<MwtNode> {
        &self.root
    }

    pub fn add_record(&mut self, data: String) {
        let mut tmp = &mut self.root;

        // Traverse MWT character by character
        for ch in data.chars() {
            let children = &mut tmp.children;
            // If the value isn't present, add it to the map
            if !children.contains_key(&ch) {
                children.insert(ch, Some(Box::new(MwtNode::new())));
            }
            tmp = match children.get_mut(&ch).unwrap() {
                Some(nd) => nd,
                None => panic!("Unreachable code hit: existing child had non-existing node!"),
            }
        }

        // Insert the new word at the end
        if !tmp.get_end() {
            tmp.toggle_end();
            tmp.set_data(data);
        }
        tmp.increment_rank(); // Increase number of times we've seen this word
    }
}
