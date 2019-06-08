extern crate bsr;
use bsr::parse::{parse_html}; // might need to add `get_and_parse_html` later

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

    let res = parse_html(raw_html).unwrap();

    println!("{:?}", res);

//    let pre_iter = rusty_soup::PtPreOrderIter::new(&tree);
//    let level_iter = rusty_soup::PtLevelOrderIter::new(&tree);
//    let links = rusty_soup::PtPreOrderIter::new(&tree).find_tags("a".to_owned());

//    println!("Anchors in html:");
//    for i in tree.pre_iter().find_tags("a".to_owned()) {
//        match i {
//            rusty_soup::HTMLContent::Raw(s) => println!("{}", s),
//            rusty_soup::HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
//        }
//    }

//    let raw_html = r#"<body>
//            <div>
//                <h1>foo</h1>
//                <a>link</a>
//            </div>
//            <div>
//                <p>baz<a>bar</a>qux</p>
//            </div>
//        </body>"#;
//
//    let tree2 = parse::parse_html(raw_html);
//    println!("\n PreOrder Traversal:");
//    for i in tree2.unwrap().pre_iter() {
//        match i {
//            bsr::HTMLContent::Raw(s) => println!("{}", s),
//            bsr::HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
//        }
//    }


//    println!("\n PreOrder Traversal:");
//    for i in tree.pre_iter() {
//        match i {
//            bsr::HTMLContent::Raw(s) => println!("{}", s),
//            bsr::HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
//        }
//    }
//
//    println!("\nLevelOrder Traversal:");
//    for i in tree.level_iter() {
//        match i {
//            bsr::HTMLContent::Raw(s) => println!("{}", s),
//            bsr::HTMLContent::Tag(box_tag) => println!("{}", (**box_tag).get_tag())
//        }
//    }


}