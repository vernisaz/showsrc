use simweb::{WebData, WebPage, html_encode};
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

struct Src;
fn main() {
    Src {}.show()
}

impl WebPage for Src {
    fn main_load(&self) -> Result<String, Box<dyn Error>> {
        Ok(include_str! {"../html/show.html"}.to_string())
    }

    fn apply_specific(&self, page_map: &mut HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let web = WebData::default();
        let src = web.param("src").unwrap_or_default();
        let path = PathBuf::from(&src);
        let content = if let Some(ext) = path.extension()
            && env!("ALLOWED_EXTS").contains(&ext.display().to_string())
        {
            fs::read_to_string(&src).unwrap_or_default()
        } else {
            "prohibited".to_string()
        };
        page_map.insert("path", src);
        page_map.insert("theme", web.param("theme").unwrap_or("monokai".to_string()));
        page_map.insert("line", web.param("line").unwrap_or_default());
        page_map.insert("col", web.param("pos").unwrap_or_default());
        page_map.insert("content", html_encode(&content));
        Ok(())
    }
}
