use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::time::{ Instant };

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref MAPPING: HashMap<String, (String, bool)> = {
        let mut map = HashMap::new();
        map.insert("doctype".to_string(), ("!DOCTYPE".to_string(), true));
        map.insert("a".to_string(), ("a".to_string(), false));
        map.insert("abbr".to_string(), ("abbr".to_string(), false));
        map.insert("address".to_string(), ("address".to_string(), false));
        map.insert("area".to_string(), ("area".to_string(), true));
        map.insert("article".to_string(), ("article".to_string(), false));
        map.insert("aside".to_string(), ("aside".to_string(), false));
        map.insert("audio".to_string(), ("audio".to_string(), false));
        map.insert("b".to_string(), ("b".to_string(), false));
        map.insert("base".to_string(), ("base".to_string(), true));
        map.insert("bdi".to_string(), ("bdi".to_string(), false));
        map.insert("bdo".to_string(), ("bdo".to_string(), false));
        map.insert("blockquote".to_string(), ("blockquote".to_string(), false));
        map.insert("body".to_string(), ("body".to_string(), false));
        map.insert("br".to_string(), ("br".to_string(), true));
        map.insert("button".to_string(), ("button".to_string(), false));
        map.insert("canvas".to_string(), ("canvas".to_string(), false));
        map.insert("caption".to_string(), ("caption".to_string(), false));
        map.insert("cite".to_string(), ("cite".to_string(), false));
        map.insert("code".to_string(), ("code".to_string(), false));
        map.insert("col".to_string(), ("col".to_string(), true));
        map.insert("colgroup".to_string(), ("colgroup".to_string(), false));
        map.insert("data".to_string(), ("data".to_string(), false));
        map.insert("datalist".to_string(), ("datalist".to_string(), false));
        map.insert("dd".to_string(), ("dd".to_string(), false));
        map.insert("del".to_string(), ("del".to_string(), false));
        map.insert("details".to_string(), ("details".to_string(), false));
        map.insert("dfn".to_string(), ("dfn".to_string(), false));
        map.insert("dialog".to_string(), ("dialog".to_string(), false));
        map.insert("div".to_string(), ("div".to_string(), false));
        map.insert("dl".to_string(), ("dl".to_string(), false));
        map.insert("dt".to_string(), ("dt".to_string(), false));
        map.insert("em".to_string(), ("em".to_string(), false));
        map.insert("embed".to_string(), ("embed".to_string(), false));
        map.insert("fieldset".to_string(), ("fieldset".to_string(), false));
        map.insert("figcaption".to_string(), ("figcaption".to_string(), false));
        map.insert("figure".to_string(), ("figure".to_string(), false));
        map.insert("footer".to_string(), ("footer".to_string(), false));
        map.insert("form".to_string(), ("form".to_string(), false));
        map.insert("h1".to_string(), ("h1".to_string(), false));
        map.insert("h2".to_string(), ("h2".to_string(), false));
        map.insert("h3".to_string(), ("h3".to_string(), false));
        map.insert("h4".to_string(), ("h4".to_string(), false));
        map.insert("h5".to_string(), ("h5".to_string(), false));
        map.insert("h6".to_string(), ("h6".to_string(), false));
        map.insert("head".to_string(), ("head".to_string(), false));
        map.insert("header".to_string(), ("header".to_string(), false));
        map.insert("hr".to_string(), ("hr".to_string(), true));
        map.insert("html".to_string(), ("html".to_string(), false));
        map.insert("i".to_string(), ("i".to_string(), false));
        map.insert("iframe".to_string(), ("iframe".to_string(), false));
        map.insert("img".to_string(), ("img".to_string(), true));
        map.insert("input".to_string(), ("input".to_string(), true));
        map.insert("ins".to_string(), ("ins".to_string(), false));
        map.insert("kbd".to_string(), ("kbd".to_string(), false));
        map.insert("label".to_string(), ("label".to_string(), false));
        map.insert("legend".to_string(), ("legend".to_string(), false));
        map.insert("li".to_string(), ("li".to_string(), false));
        map.insert("link".to_string(), ("link".to_string(), true));
        map.insert("main".to_string(), ("main".to_string(), false));
        map.insert("map".to_string(), ("map".to_string(), false));
        map.insert("mark".to_string(), ("mark".to_string(), false));
        map.insert("meta".to_string(), ("meta".to_string(), true));
        map.insert("meter".to_string(), ("meter".to_string(), false));
        map.insert("nav".to_string(), ("nav".to_string(), false));
        map.insert("noscript".to_string(), ("noscript".to_string(), false));
        map.insert("ol".to_string(), ("ol".to_string(), false));
        map.insert("optgroup".to_string(), ("optgroup".to_string(), false));
        map.insert("option".to_string(), ("option".to_string(), false));
        map.insert("output".to_string(), ("output".to_string(), false));
        map.insert("p".to_string(), ("p".to_string(), false));
        map.insert("param".to_string(), ("param".to_string(), false));
        map.insert("picture".to_string(), ("picture".to_string(), false));
        map.insert("pre".to_string(), ("pre".to_string(), false));
        map.insert("progress".to_string(), ("progress".to_string(), false));
        map.insert("q".to_string(), ("q".to_string(), false));
        map.insert("rp".to_string(), ("rp".to_string(), false));
        map.insert("rt".to_string(), ("rt".to_string(), false));
        map.insert("ruby".to_string(), ("ruby".to_string(), false));
        map.insert("s".to_string(), ("s".to_string(), false));
        map.insert("samp".to_string(), ("samp".to_string(), false));
        map.insert("script".to_string(), ("script".to_string(), false));
        map.insert("section".to_string(), ("section".to_string(), false));
        map.insert("select".to_string(), ("select".to_string(), false));
        map.insert("small".to_string(), ("small".to_string(), false));
        map.insert("source".to_string(), ("source".to_string(), true));
        map.insert("span".to_string(), ("span".to_string(), false));
        map.insert("strong".to_string(), ("strong".to_string(), false));
        map.insert("style".to_string(), ("style".to_string(), false));
        map.insert("sub".to_string(), ("sub".to_string(), false));
        map.insert("summary".to_string(), ("summary".to_string(), false));
        map.insert("sup".to_string(), ("sup".to_string(), false));
        map.insert("svg".to_string(), ("svg".to_string(), false));
        map.insert("table".to_string(), ("table".to_string(), false));
        map.insert("tbody".to_string(), ("tbody".to_string(), false));
        map.insert("td".to_string(), ("td".to_string(), false));
        map.insert("template".to_string(), ("template".to_string(), false));
        map.insert("textarea".to_string(), ("textarea".to_string(), false));
        map.insert("tfoot".to_string(), ("tfoot".to_string(), false));
        map.insert("th".to_string(), ("th".to_string(), false));
        map.insert("thead".to_string(), ("thead".to_string(), false));
        map.insert("time".to_string(), ("time".to_string(), false));
        map.insert("title".to_string(), ("title".to_string(), false));
        map.insert("tr".to_string(), ("tr".to_string(), false));
        map.insert("track".to_string(), ("track".to_string(), true));
        map.insert("u".to_string(), ("u".to_string(), false));
        map.insert("ul".to_string(), ("ul".to_string(), false));
        map.insert("var".to_string(), ("var".to_string(), false));
        map.insert("video".to_string(), ("video".to_string(), false));
        map.insert("wbr".to_string(), ("wbr".to_string(), true));

        map
    };
}

extern crate hson;
use hson::{ Hson, Node, Kind, Cast };

pub struct HsonGen {
    hson: Hson,
    process_start: Instant
}

impl HsonGen {
    pub fn new () -> HsonGen {
        HsonGen {
            hson: Hson::new(),
            process_start: Instant::now()
        }
    }

    pub fn parse (&mut self, s: &str) -> Result<(), Error> {
        self.hson.parse(s)?;

        Ok(())
    }

    pub fn generate (&mut self, s: &str) -> Result<String, Error> {
        self.parse(s)?;

        let root_id = self.hson.get_root();

        self.gen(root_id)
    }

    fn gen (&mut self, node_id: u64) -> Result<String, Error> {
        let mut html = String::from("");

        match self.hson.nodes.get(&node_id) {
            Some(n) => {
                let node = n.clone();
                let childs = node.childs.clone();
                let childs_length = childs.len();
                let key = self.hson.get_node_key(&node);
                let attributes = self.get_node_attributes(&node)?;
                let texts = self.get_node_text(&node)?;

                if let Some(results) = MAPPING.get(&key) {
                    let opening = self.get_opening_tag(&results.0);
                    let closing = self.get_closing_tag(&results.0, results.1);

                    html.push_str(&opening);
                    html.push_str(&attributes);
                    html.push_str(">");

                    if childs_length > 0 {
                        let mut i: usize = 0;

                        loop {
                            let child_id = childs[i];
                            if let Some(child) = self.hson.nodes.get(&child_id) {
                                let child_key = self.hson.get_node_key(&child);

                                if &child_key == "text" {
                                    if let Some(txt) = texts.get(&i) {
                                        html.push_str(txt);
                                    }
                                } else {
                                    let childs_html = self.gen(child_id)?;
                                    html.push_str(&childs_html);
                                }
                            }

                            i += 1;
                            if i >= childs_length {
                                break;
                            }
                        }
                    }

                    html.push_str(&closing);
                } else {
                    if node.root {
                        if childs_length > 0 {
                            let mut i = 0;

                            loop {
                                let child_id = childs[i];
                                let childs_html = self.gen(child_id)?;
                                html.push_str(&childs_html);

                                i += 1;
                                if i >= childs_length {
                                    break;
                                }
                            }
                        }
                    }
                }
            },
            None => {}
        }

        Ok(html)
    }

    fn get_opening_tag (&mut self, tag: &str) -> String {
        match tag {
            "doctype" => "<!DOCTYPE".to_string(),
            _ => {
                let mut t = String::from("<");
                t.push_str(tag);

                t
            }
        }
    }

    fn get_node_attributes (&mut self, node: &Node) -> Result<String, Error> {
        let mut attributes = String::from("");

        let mut results = Vec::new();
        let childs_length = node.childs.len();

        if childs_length > 0 {
            let mut idx = 0;

            loop {
                match self.hson.nodes.get(&node.childs[idx]) {
                    Some(n) => {
                        let k = self.hson.get_node_key(n);

                        if k == "attrs" {
                            results.push(node.childs[idx].clone());
                        }
                    },
                    None => {}
                }

                idx += 1;
                if idx >= childs_length {
                    break;
                }
            }
        }

        if results.len() > 0 {
            let mut i = 0;
            let attrs = self.hson.get_all_childs(results[0])?;
            let l = attrs.len();

            if l > 0 {
                loop {
                    match self.hson.get_vertex(attrs[i]) {
                        Some(vertex) => {
                            let key = match vertex.key_as_string() {
                                Some(s) => s,
                                None => {
                                    let e = Error::new(ErrorKind::InvalidData, format!("Cannot retrieve node key"));
                                    return Err(e);
                                }
                            };
                            let value = match vertex.value_as_string() {
                                Some(s) => s,
                                None => {
                                    let e = Error::new(ErrorKind::InvalidData, format!("Cannot retrieve node value"));
                                    return Err(e);
                                }
                            };

                            if !key.is_empty() {
                                attributes.push_str(" ");
                                attributes.push_str(&key);

                                if !value.is_empty() {
                                    attributes.push_str("=\"");

                                    match vertex.kind {
                                        Kind::Array => {
                                            let values = match vertex.value_as_array() {
                                                Some(v) => v,
                                                None => {
                                                    let e = Error::new(ErrorKind::InvalidData, format!("Cannot cast node value to vector type"));
                                                    return Err(e);
                                                }
                                            };
                                            let vl = values.len();

                                            if vl > 0 {
                                                let mut vi = 0;

                                                loop {
                                                    attributes.push_str(&values[vi]);

                                                    vi += 1;
                                                    if vi >= vl {
                                                        break;
                                                    }

                                                    attributes.push_str(" ");
                                                }
                                            }
                                        },
                                        _ => {
                                            attributes.push_str(&value);
                                        }
                                    }

                                    attributes.push_str("\"");
                                }
                            }
                        },
                        None => {}
                    }

                    i += 1;
                    if i >= l {
                        break;
                    }
                }
            }
        }

        Ok(attributes)
    }

    fn get_node_text (&mut self, node: &Node) -> Result<HashMap<usize, String>, Error> {
        let mut txt = HashMap::new();
        let mut results = Vec::new();
        let childs_length = node.childs.len();

        if childs_length > 0 {
            let mut idx = 0;

            loop {
                match self.hson.nodes.get(&node.childs[idx]) {
                    Some(n) => {
                        let k = self.hson.get_node_key(n);

                        if k == "text" {
                            results.push((idx, node.childs[idx]));
                        }
                    },
                    None => {}
                }

                idx += 1;
                if idx >= childs_length {
                    break;
                }
            }
        }

        let l = results.len();

        if l > 0 {
            let mut i = 0;

            loop {
                match self.hson.get_vertex(results[i].1) {
                    Some(vertex) => {
                        let value = match vertex.value_as_string() {
                            Some(s) => s,
                            None => {
                                let e = Error::new(ErrorKind::InvalidData, format!("Cannot retrieve node value"));
                                return Err(e);
                            }
                        };

                        txt.insert(results[i].0, value);
                    },
                    None => {}
                }

                i += 1;
                if i >= l {
                    break;
                }
            }
        }

        Ok(txt)
    }

    fn get_closing_tag(&mut self, tag: &str, auto_close: bool) -> String {
        if !auto_close {
            let mut t = String::from("</");
            t.push_str(tag);
            t.push_str(">");

            t
        } else {
            String::from("")
        }
    }

    pub fn print_process_time (&self, tag: &str) {
        let duration = self.process_start.elapsed();
        println!("{} : {:?}", tag, duration);
    }
}

