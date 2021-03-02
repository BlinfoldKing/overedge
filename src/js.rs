use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/disqus.js")]
extern "C" {
    pub fn disqus_reset(slug: String);
}

#[wasm_bindgen(module = "/js/markdown.js")]
extern "C" {
    pub fn parse_markdown(x: String) -> String;
}
