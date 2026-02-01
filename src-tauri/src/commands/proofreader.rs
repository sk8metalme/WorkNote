use crate::models::error::ErrorInfo;
use crate::services::ProofreadService;
use tauri::AppHandle;

#[tauri::command]
pub async fn proofread_markdown(
    _app: AppHandle,
    content: String,
) -> std::result::Result<String, ErrorInfo> {
    let service = ProofreadService::new();
    service.proofread(&content).map_err(ErrorInfo::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_proofreader_command() {
        // Command自体のテストはTauri環境が必要なため、
        // ProofreadService のテストで代替
    }
}
