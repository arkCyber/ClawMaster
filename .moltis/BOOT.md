<!--
BOOT.md is optional startup context.

How Moltis uses this file:
- Read on every GatewayStart by the built-in boot-md hook.
- Missing/empty/comment-only file = no startup injection.
- Non-empty content = injected as startup user message context.

Recommended usage:
- Keep it short and explicit.
- Use for startup checks/reminders, not onboarding identity setup.
-->