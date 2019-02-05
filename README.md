# hson_gen
Generate HTML from hson.

## Usage
DOM element attributes can be set using the `attrs` key.  
InnerText is set using the `text` key.
```rust
extern crate hson_gen;
use hson_gen::HsonGen;
  
...
  
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
          "href": "https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css",
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
                  "href": "https://google.com"
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
                "text": "Learn more"
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
```
Generate:
```HTML
<!DOCTYPE html>
<html>
   <head>
      <link href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css" rel="stylesheet">
      <title>Awesome page</title>
   </head>
   <body>
      <div class="main visible">
         <article class="active">
            <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
            <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
         </article>
         <aside>
            <ul class="right-bar active">
               <li><a href="https://google.com">Follow <b>the</b> Money</a></li>
               <li><a href="https://mozilla.org">Learn more</a></li>
            </ul>
         </aside>
      </div>
   </body>
</html>
```