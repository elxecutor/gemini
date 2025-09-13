#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_client_creation() {
        let client = GeminiClient::new("test_api_key".to_string());
        assert_eq!(client.api_key, "test_api_key");
        assert_eq!(
            client.base_url,
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Hello, world!".to_string(),
                }],
            }],
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Hello, world!"));
        assert!(json.contains("contents"));
        assert!(json.contains("parts"));
        assert!(json.contains("text"));
    }
}