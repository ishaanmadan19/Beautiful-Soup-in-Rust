extern crate bsr;
use bsr::parse::{parse_html}; // might need to add `get_and_parse_html` later
use bsr::tree::{HTMLContent};

fn main() {

    let raw_html = r#"<body>
	<div>
		<h1>foo</h1>
		<a>link</a>
	</div>
	<div>
		<p>baz<a>bar</a>qux</p>
	</div>
</body>"#;

    let tree = parse_html(raw_html).unwrap();

    println!("\n ");

    for i in tree.search(true, "raw").unwrap() {
        match i {
            HTMLContent::Raw(s) => println!("{}", s),
            HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
        }
    }

    println!("\nPreOrder Traversal:");
    for i in tree.pre_iter() {
       match i {
           HTMLContent::Raw(s) => println!("{}", s),
           HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
       }
    }

    println!("\nLevelOrder Traversal:");
    for i in tree.level_iter() {
       match i {
           HTMLContent::Raw(s) => println!("{}", s),
           HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
       }
    }


}