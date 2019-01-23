use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::time::{ Instant };

extern crate serde_json;
use serde_json::Value;

extern crate hson;
use hson::{ Hson, Node, Kind, Query, Ops, Cast, Debug };


pub struct Tags {
    tag: String,
    auto_close: bool,
    attrs: HashMap<String, Value>,
    text: String
}

pub struct Coyote {
    mapping: Value,
    hson: Option<Hson>,
    process_start: Instant
}

impl Coyote {
    pub fn new () -> Result<Coyote, Error> {
        let bytes = include_bytes!("map.json");
        let json = String::from_utf8_lossy(bytes).into_owned();

        let mut coyote = Coyote {
            mapping: serde_json::from_str(&json)?,
            hson: None,
            process_start: Instant::now()
        };

        Ok(coyote)
    }

    pub fn parse (&mut self, s: &str) -> Result<(), Error> {
        let mut hson = Hson::new();
        hson.parse(s)?;

        self.hson = Some(hson);

        Ok(())
    }

    pub fn generate (&mut self, node_id: &str) -> Result<String, Error> {
        let mut html = String::from("");

        match self.hson {
            Some(ref hson) => {
                match hson.nodes.get(node_id) {
                    Some(n) => {
                        let node = n.clone();
                        let childs = node.childs.clone();
                        let childs_length = childs.len();
                        let uid = node.id.clone();
                        let key = hson.get_node_key(&node);
                        let attributes = self.get_node_attributes(&node)?;
                        let text = self.get_node_text(&node)?;

                        match self.mapping.get(key) {
                            Some(t) => {
                                let tag = t.clone();
                                let opening = self.get_opening_tag(&tag)?;
                                let closing = self.get_closing_tag(&tag)?;

                                html.push_str(&opening);
                                html.push_str(&attributes);
                                html.push_str(">");
                                html.push_str(&text);

                                if childs_length > 0 {
                                    let mut i = 0;

                                    loop {
                                        let child_id = &childs[i];
                                        let childs_html = self.generate(child_id)?;
                                        html.push_str(&childs_html);

                                        i += 1;
                                        if i >= childs_length {
                                            break;
                                        }
                                    }
                                }

                                html.push_str(&closing);
                            },
                            None => {
                                if node.root {
                                    if childs_length > 0 {
                                        let mut i = 0;

                                        loop {
                                            let child_id = &childs[i];
                                            let childs_html = self.generate(child_id)?;
                                            html.push_str(&childs_html);

                                            i += 1;
                                            if i >= childs_length {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }

        Ok(html)
    }

    pub fn gen (&mut self, s: &str) -> Result<String, Error> {
        self.parse(s)?;

        self.print_process_time("PARSE");

        match self.hson {
            Some(ref mut hson) => {
                let root_id = hson.get_root();

                self.generate(&root_id)
            },
            None => {
                Ok(String::from(""))
            }
        }
    }

    fn get_opening_tag (&self, tag: &Value) -> Result<String, Error> {
        let tag_name = match tag["tag"].as_str() {
            Some(name) => name,
            None => {
                let e = Error::new(ErrorKind::InvalidData, format!("Invalid key name `tag`"));
                return Err(e);
            }
        };

        match tag_name {
            "doctype" => Ok("<!DOCTYPE".to_string()),
            _ => {
                let mut t = String::from("<");
                t.push_str(&tag_name);

                Ok(t)
            }
        }
    }

    fn get_node_attributes (&mut self, node: &Node) -> Result<String, Error> {
        let mut attributes = String::from("");

        match self.hson {
            Some(ref mut hson) => {
                let mut results = Vec::new();
                let childs_length = node.childs.len();

                if childs_length > 0 {
                    let mut idx = 0;

                    loop {
                        match hson.nodes.get(&node.childs[idx]) {
                            Some(n) => {
                                let k = hson.get_node_key(n);

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
                    let attrs = hson.get_all_childs(&results[0])?;
                    let l = attrs.len();

                    if l > 0 {
                        loop {
                            match hson.get_vertex(&attrs[i]) {
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
            },
            None => {}
        }

        Ok(attributes)
    }

    fn get_node_text (&mut self, node: &Node) -> Result<String, Error> {
        let mut txt = String::from("");

        match self.hson {
            Some(ref mut hson) => {
                let mut results = Vec::new();
                let childs_length = node.childs.len();

                if childs_length > 0 {
                    let mut idx = 0;

                    loop {
                        match hson.nodes.get(&node.childs[idx]) {
                            Some(n) => {
                                let k = hson.get_node_key(n);

                                if k == "text" {
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

                let l = results.len();

                if l > 0 {
                    let mut i = 0;

                    loop {
                        match hson.get_vertex(&results[i]) {
                            Some(vertex) => {
                                let value = match vertex.value_as_string() {
                                    Some(s) => s,
                                    None => {
                                        let e = Error::new(ErrorKind::InvalidData, format!("Cannot retrieve node value"));
                                        return Err(e);
                                    }
                                };

                                txt.push_str(&value);
                            },
                            None => {}
                        }

                        i += 1;
                        if i >= l {
                            break;
                        }
                    }
                }
            },
            None => {}
        }

        Ok(txt)
    }

    fn get_closing_tag(&self, tag: &Value) -> Result<String, Error> {
        let tag_name = match tag["tag"].as_str() {
            Some(name) => name,
            None => {
                let e = Error::new(ErrorKind::InvalidData, format!("Invalid key name `tag`"));
                return Err(e);
            }
        };
        let auto_close = match tag["auto_close"].as_bool() {
            Some(b) => b,
            None => {
            let e = Error::new(ErrorKind::InvalidData, format!("Invalid key name `auto_close`"));
            return Err(e);
            }
        };

        if !auto_close {
            let mut t = String::from("</");
            t.push_str(tag_name);
            t.push_str(">");

            Ok(t)
        } else {
            Ok(String::from(""))
        }
    }

    pub fn print_process_time (&self, tag: &str) {
        let duration = self.process_start.elapsed();
        println!("{} : {:?}", tag, duration);
    }
}

