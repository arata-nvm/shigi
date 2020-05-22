pub mod css;
pub mod dom;
pub mod html;

fn main() {
    let html_source = r#"
    <!DOCTYPE html>
    <html>
        <!-- This is a comment -->
        <body>
            <h1>Title</h1>
            <div id="main" class="test">
                <p>Hello <em>world</em>!</p>
                 <img src="something.png" alt="Something" width="100" height="200" />
            </div>
        </body>
    </html>"#
        .to_string();
    let nodes = html::Parser::parse(html_source);
    println!("{:#?}", nodes);

    let css_source = r#"
    h1, h2, h3 { margin: auto; color: #cc0000; }
    div.note { margin-bottom: 20px; padding: 10px; }
    #answer { display: none; }"#
        .to_string();
    let stylesheet = css::Parser::parse(css_source);
    println!("{:#?}", stylesheet);
}
