# Configuration

For basic configuration instructions, see [this documentation](https://developers.openai.com/codex/config-basic).

For advanced configuration instructions, see [this documentation](https://developers.openai.com/codex/config-advanced).

For a full configuration reference, see [this documentation](https://developers.openai.com/codex/config-reference).

## Subagent service tier

Set `default_subagent_service_tier` under `[agents]` to use a different tier for subagents:

```toml
service_tier = "priority"

[agents]
default_subagent_service_tier = "default"
```

When `[agents].default_subagent_service_tier` is omitted, subagents inherit the parent's tier.
`"default"` selects standard processing even when the parent uses a faster tier.
The same model support checks and feature gates apply as for the parent tier.

## Lifecycle hooks

Admins can set top-level `allow_managed_hooks_only = true` in
`requirements.toml` to ignore user, project, and session hook configs while
still allowing managed hooks from requirements and managed config layers. This
setting is only supported in `requirements.toml`; putting it in `config.toml`
does not enable managed-hooks-only mode.
