# AkurAI-CRM ISA

## Goal
A pure-Rust, zero-external-dependency CRM built on AkurAI-Framework, reverse-engineered from Twenty CRM.

## Principles
1. Zero runtime dependencies (std + Framework crates only)
2. Client-first: browser does rendering
3. No-build frontend: native ES modules
4. Declarative data model in collections.toml
5. Multi-tenant ready

## Core Entities
- Person (individual contacts)
- Company (organizations)
- Opportunity (deal pipeline)
- Task (to-dos linked to records)
- Note (rich-text notes linked to records)
- TimelineActivity (audit trail)

## API Surface
- GET/POST/PUT/DELETE /api/{entity}
- GET /api/search?q=
- GET /api/timeline?recordId=
- GET /api/_meta
