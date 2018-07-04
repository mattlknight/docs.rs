use error::{Result, Error};

use html5ever::serialize::{serialize, SerializeOpts};
use html5ever::rcdom::{RcDom, NodeData, Handle};
use html5ever::driver::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;

/// Extracts the contents of the `<head>` and `<body>` tags from an HTML document.
pub fn extract_head_and_body(html: &str) -> Result<(String, String)> {
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html);

    let (head, body) = extract_from_rcdom(&dom)?;

    Ok((stringify(head), stringify(body)))
}

fn extract_from_rcdom(dom: &RcDom) -> Result<(Handle, Handle)> {
    let mut worklist = vec![dom.document.clone()];
    let (mut head, mut body) = (None, None);

    while let Some(handle) = worklist.pop() {
        match handle.data {
            NodeData::Element { ref name, .. } => match name.local.as_ref() {
                "head" => {
                    if head.is_some() {
                        return Err("duplicate <head> tag".into());
                    } else {
                        head = Some(handle.clone());
                    }
                }
                "body" => {
                    if body.is_some() {
                        return Err("duplicate <body> tag".into());
                    } else {
                        body = Some(handle.clone());
                    }
                }
                _ => {}  // do nothing
            }
            _ => {}  // do nothing
        }

        worklist.extend(handle.children.borrow().iter().cloned());
    }

    let head = head.ok_or_else(|| Error::from("couldn't find <head> tag in rustdoc output"))?;
    let body = body.ok_or_else(|| Error::from("couldn't find <body> tag in rustdoc output"))?;
    Ok((head, body))
}

fn stringify(node: Handle) -> String {
    let mut vec = Vec::new();
    serialize(&mut vec, &node, SerializeOpts::default())
        .expect("serializing into buffer failed");

    String::from_utf8(vec).expect("html5ever returned non-utf8 data")
}
