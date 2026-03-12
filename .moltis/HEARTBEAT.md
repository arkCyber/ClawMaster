<!--
HEARTBEAT.md is an optional heartbeat prompt source.

Prompt precedence:
1) heartbeat.prompt from config
2) HEARTBEAT.md
3) built-in default prompt

Cost guard:
- If HEARTBEAT.md exists but is empty/comment-only and there is no explicit
  heartbeat.prompt override, Moltis skips heartbeat LLM turns to avoid token use.
-->