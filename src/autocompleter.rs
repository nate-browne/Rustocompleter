extern crate fs_err;
use fs_err::File;

use std::io::{BufRead, BufReader};

mod mwt;
use mwt::{Mwt, MwtNode};

const MIN_LEN: usize = 1;
const ELEMENTS_TO_RETURN: usize = 10;

/// This struct contains functionality related to performing
/// word autocompletion. It acts as a sort of wrapper class
/// for the underlying MWT.
///
/// # Fields
///
/// `trie` (`Mwt`) - The underlying MWT structure that provides the functionality.
pub struct Autocompleter {
    trie: Mwt,
}

/// This internal struct is used to store the results from the DFS
/// It's functionally identical to a tuple of `(count, data)`, just with the
/// added benefit of being able to reference fields by name instead of by index.
///
/// # Fields
///
/// `count` (`i32`) - number of instances of a particular word
///
/// `data` (`String`) - the word itself
struct SortResult {
    count: i32,
    data: String,
}

impl SortResult {
    fn new(count: i32, data: String) -> SortResult {
        SortResult { count, data }
    }
}

impl Autocompleter {
    /// Constructs a new, empty `Autocompleter`.
    pub fn new() -> Autocompleter {
        Autocompleter { trie: Mwt::new() }
    }

    /// Constructs a new `Autocompleter` and fills it in with the values
    /// from a given file.
    ///
    /// # Arguments
    ///
    /// `dict_filename` (`&String`) - Name of the file to parse for the dictionary.
    ///
    /// # Return value
    ///
    /// Either the constructed `Autocompleter`, or a `Error` with the error string.
    pub fn from_file(dict_filename: &String) -> Result<Autocompleter, String> {
        let mut val = Autocompleter::new();

        // Try to open the file for reading, or bail out if an error occurs.
        let dict_file = match File::open(dict_filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error opening file `{dict_filename}`: {e}")),
        };

        // Read through the file line by line
        let reader = BufReader::new(dict_file);
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    for mut word in l.split_whitespace() {
                        word = word.trim_end_matches(|c: char| c.is_ascii_punctuation());
                        val.trie.add_record(word.to_string());
                    }
                }
                Err(e) => return Err(format!("Error reading line from file: {e}")),
            }
        }

        Ok(val)
    }

    /// Adds a word to the `Autocompleter`.
    ///
    /// Delegates to the underlying `Mwt` subroutine.
    ///
    /// # Arguments
    ///
    /// `word` (`String`) - Word to add to the structure.
    pub fn add_word(&mut self, word: String) {
        self.trie.add_record(word);
    }

    /// Runs a prediction check for a given prefixed String.
    ///
    /// This prediction check is accomplished by traversing the MWT as
    /// far down as possible, then it runs a depth-first search to traverse
    /// the rest of the MWT to grab finished words.
    ///
    /// From there, the autocompleter returns the top 10 most popular words sorted
    /// first on alphabetical order and second by the frequency.
    ///
    /// # Arguments
    ///
    /// `prefix` (`String`) - Word to search for, either complete or the beginning.
    ///
    /// # Return value
    ///
    /// This function returns a vector of strings that corresponds to the predictions.
    pub fn predict_completions(&self, prefix: &String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut tmp = self.trie.get_root();

        if prefix.len() >= MIN_LEN {
            // Walk down the Trie as far as we can
            for ch in prefix.chars() {
                let children = tmp.get_children();

                if !children.contains_key(&ch) {
                    return res;
                }
                tmp = match children.get(&ch).unwrap() {
                    Some(nd) => nd,
                    None => panic!("Unreachable code hit: existing child had non-existing node!"),
                }
            }
            // Run DFS to get all completion predictions
            let mut dfs_results = Autocompleter::depth_first_search(Some(tmp));

            // Sort by alphabetical order first, then stable sort on frequency second
            // Frequency sort should be reversed from largest to smallest
            dfs_results.sort_unstable_by(|a, b| a.data.cmp(&b.data));
            dfs_results.sort_by(|a, b| b.count.cmp(&a.count));

            let num_to_ret = if dfs_results.len() < ELEMENTS_TO_RETURN {
                dfs_results.len()
            } else {
                ELEMENTS_TO_RETURN
            };

            for ind in 0..num_to_ret {
                res.push(dfs_results[ind].data.clone());
            }
        }
        res
    }

    /// This function is used in the second half of `predict_completions`.
    /// Once the correct ending node of the prefix is found, we recursively
    /// search the rest of the Trie looking for all completed words and add
    /// them to the return vector.
    ///
    /// # Arguments
    ///
    /// `node` (`Option<&Box<MwtNode>>`) - Current node in the MWT we are searching
    ///
    /// # Return value
    ///
    /// A vector of tuples, where the first value is the frequency and the second is the
    /// word corresponding to that frequency.
    fn depth_first_search(node: Option<&Box<MwtNode>>) -> Vec<SortResult> {
        let mut ret: Vec<SortResult> = Vec::new();
        if let Some(nd) = node {
            let children = nd.get_children();

            if nd.get_end() {
                ret.push(SortResult::new(nd.get_rank(), nd.get_data().to_string()));
            }

            for value in children.values() {
                let recursive_res = Autocompleter::depth_first_search(value.as_ref());
                if !recursive_res.is_empty() {
                    for item in recursive_res {
                        ret.push(item);
                    }
                }
            }
        }
        ret
    }
}
