//! Type definitions for QQ messages and events.

use serde::{Deserialize, Serialize};

/// QQ message type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QqMessageType {
    /// Private message
    Private,
    /// Group message
    Group,
    /// Discuss message
    Discuss,
}

/// QQ message structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqMessage {
    /// Message ID
    pub message_id: i64,

    /// Message type
    pub message_type: QqMessageType,

    /// User ID (sender)
    pub user_id: i64,

    /// Group ID (for group messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,

    /// Message content
    pub message: String,

    /// Raw message (CQ code format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_message: Option<String>,

    /// Sender info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<QqSender>,

    /// Timestamp
    pub time: i64,
}

/// QQ sender information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqSender {
    /// User ID
    pub user_id: i64,

    /// Nickname
    pub nickname: String,

    /// Card name (group card)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    /// Role (for group messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// QQ event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum QqEvent {
    /// Message event
    Message(QqMessage),

    /// Notice event
    Notice {
        notice_type: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },

    /// Request event
    Request {
        request_type: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },

    /// Meta event
    MetaEvent {
        meta_event_type: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

/// QQ API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqApiResponse<T> {
    /// Status code
    pub status: String,

    /// Return code
    pub retcode: i32,

    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,

    /// Warning message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wording: Option<String>,
}

impl<T> QqApiResponse<T> {
    /// Check if the response is successful.
    pub fn is_success(&self) -> bool {
        self.status == "ok" && self.retcode == 0
    }

    /// Get the error message if any.
    pub fn error_message(&self) -> Option<String> {
        if self.is_success() {
            None
        } else {
            Some(
                self.msg
                    .clone()
                    .or_else(|| self.wording.clone())
                    .unwrap_or_else(|| format!("Error code: {}", self.retcode)),
            )
        }
    }
}

/// Send message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// Message type
    pub message_type: QqMessageType,

    /// User ID (for private messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Group ID (for group messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,

    /// Message content
    pub message: String,

    /// Auto escape (disable CQ code parsing)
    #[serde(default)]
    pub auto_escape: bool,
}

/// Send message response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// Message ID
    pub message_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_serialization() {
        let msg_type = QqMessageType::Private;
        let json = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(json, "\"private\"");

        let msg_type = QqMessageType::Group;
        let json = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(json, "\"group\"");
    }

    #[test]
    fn test_api_response_success() {
        let response: QqApiResponse<()> = QqApiResponse {
            status: "ok".to_string(),
            retcode: 0,
            data: None,
            msg: None,
            wording: None,
        };

        assert!(response.is_success());
        assert!(response.error_message().is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: QqApiResponse<()> = QqApiResponse {
            status: "failed".to_string(),
            retcode: 1,
            data: None,
            msg: Some("Error occurred".to_string()),
            wording: None,
        };

        assert!(!response.is_success());
        assert_eq!(response.error_message(), Some("Error occurred".to_string()));
    }
}
