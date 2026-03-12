//! Type definitions for WeChat Work messages and events.

use serde::{Deserialize, Serialize};

/// WeChat message type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WeChatMessageType {
    /// Text message
    Text,
    /// Image message
    Image,
    /// Voice message
    Voice,
    /// Video message
    Video,
    /// File message
    File,
    /// Text card message
    TextCard,
    /// News message
    News,
    /// Markdown message
    Markdown,
}

/// WeChat text message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatTextMessage {
    /// Message content
    pub content: String,
}

/// WeChat message structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatMessage {
    /// Recipient user IDs (separated by |)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,

    /// Recipient party IDs (separated by |)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,

    /// Recipient tag IDs (separated by |)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,

    /// Message type
    pub msgtype: WeChatMessageType,

    /// Agent ID
    pub agentid: String,

    /// Text message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<WeChatTextMessage>,

    /// Safe mode (0: can share, 1: cannot share)
    #[serde(default)]
    pub safe: u8,

    /// Enable ID translation
    #[serde(default)]
    pub enable_id_trans: u8,

    /// Enable duplicate check
    #[serde(default)]
    pub enable_duplicate_check: u8,

    /// Duplicate check interval (seconds)
    #[serde(default)]
    pub duplicate_check_interval: u32,
}

/// WeChat API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatApiResponse<T> {
    /// Error code (0 means success)
    pub errcode: i32,

    /// Error message
    pub errmsg: String,

    /// Response data
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T> WeChatApiResponse<T> {
    /// Check if the response is successful.
    pub fn is_success(&self) -> bool {
        self.errcode == 0
    }

    /// Get the error message if any.
    pub fn error_message(&self) -> Option<String> {
        if self.is_success() {
            None
        } else {
            Some(format!("{} (code: {})", self.errmsg, self.errcode))
        }
    }
}

/// Access token response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    /// Access token
    pub access_token: String,

    /// Expires in seconds
    pub expires_in: u64,
}

/// Send message response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// Invalid user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invaliduser: Option<String>,

    /// Invalid party IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidparty: Option<String>,

    /// Invalid tag IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidtag: Option<String>,

    /// Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msgid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_serialization() {
        let msg_type = WeChatMessageType::Text;
        let json = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(json, "\"text\"");

        let msg_type = WeChatMessageType::Image;
        let json = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(json, "\"image\"");
    }

    #[test]
    fn test_api_response_success() {
        let response: WeChatApiResponse<()> = WeChatApiResponse {
            errcode: 0,
            errmsg: "ok".to_string(),
            data: None,
        };

        assert!(response.is_success());
        assert!(response.error_message().is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: WeChatApiResponse<()> = WeChatApiResponse {
            errcode: 40001,
            errmsg: "invalid credential".to_string(),
            data: None,
        };

        assert!(!response.is_success());
        assert_eq!(
            response.error_message(),
            Some("invalid credential (code: 40001)".to_string())
        );
    }

    #[test]
    fn test_text_message_creation() {
        let msg = WeChatMessage {
            touser: Some("user1|user2".to_string()),
            toparty: None,
            totag: None,
            msgtype: WeChatMessageType::Text,
            agentid: "1000002".to_string(),
            text: Some(WeChatTextMessage {
                content: "Hello WeChat!".to_string(),
            }),
            safe: 0,
            enable_id_trans: 0,
            enable_duplicate_check: 0,
            duplicate_check_interval: 0,
        };

        assert_eq!(msg.msgtype, WeChatMessageType::Text);
        assert!(msg.text.is_some());
    }
}
