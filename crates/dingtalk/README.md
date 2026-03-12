# clawmaster-dingtalk

DingTalk (钉钉) integration for ClawMaster.

## Features
- ✅ DingTalk Open API support
- ✅ Robot webhook support
- ✅ Message sending
- ✅ Interactive cards
- ✅ Access token management

## Configuration
```toml
[channels.dingtalk]
enabled = true
app_key = "your_app_key"
app_secret = "your_app_secret"
webhook_url = "https://oapi.dingtalk.com/robot/send?access_token=xxx"
robot_secret = "your_robot_secret"
```
