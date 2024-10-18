use markdown_to_html::markdown_to_html;

const EXAMPLE: &str = "# This is a nice converter

 ## It handle *all* titles

## From # to ### with no problems

### With attention to details ;)
###With attention to details ;)

> Blockquotes are handled too
>if well formatted of course

And **bold** and *italics* of course work as well.

****

**bold on
multiple
lines
also**";

fn main() {
    println!("{}", markdown_to_html(EXAMPLE));
}

// $ cargo run
// <h1>This is a nice converter</h1>

//  <h2>It handle <em>all</em> titles</h2>

// <h2>From # to ### with no problems</h2>

// <h3>With attention to details ;)</h3>
// ###With attention to details ;)

// <blockquote>Blockquotes are handled too</blockquote>
// >if well formatted of course

// And <strong>bold</strong> and <em>italics</em> of course work as well.

// <strong></strong>

// <strong>bold on
// multiple
// lines
// also</strong>
// $