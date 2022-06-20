use std::fs::File;
use std::io::{BufRead, BufReader};

mod mwt;
use mwt::{Mwt, MwtNode};

const MIN_LEN: usize = 1;
const ELEMENTS_TO_RETURN: usize = 10;

/// This tuple is the return value used in DFS for ease
/// of sorting/use.
///
/// First element is the rank, second is the word.
type RetTup = (i32, String);

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
    pub fn from_dict(dict_filename: &String) -> Result<Autocompleter, String> {
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
                Ok(l) => val.trie.add_record(l),
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
            dfs_results.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            dfs_results.sort_by(|a, b| a.0.cmp(&b.0));

            let num_to_ret = if dfs_results.len() < ELEMENTS_TO_RETURN {
                dfs_results.len()
            } else {
                ELEMENTS_TO_RETURN
            };

            for ind in 0..num_to_ret {
                res.push(dfs_results[ind].1.clone());
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
    fn depth_first_search(node: Option<&Box<MwtNode>>) -> Vec<RetTup> {
        let mut ret: Vec<RetTup> = Vec::new();
        if let Some(nd) = node {
            let children = nd.get_children();

            if nd.get_end() {
                ret.push((nd.get_rank(), nd.get_data().to_string()));
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
