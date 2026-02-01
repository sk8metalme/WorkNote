use crate::models::{KnowledgeInput, Result, WorkNoteError};
use crate::services::FileGenerator;
use pulldown_cmark::{html, Parser};

pub struct MarkdownRenderer;

impl MarkdownRenderer {
    /// KnowledgeInputからHTMLプレビューを生成
    pub fn render_markdown(input: &KnowledgeInput) -> Result<String> {
        // FileGeneratorを使ってMarkdown文字列を生成
        let markdown = FileGenerator::generate_markdown_for_preview(input);

        // Markdownを HTML に変換
        let parser = Parser::new(&markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(html_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, Severity};

    #[test]
    fn test_render_markdown() {
        let input = KnowledgeInput {
            title: "Test Title".to_string(),
            category: Category::Alerts,
            severity: Severity::High,
            symptoms: "Test symptoms".to_string(),
            procedure: "Test procedure".to_string(),
            notes: Some("Test notes".to_string()),
            related_links: Some("https://example.com".to_string()),
        };

        let result = MarkdownRenderer::render_markdown(&input);
        assert!(result.is_ok());

        let html = result.unwrap();
        assert!(html.contains("<h1>Test Title</h1>"));
        assert!(html.contains("Test symptoms"));
        assert!(html.contains("Test procedure"));
    }
}
