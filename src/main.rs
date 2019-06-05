

#[macro_use]
extern crate nom;

//// beginning of object ideas
//
struct ParseTree<'a> {
    roots: Vec<HTMLContent>,
    stack: Option<Vec<&'a HTMLContent>>,
}

struct Tag {
    tag_type: String,
    //        attributes: Vec<String>, // not sure which to use, hashmap bigger but clunkier
    attributes: HashMap<String, String>,
    content: Vec<HTMLContent>,
}

enum HTMLContent {
    Raw(String),
    Tag(Box<Tag>),
}

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


named!(html_tag_type<&str,&str>,
    ws!(complete!(nom::alpha)) //might be alphanumeric?
);

named!(html_start_tag<&str,(&str,&str,&str)>,
        tuple!(
            tag_s!("<"),
            html_tag_type,
            tag_s!(">")
        )
);

named!(html_end_tag<&str,(&str,&str,&str)>,
        tuple!(
            tag_s!("</"),
            html_tag_type,
            tag_s!(">")
        )
);


#[derive(Debug)]
struct HTML_Tag {
    tag_type: String,
    content: String,
}

named!(get_tag<&str, HTML_Tag>,
        do_parse!(
            start: html_start_tag >>
            c: complete!(nom::alphanumeric) >>
            end:html_end_tag
            >>
            (HTML_Tag{tag_type: start.1.to_owned(), content: c.to_owned()}) //check if start tag == end tag
        )
    );

named!(get_tags<&str, Vec<HTML_Tag>>,
        many1!(complete!(get_tag))
    );


fn main() {
    let res = html_start_tag("<foo >");
    println!("{:?}",res);
    let res = html_end_tag("</foo >");
    println!("{:?}",res);
    let res = get_tag("<foo >foobar</foo >");
    println!("{:?}",res);
    let res = get_tags("<foo>foobar</foo><buz>baz</buz>");
    println!("{:?}",res);
}