## [Unreleased]

## [0.1.1] - 2026-06-26

### Added

- Project scaffold based on AkurAI-Framework
- CRM core data model: Person, Company, Opportunity, Task, Note
- REST API with full CRUD for all entities, search, timeline, meta
- MCP server with 10 AI-accessible tools
- Activity timeline auto-recorded on create/update/delete
- No-build frontend with design system
- Data-driven pages: list views, detail views, kanban board
- CLI binary with serve/seed commands
- Integration tests: 7 passing
- Open-source release: MIT, public repo
- AkurAI-Framework theme system: vendor themes.css (17 themes), theme.js ESM switcher, flash-guard, theme-picker across all pages
- All CSS color tokens aliased to framework semantic tokens (--bg, --panel, --accent, --fg, --muted, --ok, --warn, --danger, --info, --border-2, etc.)
- Deployed to https://akurai-crm.olibuijr.com (EC2, nginx, systemd, Let's Encrypt)
