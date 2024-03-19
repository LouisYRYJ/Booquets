use lopdf::Document;
use std::error::Error;

pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> bool {
    if case_insensitive {
        return contents.to_lowercase().contains(&query.to_lowercase());
    }
    contents.contains(query)
}

//will be used later
pub fn search_pdf(query: &str, file_path: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let doc = Document::load(file_path)?;

    let pages = doc.get_pages();
    let mut results: Vec<u32> = Vec::new();

    for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = doc.extract_text(&[page_number])?;
        if text.contains(query) {
            results.push(page_number);
        }
    }

    Ok(results)
}
