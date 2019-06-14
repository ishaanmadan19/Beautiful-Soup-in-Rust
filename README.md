# Beautiful-Soup-in-Rust

To generate the parse tree use one of the following functions.

For already downloaded html files:

let tree = parse_html(path/to/html).unwrap();

For html from urls:

let tree = get_and_parse_html("url").unwrap();

Iterate over the tree with:

for i in tree.iter(T or F):

with true for pre-order and false for level-order

