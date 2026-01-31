use serde::{Deserialize, Serialize};

/// カテゴリ種別
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Alerts,
    Maintenance,
    Troubleshooting,
}

impl Category {
    /// 文字列表現を取得
    pub fn as_str(&self) -> &str {
        match self {
            Category::Alerts => "alerts",
            Category::Maintenance => "maintenance",
            Category::Troubleshooting => "troubleshooting",
        }
    }
}

/// 重要度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    /// 文字列表現を取得
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }
}

/// ナレッジ入力データ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeInput {
    pub title: String,
    pub category: Category,
    pub severity: Severity,
    pub symptoms: String,
    pub procedure: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_links: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_as_str() {
        assert_eq!(Category::Alerts.as_str(), "alerts");
        assert_eq!(Category::Maintenance.as_str(), "maintenance");
        assert_eq!(Category::Troubleshooting.as_str(), "troubleshooting");
    }

    #[test]
    fn test_severity_as_str() {
        assert_eq!(Severity::Low.as_str(), "low");
        assert_eq!(Severity::Medium.as_str(), "medium");
        assert_eq!(Severity::High.as_str(), "high");
        assert_eq!(Severity::Critical.as_str(), "critical");
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Low < Severity::Medium);
        assert!(Severity::Medium < Severity::High);
        assert!(Severity::High < Severity::Critical);
    }

    #[test]
    fn test_knowledge_input_serialization() {
        let input = KnowledgeInput {
            title: "Test".to_string(),
            category: Category::Alerts,
            severity: Severity::High,
            symptoms: "CPU high".to_string(),
            procedure: "Check processes".to_string(),
            notes: Some("Notes".to_string()),
            related_links: None,
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"title\":\"Test\""));
        assert!(json.contains("\"category\":\"alerts\""));
        assert!(json.contains("\"severity\":\"high\""));
    }
}
