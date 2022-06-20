# Rustocompleter

This is a pretty basic "autocomplete" like functionality wrapped into a little script for toy purposes.
It can be initialized via either a dictionary of words (file with 1 word per line) or empty and
manually added to.

Autocomplete functionality is provided via a [Multi-way Trie data structure](https://en.wikipedia.org/wiki/Trie).
Another option would have been to use a [ternary search trie](https://en.wikipedia.org/wiki/Ternary_search_tree) but
I find multiway tries to be easier to implement despite being less memory efficient.