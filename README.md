# hson_gen
Generate HTML from hson.

## Usage
DOM element attributes can be set using the `attrs` key.  
InnerText is set using the `text` key.
```rust
let data = r#"
{
  "doctype": {
    "attrs": {
      "html": ""
    }
  },
  "html": {
    "head": {
      "link": {
        "attrs": {
          "href": "https://fonts.googleapis.com/css?family=Roboto",
          "rel": "stylesheet"
        }
      },
      "title": {
        "text": "Awesome page"
      }
    },
    "body": {
      "div": {
        "attrs": {
          "class": ["main", "visible"]
        },
        "article": {
          "attrs": {
            "class": ["active"]
          },
          "p": {
            "text": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
          },
          "p": {
            "text": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
          }
        },
        "aside": {
          "ul": {
            "attrs": {
              "class": ["right-bar", "active"]
            },
            "li": {
              "a": {
                "attrs": {
                  "href": "https://mozilla.org"
                },
                "text": "Follow ",
                "b": {
                    "text": "the"
                },
                "text": " Money"
              }
            },
            "li": {
              "a": {
                "attrs": {
                  "href": "https://mozilla.org"
                },
                "text": "My link"
              }
            }
          }
        }
      }
    }
  }
}
"#;
  
let mut generator = HsonGen::new();
let html = generator.generate(&data).unwrap();
  
println!("{}", &html);
  
// Generate:  

```