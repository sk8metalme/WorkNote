use crate::models::{ErrorInfo, KnowledgeInput};
use crate::services::MarkdownRenderer;

#[tauri::command]
pub fn render_markdown(input: KnowledgeInput) -> std::result::Result<String, ErrorInfo> {
    MarkdownRenderer::render_markdown(&input).map_err(ErrorInfo::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, Severity};

    #[test]
    fn test_render_markdown_command() {
        let input = KnowledgeInput {
            title: "Test".to_string(),
            category: Category::Alerts,
            severity: Severity::High,
            symptoms: "Test symptoms".to_string(),
            procedure: "Test procedure".to_string(),
            notes: None,
            related_links: None,
        };

        let result = render_markdown(input);
        assert!(result.is_ok());
    }
}
