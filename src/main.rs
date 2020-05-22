use crate::html::Parser;

pub mod dom;
pub mod html;

fn main() {
    let source = r#"
    <html>
        <!-- This is a comment -->
        <body>
            <h1>Title</h1>
            <div id="main" class="test">
                <p>Hello <em>world</em>!</p>
            </div>
        </body>
    </html>"#.to_string();
    let nodes = Parser::parse(source);
    println!("{:#?}", nodes);
}
