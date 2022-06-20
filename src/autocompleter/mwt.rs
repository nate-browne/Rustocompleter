use std::collections::HashMap;

/// Type alias for ease of use.
type HeapMap = Box<HashMap<char, Option<Box<MwtNode>>>>;

/// Implementation of an individual node that makes up the MWT.
///
/// # Fields
///
/// * `is_end` (`bool`) - Indicates if a node holds a completed word
/// * `data` (`String`) - The word stored in this node, or ""
/// * `rank` (`i32`) - How many times this word appears in the dataset/is inserted
/// * `children` (`HeapMap`) - Mapping from character to `MwtNode`. For each character in inserted
/// words, we make an entry here.
///
/// The definition of `HeapMap` is given above.
pub struct MwtNode {
    is_end: bool,
    data: String,
    rank: i32,
    children: HeapMap,
}

impl MwtNode {
    /// Constructs an empty MwtNode.
    ///
    /// All fields are set to default empty values.
    fn new() -> MwtNode {
        MwtNode {
            is_end: false,
            data: String::new(),
            rank: 0,
            children: Box::new(HashMap::new()),
        }
    }

    /// Accessor method for the word held at this node.
    ///
    /// # Return value
    ///
    /// Reference to the `data` field of the given `MwtNode`.
    pub fn get_data(&self) -> &String {
        &self.data
    }

    /// Accessor method for the count of appearances of a finished word.
    /// 
    /// # Return value
    /// 
    /// Copy of the `rank` field of the given `MwtNode`.
    pub fn get_rank(&self) -> i32 {
        self.rank
    }

    /// Accessor method for the end marker of a `MwtNode`.
    ///
    /// # Return value
    ///
    /// Copy of the `is_end` field of the given `MwtNode`.
    pub fn get_end(&self) -> bool {
        self.is_end
    }

    /// Accessor method for the `children` map of a `MwtNode`.
    ///
    /// # Return value
    ///
    /// Reference of the `children` field of the given `MwtNode`.
    pub fn get_children(&self) -> &HeapMap {
        &self.children
    }

    /// Mutator method for the `rank` of a finished word.
    /// Simply increments the field by one. Used whenever
    /// a word is inserted/re-inserted.
    fn increment_rank(&mut self) {
        self.rank += 1;
    }

    /// Mutator method for the `data` field of a `MwtNode`.
    ///
    /// # Arguments
    ///
    /// * `data` (`String`) - New value to set. Consumed by the function.
    fn set_data(&mut self, data: String) {
        self.data = data;
    }

    /// Mutator method for the `is_end` field of a `MwtNode`.
    ///
    /// Used when a word is updated to mark the node as containing a finished word.
    fn toggle_end(&mut self) {
        self.is_end = !self.is_end;
    }
}

/// Implementation of the `MWT` itself.
///
/// The structure is quite simple, only consisting of a root node
/// and methods to act on that node.
///
/// # Fields
///
/// `root` (`Box<MwtNode>`) - Base node of the structure.
pub struct Mwt {
    root: Box<MwtNode>,
}

impl Mwt {
    /// Constructs a new `MWT`.
    /// This operation consists of simply constructing the `root`.
    pub fn new() -> Mwt {
        Mwt {
            root: Box::new(MwtNode::new()),
        }
    }

    /// Accessor method for the `root`.
    ///
    /// # Return value
    ///
    /// Returns the reference to the `root` field.
    pub fn get_root(&self) -> &Box<MwtNode> {
        &self.root
    }

    /// Adds a new string to the MWT.
    ///
    /// Iterates through the string to insert, creating
    /// new `MwtNode`s as needed until the entire string is traversed,
    /// then inserts the word at that node.
    ///
    /// # Arguments
    ///
    /// * `data` (`String`) - New word to insert
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
