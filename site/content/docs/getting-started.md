# Getting Started with AkurAI-CRM

AkurAI-CRM is a pure Rust, zero-external-dependency CRM built on the AkurAI-Framework.

## Quick Start

```bash
# Clone and build
git clone https://github.com/olibuijr/akurai-crm.git
cd akurai-crm
cargo build --release

# Seed demo data
./target/release/akurai-crm seed crm.db

# Start the server
./target/release/akurai-crm serve --port 8091
```

Visit http://127.0.0.1:8091 to access the CRM.

## API

The CRM exposes a REST API at `/api/`:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/_meta` | GET | Discover all object types and fields |
| `/api/people` | GET | List all people |
| `/api/people/{id}` | GET | Get a person |
| `/api/people` | POST | Create a person |
| `/api/people/{id}` | PUT | Update a person |
| `/api/people/{id}` | DELETE | Delete a person |
| `/api/companies` | * | CRUD for companies |
| `/api/opportunities` | * | CRUD for opportunities |
| `/api/tasks` | * | CRUD for tasks |
| `/api/notes` | * | CRUD for notes |
| `/api/search?q=` | GET | Full-text search across all entities |
| `/api/timeline` | GET | Activity timeline for a record |
