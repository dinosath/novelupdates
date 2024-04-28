# axum-template

This is a [baker](https://github.com/aliev/baker) template for quickly scaffolding a Rust backend powered by [axum](https://github.com/tokio-rs/axum).

## Table of Contents

- [What does this template do?](#what-does-this-template-do)
- [Getting Started](#getting-started)
- [Baker Questions](#baker-questions)
- [Defining Entities for CRUD Operations](#defining-entities-for-crud-operations)
  - [Entity Schema Structure](#entity-schema-structure)
  - [Property Types](#property-types)
  - [Relationships](#relationships)
  - [Complete Entity Example](#complete-entity-example)
- [Schema Usage in Migrations](#schema-usage-in-migrations)
- [Primary Key / Foreign Key Helper Macros](#primary-key--foreign-key-helper-macros)
- [Docker Multi-Architecture Support](#docker-multi-architecture-support)
- [Why Are Some Structs Red in the IDE?](#why-are-some-structs-red-in-the-ide)
- [Features](#features)
- [License](#license)

## What does this template do?

- **Simple project:** If you just want a basic project structure, you can generate a minimal backend using axum.
- **Full CRUD application:** If you define your entities in the baker prompts, this template will generate a complete CRUD backend with sea-orm models, controllers, and migrations.

## Getting Started

### 1. Install Baker

Follow the [installation instructions for baker](https://github.com/aliev/baker?tab=readme-ov-file#installation).

### 2. Generate a New Project Using This Template

```sh
baker https://github.com/dinosath/axum-template path-to-dir
```

This will prompt you for configuration options and generate a new project in the specified directory.

## Baker Questions

Below are the configurable questions exposed by `baker.yaml`:

- `project_name`: Name of your application (used for crate name and other identifiers)
- `project_author`: Author metadata
- `project_version`: Initial version
- `authentication`: Select auth mechanism (`oidc` or `none`)
- `database`: Currently only `postgres`
- `db_schema`: PostgreSQL schema where all generated tables will be created. Defaults to `public`.
- `id_type`: Type used for primary keys & foreign keys. Choices:
  - `integer` (default): `SERIAL` PK in SQL, Rust type `i32`
  - `big_integer`: `BIGSERIAL` PK, Rust type `i64`
  - `uuid`: `UUID DEFAULT gen_random_uuid()` PK, Rust type `Uuid`
- `features`: Optional extra features (e.g. `open-telemetry`)
- `protocol`: `rest` or `grpc`
- `crudcrate`: Whether to use crudcrate-generated controllers (only asked when protocol is `rest`)
- `entities`: JSON schema describing your domain entities

## Defining Entities for CRUD Operations

The `entities` configuration accepts a JSON object defining your domain model. For each entity, Baker will generate:
- Database models (using sea-orm)
- SQL migrations
- CRUD controllers (if `protocol = rest` and `crudcrate = false`)

### Entity Schema Structure

Each entity is defined using JSON Schema format with the following structure:

```json
{
  "EntityName": {
    "type": "object",
    "title": "EntityName",
    "properties": {
      "field_name": {
        "type": "string|integer|number|boolean",
        "format": "optional format specifier",
        "description": "Field description for documentation"
      }
    },
    "required": ["field_name"]
  }
}
```

### Property Types

The template supports the following property types and formats:

#### String Types
- **Basic string**: `{"type": "string"}`
- **UUID**: `{"type": "string", "format": "uuid"}` → Rust `Uuid`, SQL `UUID`
- **DateTime**: `{"type": "string", "format": "date-time"}` → Rust `DateTime`, SQL `TIMESTAMPTZ`
- **Date**: `{"type": "string", "format": "date"}` → Rust `NaiveDate`, SQL `DATE`
- **Time**: `{"type": "string", "format": "time"}` → Rust `TimeTime`, SQL `TIME`
- **Email**: `{"type": "string", "format": "email"}` → Rust `String` (validated)
- **URL**: `{"type": "string", "format": "url"}` → Rust `String` (validated)

#### Numeric Types
- **Integer**: `{"type": "integer"}` → Maps to `i8`, `i16`, `i32`, `i64` based on min/max constraints
  - With `minimum` and `maximum` constraints for appropriate sizing
  - Example: `{"type": "integer", "minimum": 0, "maximum": 255}` → `i8`
- **Number**: `{"type": "number"}` → Maps to `f32` or `f64` based on constraints
  - Example: `{"type": "number"}` → `f64`

#### Boolean
- **Boolean**: `{"type": "boolean"}` → Rust `bool`, SQL `BOOLEAN`

#### Enums
Define an enum by providing choices:
```json
{
  "status": {
    "enum": ["active", "inactive", "pending"],
    "description": "User status"
  }
}
```

### Relationships

Entities can define relationships using the `x-relationship` extension and `$ref` to reference other entities:

#### Many-to-One (Foreign Key)
```json
{
  "user_id": {
    "x-relationship": "many-to-one",
    "$ref": "User.json",
    "description": "The user who created this"
  }
}
```

#### One-to-Many (Reverse of Many-to-One)
```json
{
  "posts": {
    "x-relationship": "one-to-many",
    "$ref": "Post.json",
    "description": "All posts by this user"
  }
}
```

#### Many-to-Many
```json
{
  "tags": {
    "type": "array",
    "items": {
      "x-relationship": "many-to-many",
      "$ref": "Tag.json"
    },
    "description": "Tags associated with this post"
  }
}
```

Baker will automatically generate junction tables for many-to-many relationships.

### Complete Entity Example

Here's a complete example defining a blog system with Users, Posts, and Tags:

```json
{
  "User": {
    "type": "object",
    "title": "User",
    "properties": {
      "username": {
        "type": "string",
        "description": "Unique username"
      },
      "email": {
        "type": "string",
        "format": "email",
        "description": "User email address"
      },
      "first_name": {
        "type": "string"
      },
      "last_name": {
        "type": "string"
      },
      "is_active": {
        "type": "boolean",
        "description": "Whether the user account is active"
      },
      "created_at": {
        "type": "string",
        "format": "date-time"
      }
    },
    "required": ["username", "email"]
  },
  "Post": {
    "type": "object",
    "title": "Post",
    "properties": {
      "title": {
        "type": "string",
        "description": "Post title"
      },
      "content": {
        "type": "string",
        "description": "Post content"
      },
      "published": {
        "type": "boolean",
        "description": "Whether the post is published"
      },
      "author_id": {
        "x-relationship": "many-to-one",
        "$ref": "User.json",
        "description": "The author of this post"
      },
      "tags": {
        "type": "array",
        "items": {
          "x-relationship": "many-to-many",
          "$ref": "Tag.json"
        },
        "description": "Tags for categorizing the post"
      },
      "created_at": {
        "type": "string",
        "format": "date-time"
      }
    },
    "required": ["title", "content", "author_id"]
  },
  "Tag": {
    "type": "object",
    "title": "Tag",
    "properties": {
      "name": {
        "type": "string",
        "description": "Tag name"
      },
      "slug": {
        "type": "string",
        "description": "URL-friendly tag identifier"
      }
    },
    "required": ["name", "slug"]
  }
}
```

This will generate:
- **Database tables**: `users`, `posts`, `tags`, and `post_tag` (junction table)
- **Rust models**: `User`, `Post`, `Tag` with proper sea-orm annotations
- **CRUD endpoints** (if REST):
  - `POST /api/users`, `GET /api/users`, `GET /api/users/{id}`, `PUT /api/users/{id}`, `DELETE /api/users/{id}`
  - `POST /api/posts`, `GET /api/posts`, `GET /api/posts/{id}`, `PUT /api/posts/{id}`, `DELETE /api/posts/{id}`
  - `POST /api/tags`, `GET /api/tags`, `GET /api/tags/{id}`, `PUT /api/tags/{id}`, `DELETE /api/tags/{id}`
- **Migrations**: SQL scripts with proper foreign key constraints

## Schema Usage in Migrations

The migration template (`migrations/00001_init_db.sql.baker.j2`) uses `db_schema` and `id_type`:

- Creates schema only if not `public`.
- Chooses PK column form based on `id_type`.
- Foreign keys inherit the chosen PK base type.

Example with `db_schema = app` and `id_type = uuid`:

```sql
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE SCHEMA IF NOT EXISTS app;
CREATE TABLE IF NOT EXISTS app.users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMPTZ DEFAULT NOW(),
  last_updated TIMESTAMPTZ DEFAULT NOW()
  -- other columns
);
```

Example with `id_type = big_integer`:
```sql
id BIGSERIAL PRIMARY KEY
```
Example with `id_type = integer`:
```sql
id SERIAL PRIMARY KEY
```

Many‑to‑many join tables and many‑to‑one foreign keys adopt the same underlying type (`UUID`, `BIGINT`, or `INTEGER`).

## Primary Key / Foreign Key Helper Macros

Macros added in `macros.jinja`:
- `rust_id_type(id_type)` → `Uuid | i64 | i32`
- `pk_sql_type(id_type)` → `UUID | BIGINT | INTEGER`
- `pk_column_definition(id_type)` → Full SQL PK declaration.

(If you extend model/controller templates, reuse these macros for consistency.)

## Docker Multi-Architecture Support

The template includes a `Dockerfile` optimized for multi-architecture builds using `cargo-chef`:

### Benefits

- **Multi-architecture support**: Build for `linux/amd64` and `linux/arm64`
- **Fast builds**: `cargo-chef` caches dependencies separately from source code
- **Reproducible builds**: `--locked` flag ensures consistent dependency versions
- **Optimized images**: Multi-stage build with minimal runtime image
- **Compression**: Binary is stripped and uses LTO (Link-Time Optimization) for maximum compression
- **Small image size**: Final multiarch image is approximately 5 MB

### Building Locally

```sh
# Build for your current platform
docker build -t my-app:latest .

# Build for multiple platforms using buildx
docker buildx build --platform linux/amd64,linux/arm64 -t my-app:latest --push .
```

### Dockerfile Stages

1. **Chef stage**: Installs `cargo-chef` for dependency caching
2. **Planner stage**: Analyzes dependencies to create a recipe
3. **Builder stage**: Builds dependencies (cached), then builds your application
4. **Runtime stage**: Minimal Alpine image with just the binary

## Features
- Rust backend powered by axum
- Database support for postgres via sea-orm
- Docker multi-architecture support (amd64, arm64) with cargo-chef
- Compressed, optimized images (~5 MB) using LTO and binary stripping
- OpenAPI documentation with interactive UI (utoipa + Scalar)
- Flexible code generation via baker
- Generate either a minimal project or a full CRUD app based on your entity definitions

## License
MIT