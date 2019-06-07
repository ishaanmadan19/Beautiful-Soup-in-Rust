



//use bsr::*;

mod parse;


//// beginning of object ideas
//
//struct ParseTree<'a> {
//    roots: Vec<HTMLContent>,
//    stack: Option<Vec<&'a HTMLContent>>,
//}
//
//struct Tag {
//    tag_type: String,
//    //        attributes: Vec<String>, // not sure which to use, hashmap bigger but clunkier
//    attributes: HashMap<String, String>,
//    content: Vec<HTMLContent>,
//}
//
//enum HTMLContent {
//    Raw(String),
//    Tag(Box<Tag>),
//}

//
//
//named!(parse_html<&str, ParseTree>,
//        do_parse!(
//            tag_s!("<body>") >>
//            main: mult_parse >>
//            tag_s!("</body>")
//            >>
//            (ParseTree{roots: main})
//        )
//    );
//
//named!(mult_parse<&str, Vec<HTMLContent>,
//        many0!(complete!(parse))
//    );
//
//named!(parse<&str, HTMLContent,
//        do_parse!(
//            start: html_start_tag >>
//            c: complete!(nom::alphanumeric) >>
//            end:html_end_tag
//            >>
//            (HTML_Tag{tag_type: start.1.to_owned(), content: c.to_owned()}) //check if start tag == end tag
//        )
//    );
//
//named!(tag_parse<&str, HTMLContent,
//        do_parse!(
//            start: html_start_tag >>
//            c: multparse >>
//            end:html_end_tag
//            >>
//            (HTML_Tag{tag_type: start.1.to_owned(), content: c.to_owned()}) //check if start tag == end tag
//        )
//    );
//
//named!(raw_parse<&str, HTMLContent,
//        do_parse!(
//
//        )
//    );
//
//
//
//
//named!(tag_type<&str,&str>,
//    ws!(complete!(nom::alpha)) //might be alphanumeric?
//);
//
//named!(start_tag<&str,(&str,&str,&str)>,
//        tuple!(
//            tag_s!("<"),
//            html_tag_type,
//            tag_s!(">")
//        )
//);
//
//named!(end_tag<&str,(&str,&str,&str)>,
//        tuple!(
//            tag_s!("</"),
//            html_tag_type,
//            tag_s!(">")
//        )
//);
//
//
//// IMPLEMENT SELF CLOSING TYPES, hard code which ones should be self closing???
//
//// end of object ideas
//
//// MUST PASS: <foo>baz<qux>buz</qux>baz</foo>
//
//
//
//// SOME TESTING, SEMI EXAMPLES

//
//named!(html_tag_type<&str,&str>,
//    ws!(complete!(nom::alphanumeric)) //might be alphanumeric?
//);
//
//named!(html_start_tag<&str,(&str,&str,&str)>,
//        tuple!(
//            tag_s!("<"),
//            html_tag_type,
//            tag_s!(">")
//        )
//);
//
//named!(html_end_tag<&str,(&str,&str,&str)>,
//        tuple!(
//            tag_s!("</"),
//            html_tag_type,
//            tag_s!(">")
//        )
//);
//
//
//#[derive(Debug)]
//struct HTML_Tag {
//    tag_type: String,
//    content: String,
//}
//
//named!(get_tag<&str, HTML_Tag>,
//        do_parse!(
//            start: html_start_tag >>
//            c: complete!(nom::alphanumeric) >>
//            html_end_tag
//            >>
//            (HTML_Tag{tag_type: {}start.1.to_owned(), content: c.to_owned()}) //check if start tag == end tag
//        )
//    );
//
//named!(get_tags<&str, Vec<HTML_Tag>>,
//        many1!(complete!(get_tag))
//    );


fn main() {


    let tree = bsr::ParseTree::testing_tree();

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