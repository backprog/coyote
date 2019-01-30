use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::time::{ Instant };

extern crate hson;
use hson::{ Hson, Node, Kind, Search, Ops, Vertex, Cast, Debug };


pub struct HsonGen {
    mapping: Hson,
    hson: Hson,
    process_start: Instant
}

impl HsonGen {
    pub fn new () -> Result<HsonGen, Error> {
        let bytes = include_bytes!("map.json");
        let json = String::from_utf8_lossy(bytes).into_owned();

        let mut hson_gen = HsonGen {
            mapping: Hson::new(),
            hson: Hson::new(),
            process_start: Instant::now()
        };

        hson_gen.mapping.parse(&json)?;

        Ok(hson_gen)
    }

    pub fn parse (&mut self, s: &str) -> Result<(), Error> {
        self.hson.parse(s)?;

        Ok(())
    }

    pub fn generate (&mut self, s: &str) -> Result<String, Error> {
        self.parse(s)?;

        let root_id = self.hson.get_root();

        self.gen(&root_id)
    }

    fn gen (&mut self, node_id: &str) -> Result<String, Error> {
        let mut html = String::from("");

        match self.hson.nodes.get(node_id) {
            Some(n) => {
                let node = n.clone();
                let childs = node.childs.clone();
                let childs_length = childs.len();
                let uid = node.id.clone();
                let key = self.hson.get_node_key(&node);
                let attributes = self.get_node_attributes(&node)?;
                let text = self.get_node_text(&node)?;

                let results = self.mapping.search(&key)?;
                if results.len() > 0 {
                    let opening = self.get_opening_tag(&results[0])?;
                    let closing = self.get_closing_tag(&results[0])?;

                    html.push_str(&opening);
                    html.push_str(&attributes);
                    html.push_str(">");
                    html.push_str(&text);

                    if childs_length > 0 {
                        let mut i = 0;

                        loop {
                            let child_id = &childs[i];
                            let childs_html = self.gen(child_id)?;
                            html.push_str(&childs_html);

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
                                let child_id = &childs[i];
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

    fn get_opening_tag (&mut self, tag: &String) -> Result<String, Error> {
        let results = self.mapping.search_in(tag, "tag")?;
        let e = Error::new(ErrorKind::InvalidData, format!("Invalid key name `tag`"));

        if results.len() > 0 {
            match self.mapping.get_vertex(&results[0]) {
                Some(vertex) => {
                    let tag_name = match vertex.value_as_string() {
                        Some(name) => name,
                        None => return Err(e)
                    };

                    match tag_name.as_str() {
                        "doctype" => Ok("<!DOCTYPE".to_string()),
                        _ => {
                            let mut t = String::from("<");
                            t.push_str(&tag_name);

                            Ok(t)
                        }
                    }
                },
                None => return Err(e)
            }
        } else {
            return Err(e);
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
            let attrs = self.hson.get_all_childs(&results[0])?;
            let l = attrs.len();

            if l > 0 {
                loop {
                    match self.hson.get_vertex(&attrs[i]) {
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

    fn get_node_text (&mut self, node: &Node) -> Result<String, Error> {
        let mut txt = String::from("");

        let mut results = Vec::new();
        let childs_length = node.childs.len();

        if childs_length > 0 {
            let mut idx = 0;

            loop {
                match self.hson.nodes.get(&node.childs[idx]) {
                    Some(n) => {
                        let k = self.hson.get_node_key(n);

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
                match self.hson.get_vertex(&results[i]) {
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

        Ok(txt)
    }

    fn get_closing_tag(&mut self, tag: &String) -> Result<String, Error> {
        let tag_results = self.mapping.search_in(tag, "tag")?;
        let close_results = self.mapping.search_in(tag, "auto_close")?;
        let e = Error::new(ErrorKind::InvalidData, format!("Invalid key name `tag` or `auto_close`"));

        if tag_results.len() > 0 && close_results.len() > 0 {
            let tag_vertex = match self.mapping.get_vertex(&tag_results[0]) {
                Some(vertex) => vertex,
                None => return Err(e)
            };
            let close_vertex = match self.mapping.get_vertex(&close_results[0]) {
                Some(vertex) => vertex,
                None => return Err(e)
            };
            let tag_name = match tag_vertex.value_as_string() {
                Some(name) => name,
                None => {
                    return Err(e);
                }
            };
            let auto_close = match close_vertex.value_as_bool() {
                Some(close) => close,
                None => {
                    return Err(e);
                }
            };

            if !auto_close {
                let mut t = String::from("</");
                t.push_str(tag_name.as_str());
                t.push_str(">");

                Ok(t)
            } else {
                Ok(String::from(""))
            }
        } else {
            return Err(e);
        }
    }

    pub fn print_process_time (&self, tag: &str) {
        let duration = self.process_start.elapsed();
        println!("{} : {:?}", tag, duration);
    }
}

