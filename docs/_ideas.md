## DX expectations

This document collects legacy and current DX ideas across the Exonware libraries and groups them **per library**.  
Many of these notes predate the `xw*` naming and the fee code; where older names are used, the intent is assumed to map to the modern `xw*` / `XW*` APIs.

**DX maturity color codes**

- **GREEN**: clear DX, solid idea, close to ready.
- **YELLOW**: promising, needs refinement or validation.
- **RED**: risky or probably a bad pattern; keep only as a warning.

For each library you will find:

- **Role**: what the lib is supposed to do.
- **DX goals / examples**: how you want to use it.
- **Idea assessment**: which ideas look solid, which need refinement, and which are likely bad.

---

### 1. Global platform vision (XWBASE)

**DX maturity: 🟡 YELLOW (big vision, needs concrete slices).**

**Status: 🧩 Partially included.** `xwauth`, `xwstorage`, `xwmodels`, `xwquery`, `xwentity`, `xwui` all exist today, but there is **no single `xwbase` facade yet**.  
To get similar behavior now, you compose those libs directly (e.g. auth via `xwauth`, data via `xwstorage`/`xwdata`, UI via `xwui`) and later wrap them in an `xwbase` package.

- **Goal**
  - `GOAL = XWBASE`: extensible base, more flexible than Google’s ecosystem (Firebase, etc.).
  - Starts with: `xwauth`, `xwstorage`, `xwmodels` (entity, collection, group), `xwquery`.
  - High-level example:

```python
entity_example = XWEntity({
    "schema": "...",
    "actions": {...},
    "data": "..",
})

entity_example = XWEntity(
    schema="...",
    actions={...},
    data="..",
)

entity_example = XWEntity(
    schema=XWSchema(...),
    actions={
        "add": XWAction(..),
    },
    data=XWData,
)

def ClassExample():
    pass

entity = XWEntity[ClassExample]()
```

- **Idea assessment**
  - **Makes sense**: unifying around `XWEntity` / `XWGroup` / `XWCollection` as the main mental model, similar to Firestore collections and documents.
  - **Needs refinement**: separate clearly between **schema**, **actions**, and **data backends**; avoid overloading `XWEntity` with too many responsibilities.
  - **Risk / probably bad**: relying too heavily on “magic” generic signatures (`XWEntity[ClassExample]`) without explicit configuration can hurt clarity; these should probably be optional sugar, not the default path.

#### Improved DX version (aligned with current libs)

- **Single `xwbase` facade built from existing libs**: implement a thin `exonware.xwbase` package that re-exports high-level entry points from `xwauth`, `xwstorage`, `xwdata`, `xwentity`, and `xwui`.
- **Entity-first API surface**: standardize around `XWGroup` → `XWCollection` → `XWEntity` navigation, using `xwentity` and `xwdata` under the hood, reusing their schemas/actions instead of redefining them.
- **Minimal generics by default**: keep `XWEntity[ClassExample]` as an *optional* helper, but prefer explicit `schema=...`, `actions=...`, `data=...` parameters in the official examples.

---

### 2. Library: `xwsystem`

**DX maturity: 🟡 YELLOW (strong core, risk of god-module).**

**Status: ✅ Already included and usable.** `xwsystem` is implemented and used as a foundation (config, IO, patterns, etc.).  
Focus work: keep `xwsystem` small and stable, and push higher-level concepts into their own libs so it doesn’t turn into a god-module.

- **Role**
  - Base for almost everything, used by other libs.
  - Contains important submodules: `Config`, `Caching`, `IO`, `Patterns`, `CLI`, and more.
  - `IO` holds codecs, archives, serialization, and many formats.

- **DX goals / examples**

```python
# xwsystem key things:

caching = XWCaching(id="test", strategy="external_lru")
serializer = XWSerlizer(format="json")
archiver = XWAchiver(format="zip")
indexing = XWIndexing(...)

# xwformats is automatically added to serializer and archiver
# xwnode and xwsyntax integrate through IO or related submodules
```

- **Idea assessment**
  - **Makes sense**
    - Having `xwsystem` as the only lib imported by others is a good layering rule if it stays small and stable.
    - Centralizing **config, logging, caching, IO** is strong for consistency across the ecosystem.
  - **Needs refinement**
    - IO boundaries vs `xwformats` / `xwsyntax` need a clear contract: which one owns codecs, which one owns query languages, etc.
    - Naming typos (`Serlizer`, `Achiver`) should be cleaned up for DX and autocomplete.
  - **Probably bad**
    - Letting `xwsystem` become a “god module” that knows about everything; it should provide primitives, not own higher-level business concepts.

#### Improved DX version (aligned with current libs)

- **Lock `xwsystem` to core primitives only**: configuration, logging, IO utilities, process tools. Push anything “domain-y” into `xwdata`, `xwentity`, `xwstorage`, etc.
- **Typed config objects**: promote a standard `XWConfig` pattern (already appearing across libs) that reads from env/files and is re-used by higher-level packages instead of each reinventing config parsing.
- **IO boundary clarity**: document a simple rule—“formats live in `xwformats`, syntaxes in `xwsyntax`, IO pipelines in `xwsystem.io`”—and keep to it across all new work.

```python
# Improved DX sketch using current xwsystem facades

from xwsystem import (
    # Serialization & files
    XWSerializer, quick_serialize, quick_deserialize,
    # Caching / indexing / archiving
    XWCache, XWIndex, XWArchive,
    # Security & HTTP
    quick_hash, quick_encrypt, quick_decrypt, HttpClient,
    # Validation & monitoring
    XModel, Field, performance_monitor,
    # IO & threading & CLI
    XWIO, ThreadSafeFactory, CliConsole, CliProgressBar,
)
from exonware.xwsystem.facade import XWSystem


def improved_dx_example() -> None:
    # Unified system facade (apps can hold this)
    xw = XWSystem.create()

    # Serialization & files
    payload = quick_serialize({"user": 1}, "json")
    user = quick_deserialize(payload, "auto")
    XWSerializer().save_file(user, "data/user.json")

    # Caching
    cache = XWCache.create(strategy="LRU", capacity=1000)
    cache.put("user:1", user)

    # Indexing (JSONL logs)
    idx = XWIndex("logs.jsonl", id_field="id")
    errors_page = idx.get_page(page=0, size=50)

    # Archiving
    XWArchive.create_archive("backup.zip", ["data/", "logs/"])

    # Security & HTTP
    digest = quick_hash("secret", algorithm="sha256")
    encrypted, key = quick_encrypt("top-secret")
    resp = HttpClient().get("https://api.example.com/data")

    # Validation
    class UserModel(XModel):
        id: int = Field(...)

    validated = UserModel(**{"id": 1})

    # Monitoring
    with performance_monitor("load_user"):
        _ = validated.id

    # IO & CLI UX
    data = XWIO().read_file("data/user.json")
    CliConsole().print(f"User: {data}")
    bar = CliProgressBar(total=100)
    bar.update(10)
```

---

### 3. Library: `xwformats`

**DX maturity: 🟢 GREEN (responsibilities are clear).**

**Status: ✅ Already included and usable.** `xwformats` is implemented with many concrete formats and is already wired into other libs (e.g. `xwdata`).  
You can rely on it now for serialization/archiving, and add new formats via the existing extension points.

- **Role**
  - Format adapters: binary, database, scientific, text, schema formats, etc.
  - Intended to be plugged automatically into serializer/archiver and into IO-heavy libs like `xwdata`.

- **DX goals / examples**
  - `xwformats` is automatically added to `xwserlizer` and `xwarchiver`.
  - Coexists with `xwsyntax` so the same formats can be used either as:
    - **formats** (load/save, archiving, serialization), or
    - **syntaxes** (query languages).

- **Idea assessment**
  - **Makes sense**
    - Separating *storage formats* from *query syntaxes* is solid; this also fits with how you’ve structured tests and subpackages already.
  - **Needs refinement**
    - Decide whether `xwformats` depends on `xwsystem.io` only, or also on `xwnode` / `xwdata` abstractions.
  - **Probably bad**
    - Letting `xwformats` also start doing query-like work (that belongs in `xwsyntax` / `xwquery`) will blur responsibilities.

#### Improved DX version (aligned with current libs)

- **Explicit “format adapters” API**: surface a small `register_format(name, loader, dumper)` API that `xwdata` and `xwstorage` can depend on, instead of each one special-casing formats.
- **Tight coupling to IO, loose coupling to data models**: keep `xwformats` depending on `xwsystem.io` only, with adapters that know nothing about `XWNode` or `XWEntity`, to reduce coupling.
- **Docs that map formats to real usage**: reuse existing `xwdata` examples (JSON, Parquet, etc.) to show “if you install `xwformats[x]`, here’s exactly how `XWData` uses it”.

---

### 4. Library: `xwnode`

**DX maturity: 🟡 YELLOW (core is solid, imports need simplification).**

**Status: ✅ Already included and usable.** `xwnode` exists as a standalone lib with strategies, caching, and graph features.  
Best practice today is to import via the canonical `exonware.xwnode` path and treat other aliases as optional sugar (or deprecate them over time).

- **Role**
  - Core node/graph abstraction: node building strategies, edge strategies, graph management, caching, references, lazy loading.

- **DX goals / examples**

```python
node = XWNode(node_strategy="Tree", edge_strategy="Bidrectional")

node["id"]
node.query("x")

# Generic flavor:
node = XWNode[NodeStrategy, EdgeStrategy]()
```

- **Import aliases (DX ideas)**

```python
import exonware.xwnode       # alias
import xw.node               # alias
import xwnode                # alias
import exonware.node         # alias
```

- **Idea assessment**
  - **Makes sense**
    - Having `XWNode` as the core data structure that `xwdata` and others can extend or compose with is a strong idea.
    - Strategy-based configuration (`node_strategy`, `edge_strategy`) is powerful for flexibility.
  - **Needs refinement**
    - Decide a **single canonical import path** (e.g. `from exonware import xwnode as node`) and only expose a couple of *re-exported* aliases; too many aliases hurt tooling and docs.
  - **Probably bad**
    - Supporting 3–4 full alias paths as “official” will confuse users and make docs harder; most ecosystems pick one obvious path and maybe one short alias.

#### Improved DX version (aligned with current libs)

- **One official import path**: standardize on `from exonware.xwnode import XWNode` and offer at most *one* short alias (e.g. `from exonware import xwnode as node`).
- **Preset strategies**: expose ready-made strategies like `XWNode.tree()`, `XWNode.graph()` that internally configure the right `node_strategy`/`edge_strategy` objects already used in the codebase.
- **Interop with `xwdata` / `xwquery`**: document how `XWData` and `XWQuery` already wrap `XWNode` so users don’t have to manually wire nodes unless they’re doing advanced graph work.

---

### 5. Library: `xwsyntax`

**DX maturity: 🟡 YELLOW (clean separation, needs a single abstraction).**

**Status: ✅ Already included and usable.** `xwsyntax` exists and is integrated into the ecosystem, especially around query and parsing use cases.  
Short term, use the existing syntax abstractions as-is; medium term, unify them behind one clear `XWSyntax` interface.

- **Role**
  - Parallel to IO but focused on **syntaxes** instead of storage formats.
  - Supports *the same formats* but with a different engine: query/expression languages, DSLs, etc.

- **DX goals / examples**

```python
# To add a new syntax:
syntax_html = XWSyntax(input="path", output="path")

# xwsyntax is added automatically to IO-related submodules,
# similar to how xwformats integrates.
```

- **Idea assessment**
  - **Makes sense**
    - Splitting “how data is stored” (`xwformats`) from “how data is expressed/queried” (`xwsyntax`) is clean and future-proof.
  - **Needs refinement**
    - You should define **one clear abstraction** for a syntax (parse, validate, maybe compile), and keep it independent from the physical storage backend.
  - **Probably bad**
    - Refactoring too many concerns into `xwsyntax` (e.g. doing heavy IO work) would duplicate `xwformats` responsibilities.

#### Improved DX version (aligned with current libs)

- **Single `XWSyntax` contract**: formalize the core interface used in `xwsyntax` (e.g. `parse`, `to_ast`, `to_backend_query`) and adopt it consistently across all syntaxes.
- **Backend-agnostic AST**: keep syntax objects focused on producing a neutral AST that `xwquery` or specific backends can then compile, rather than letting syntaxes talk to storage directly.
- **Pluggable syntax registry**: mirror how strategies are handled today and allow `xwquery`/`xwdata` to look up syntaxes by name, making it easy to add new ones without touching core code.

---

### 6. Library: `xwquery`

**DX maturity: 🟡 YELLOW (good model, backend plug-ins not designed yet).**

**Status: ✅ Already included and usable.** `xwquery` is implemented with multiple strategies and can already run real queries (see its examples/tests).  
To get “xwbase-style” queries now, plug `xwquery` into `xwdata`/`xwnode` and standardize adapters per backend (SQL, Graph, etc.).

- **Role**
  - Query engine using `xwsyntax` to talk to various formats and backends.

- **DX goals / examples**

```python
from exonware.xwquery import XWQuery

query = XWQuery[XWNode](format="xwqs", query="select * from table", context=obj)
nodes = query.execute()
```

- **Idea assessment**
  - **Makes sense**
    - Using generics (`XWQuery[XWNode]`) to indicate the shape of data is a nice type-level hint, especially for editors.
    - Separating **query description** from **execution context** (`context=obj`) is a good design.
  - **Needs refinement**
    - Figure out a standard way to plug in backends (e.g. adapters for SQL, GraphQL, PromQL, etc.) so the user does not need to think about registry details.
  - **Probably bad**
    - Supporting too many query syntaxes in one monolithic package without modularization; better to have pluggable strategies per backend.

#### Improved DX version (aligned with current libs)

- **Backend adapters as first-class objects**: define a simple `XWQueryBackend` interface and expose official adapters for the backends you already use (SQL, JSON, logs, etc.).
- **Query builder sugar over syntax strings**: on top of `query="select * from table"`, offer small builder helpers (e.g. `XWQuery.sql().from_("table").select("*")`) that compile to the same underlying `xwsyntax`.
- **Tight pairing with `xwdata`**: document the recommended flow as `XWData(...).query(...)` delegating to `xwquery` internally, rather than users instantiating `XWQuery` in isolation.

---

### 7. Library: `xwdata`

**DX maturity: 🟡 YELLOW (great ergonomics, performance semantics need tightening).**

**Status: ✅ Already included and usable.** `xwdata` is a full library with rich docs, benchmarks, and examples; it already supports subscript access, lazy strategies, and integration with formats.  
If you want the “ultimate DX” version, build on top of the current `XWData` APIs and tighten the lazy/materialization guarantees rather than starting from scratch.

- **Role**
  - Data access abstraction built on `xwnode`, `xwformats`, and `xwquery`.
  - Handles loading/saving, references, lazy loading, and query execution.

- **DX goals / examples**

```python
from exonware.xwdata import XWData

data_json = XWData[JSON](path="file path", config={"lazy": True, "atomic": True})

first = data_json[0]
id_ = data_json["id"]
result = data_json.query("query here")
```

- **Idea assessment**
  - **Makes sense**
    - Treating data containers as indexable (`__getitem__`) and queryable is very intuitive for Python developers.
  - **Needs refinement**
    - Be explicit about when data is **materialized** vs **lazy**; “atomic” semantics should be clearly documented.
  - **Probably bad**
    - If `XWData` tries to hide all backend details (e.g. loading gigabytes into memory by default), DX will look nice but performance will be dangerous; you’ll want clear knobs.

#### Improved DX version (aligned with current libs)

- **Profiles that match real implementations**: define named profiles like `profile="in_memory" | "lazy_indexed" | "streaming"` that map directly onto strategies and caches already implemented in `xwdata`.
- **Explicit materialization methods**: standardize on methods like `.materialize()`, `.to_pandas()`, `.to_list()` so it’s always clear when full data is being loaded.
- **Config objects instead of giant dicts**: promote small `XWDataConfig` dataclasses (or similar) mirroring what `xwdata` already uses internally, instead of opaque `config={...}` dicts.

---

### 8. Library: `xwjson`

**DX maturity: 🟡 YELLOW (useful helper, should stay thin over xwdata).**

**Status: ✅ Already included and usable.** `xwjson` exists as its own lib and is also deeply integrated into `xwdata` examples (e.g. chatdb).  
To get the intended DX, use `xwjson` as a friendly entry point that internally leverages `xwdata` and `xwformats(json)`.

- **Role**
  - JSON-focused storage/query helper.

- **DX goals / examples**

```python
from exonware.xwjson import XWJSON

json_db = XWJSON(path="file path")
json_db.query("select *")
```

- **Idea assessment**
  - **Makes sense**
    - A simple JSON-backed store is ideal for quick prototyping and local dev.
  - **Needs refinement**
    - Should probably be just a thin adapter over `xwdata` + `xwformats(json)` to avoid another custom stack.
  - **Probably bad**
    - Re-implementing a full query engine unique to JSON here instead of re-using `xwquery` / `xwsyntax`.

#### Improved DX version (aligned with current libs)

- **Make `xwjson` a thin facade over `xwdata`**: position `XWJSON` as `XWData` preconfigured with JSON format, lazy defaults, and sensible file layout (as is already done in examples like `chatdb`).
- **Opinionated starter API**: provide tiny helpers like `XWJSON.open(path)` or `XWJSON.db(path)` that hide away configuration while still returning an `XWData`-compatible object.
- **Unified query story**: route `json_db.query(...)` calls directly through `xwquery` so users are learning one query language across formats.

---

### 9. Library: `xwschema`

**DX maturity: 🟢 GREEN (concept is standard and clear).**

**Status: 🧩 Partially included.** Schema concepts are present today via `xwmodels`, JSON schemas in examples, and validation patterns, but there is no single, central `xwschema` package with a stable API.  
In practice, you can get similar behavior by pairing `xwdata` with existing JSON schema/pydantic/dataclasses and gradually extracting that into a dedicated `xwschema` layer.

- **Role**
  - Validates that `xwdata` instances conform to schema.

- **DX goals / examples**

```python
schema = XWSchema(...)
data = XWData(...)

schema.validate(data)
```

- **Idea assessment**
  - **Makes sense**
    - Having schema separate from data and actions is critical for correctness and introspection.
  - **Needs refinement**
    - Decide whether schemas are runtime-only, type-level, or both (e.g. support for pydantic/dataclasses interop).
  - **Probably bad**
    - Embedding too much execution logic in schema objects; they should describe, not execute.

#### Improved DX version (aligned with current libs)

- **Central `xwschema` package built from existing patterns**: extract common schema concepts from `xwmodels`, JSON schemas, and validators into a single `exonware.xwschema` module.
- **Runtime + typing integration**: document how `XWSchema` can be generated from or mapped onto pydantic/dataclasses, mirroring how current libs already validate data.
- **Schema as metadata only**: ensure `XWSchema` objects focus on describing constraints and generating docs, leaving all execution and side effects to `xwaction`/`xwentity`.

---

### 10. Library: `xwaction`

**DX maturity: 🟡 YELLOW (powerful, but needs standard action contracts).**

**Status: ✅ Already included and usable.** `xwaction` exists as a concrete lib; actions are used across the stack today (especially with entities and APIs).  
To achieve the ideal DX, define and document a single action signature pattern (inputs/outputs/context) and align all existing actions to it.

- **Role**
  - Runs commands using `xwschema`, `xwdata`, and `xwquery`.

- **DX goals / examples**

```python
from exonware.xwaction import XWAction

action = XWAction(...)
action.run(data=XWData(...), schema=XWSchema(...))
```

- **Idea assessment**
  - **Makes sense**
    - Having multi-tenant, schema-aware actions is powerful, especially when paired with auth.
  - **Needs refinement**
    - Standardize action signatures (inputs/outputs/context) so they can be composed and executed by bots and flows.
  - **Probably bad**
    - Letting `XWAction` directly manage IO and storage; better to coordinate with `xwdata` / `xwstorage`.

#### Improved DX version (aligned with current libs)

- **One canonical `XWAction` signature**: align all current actions around something like `run(self, data: XWData, ctx: XWContext) -> XWResult`, mirroring how you already call actions in services.
- **Composable actions library**: promote small reusable actions (filter, map, validate, persist) that can be chained together, matching how flows and bots want to compose logic.
- **Separation from storage**: document that actions *call into* `xwdata` / `xwstorage` and never talk to files/DBs directly, so testing and reuse stay easy.

---

### 11. Library: `xwentity`

**DX maturity: 🟡 YELLOW (very strong pattern, needs clear magic vs config).**

**Status: ✅ Already included and usable.** `xwentity` is implemented and actively used; it already combines schema, actions, and data in a powerful way.  
Short term, you can build entities on top of the current base class; long term, refine how much meta-programming it does vs explicit configuration.

- **Role**
  - Combines the power of `xwschema`, `xwaction`, and `xwdata`.
  - Serves as the main building block for domain objects like users, posts, etc.

- **DX goals / examples**

```python
class XWUser(XWEntity):
    # Properties via XWSchema:
    # UID, ID, Name, EMail, ...
    # Actions via XWAction: Disable, Enable, ...
    # Inherited stuff: Create/Add, View, Save, Load, Search, List, ...
    pass


class XWEntity(IEntity):
    # _properties returns list[dict] with XWSchema
    # _actions returns list[dict] with XWAction
    # _schema = _properties + _actions
    # _data returns XWData with keys and values/XWData
    ...
```

- **Idea assessment**
  - **Makes sense**
    - Defining entities declaratively and letting the framework provide CRUD, list, search, etc. is a very strong DX pattern.
  - **Needs refinement**
    - Carefully define how much “magic” the base `XWEntity` does (e.g. meta-class parsing of fields) vs explicit config.
  - **Probably bad**
    - If entities become god-objects that know about storage, auth, caching, and UI simultaneously, separation of concerns will suffer.

#### Improved DX version (aligned with current libs)

- **Slim `XWEntity` core + mixins**: keep the base `XWEntity` focused on schema/data wiring and compose in extra capabilities (auth, caching, UI hints) via mixins, similar to patterns already used in the codebase.
- **Declarative field definitions using current tools**: lean on the style used in `xwmodels`/`xwentity` today (field descriptors, metadata) and document it as the canonical way to define properties.
- **Generated docs and admin UIs via `xwui`**: tie `XWEntity` metadata into `xwui` to auto-generate list/detail forms, reflecting what’s already possible with your component set.

---

### 12. Library: `xwauth`

**DX maturity: 🟡 YELLOW (central service, details of providers still open).**

**Status: ✅ Already included and usable.** `xwauth` is a real, substantial library with APIs, docs, and integration points.  
You can use it today for scoped tokens and auth flows; work needed is mostly on polishing the provider patterns and client DX.

- **Role**
  - State-of-the-art auth lib and web app.
  - Provides username/password logins and returns scoped tokens.

- **DX goals / examples**

```python
# Service base URL
#   https://api.xwauth.dev/api/v1/

from exonware.xwapi import XWAuthProvider

xwauth_agent = XWAuthAgent(provider="xwauth", api_token=env.xwauth_key)

# Example of username/password flow:
# POST /api/v1/... with username/password → returns API token with scopes
```

- **Idea assessment**
  - **Makes sense**
    - Central auth service for the whole ecosystem is the right call.
  - **Needs refinement**
    - Make sure the provider pattern (`XWAuthAgent(provider="xwauth", ...)`) is consistent with other services.
  - **Probably bad**
    - Hard-coding domain names deeply into client libs; better to allow environment-based configuration.

#### Improved DX version (aligned with current libs)

- **Unified auth client**: provide a single `XWAuthClient` that wraps what `XWAuthAgent` and other helpers do today, with clear methods for login, token refresh, and scope checks.
- **Environment-driven endpoints**: adopt the pattern already used in other libs—read base URLs from config/env—so the same client works across `dev/stage/prod` without code changes.
- **Tight integration with `xwentity` and `xwbase`**: surface helpers like `current_user()` or `with_roles(...)` that hook directly into the entity/group model, matching how existing services think about users.

---

### 13. Library: `xwstorage`

**DX maturity: 🟡 YELLOW (great config model, needs connector story).**

**Status: 🧩 Partially included.** There is an `xwstorage` package (and integration files like `xwdata_integration.py`), but the “full vision” of every backend/connector isn’t complete yet.  
Right now, you can connect to specific backends that already have adapters; to reach the final DX, standardize the connector interface and add more backends behind the same config model.

- **Role**
  - Base data graph/db engine and connector library.
  - Handles connections to various storage backends (local, cloud, DBs).

- **DX goals / examples**

```python
xwstorage_agent = XWStorageAgent(provider="xwstorage", api_token=env.xwstorage_key)

google_sheet_connection = exonware.xwstorage.connection({ "type": "google_sheets", ... })
ms_excel_connection = exonware.xwstorage.connection({ "type": "excel", ... })
xwjson_local_connection = exonware.xwstorage.connection({ "type": "xwjson_local", ... })
csv_local_connection = exonware.xwstorage.connection({ "type": "csv_local", ... })
ftp_folder_connection = exonware.xwstorage.connection({ "type": "ftp", ... })
google_folder_connection = exonware.xwstorage.connection({ "type": "google_drive", ... })
```

- **Config model**

```python
# Two dimensions:
#   format_type vs location_type

config.location  # local folder, ftp, onedrive, gdrive, oracle, MySQL, AWS, Azure, ...
config.format    # Tabular, Node, Graph, JSON-like, ...
config.auth      # XWAuth object or other provider
```

- **Idea assessment**
  - **Makes sense**
    - Splitting **location**, **format**, and **auth** into separate config axes is excellent; this will scale well.
  - **Needs refinement**
    - Decide how much of this belongs to `xwstorage` vs `xwdata`; avoid duplicating configuration structures.
  - **Probably bad**
    - Trying to encode every possible backend as built-in types; better to support pluggable connectors.

#### Improved DX version (aligned with current libs)

- **Shared config model with `xwdata`**: align `XWStorageConfig` with how `xwdata` already describes formats and locations so users see one mental model across both.
- **Connector registry**: formalize the adapter pattern already hinted at (e.g. Google Sheets, CSV, FTP) into a `register_connector()` API and list official connectors in docs.
- **Graceful degradation**: document a clear fallback path where, if a connector is missing, users can still mount that backend via `xwdata` + custom IO code.

---

### 14. Library: `xwwebapp`

**DX maturity: 🟡 YELLOW (useful layer, UI/routing conventions pending).**

**Status: 🧩 Partially included (via other services).** There isn’t a single `xwwebapp` repo, but `xwauth`, `xwapi`, and `xwui` already implement pieces of the webapp story (FastAPI servers, admin-style UIs, etc.).  
To approximate `xwwebapp` today, combine those pieces: build FastAPI-based services using `xwapi`/`xwauth` and front them with `xwui` components.

- **Role**
  - Enables `xwauth`, `xwstorage`, and `xwbase` to be web apps.

- **DX goals / examples**
  - Likely uses standard web frameworks (FastAPI, etc.).

- **Idea assessment**
  - **Makes sense**
    - Having a unified webapp layer for admin consoles, dashboards, and dev tools is useful.
  - **Needs refinement**
    - You should standardize routing and component conventions to avoid each service inventing its own UI.

#### Improved DX version (aligned with current libs)

- **Standard FastAPI + `xwui` template**: encode what you already do in `xwauth`/`xwapi` into a small “xw webapp starter” that wires FastAPI routes to `xwui` components.
- **Auth + storage middleware**: reuse the middlewares present today in your services (auth checks, connection injection) and package them as plug-and-play pieces.
- **Single admin shell**: define a shared admin layout (using `xwui`) that can mount feature modules from `xwauth`, `xwstorage`, `xwbase`, etc., instead of each building its own shell.

---

### 15. Library: `xwui`

**DX maturity: 🟡 YELLOW (strong idea, needs framework decision).**

**Status: ✅ Already included and usable.** `xwui` is a large, actively developed UI library with many components and framework examples.  
You can use it right now in multiple frontends (React/Vue/etc. via the examples); remaining work is mostly about tightening the primary integration story and design system.

- **Role**
  - UI library using TypeScript with many components.
  - Used by `xwbase` and potentially `xwwebapp`.

- **DX goals / examples**
  - Component library for auth screens, data browsers, flow builders, bot UIs, etc.

- **Idea assessment**
  - **Makes sense**
    - Centralized cross-product UI library is great for consistency and faster dev.
  - **Needs refinement**
    - Decide whether `xwui` is React-only or framework-agnostic.

#### Improved DX version (aligned with current libs)

- **Web components as the stable core**: lean into the cross-framework stories you already have (React/Vue/Svelte examples) by making web components the primary delivery format.
- **Official bindings per ecosystem**: build and maintain thin wrappers for React/Vue/etc. that simply wrap the web components, following patterns already sketched in the examples.
- **Design tokens + theming**: promote the token system already present in `xwui` as the main customization surface (light/dark, brand colors) instead of per-component overrides.

---

### 16. Library: `xwbase`

**DX maturity: 🟡 YELLOW (high impact, should be sliced and validated).**

**Status: ❌ Not included as a single package yet.** There is no top-level `xwbase` repo; instead, the platform is spread across `xwauth`, `xwstorage`, `xwdata`, `xwentity`, `xwui`, etc.  
To get similar outcomes today, build on those individual libs and, if needed, create a small internal “xwbase” wrapper package that re-exports the most common flows.

- **Role**
  - Firebase/Supabase-like platform built using `xwauth`, `xwstorage`, `xwui`.
  - Three parts: `xwbase` lib, `xwbase` API, `xwbase` UI.

- **DX goals / examples**

```python
exonware.xwbase.groups["group_name"].collections["posts"]
exonware.xwbase.objects["UID"]
exonware.xwbase.users["user_id"]
exonware.xwbase.roles["role_id"]

Exonware.xwbase.add_group(...)
```

- **Group model**

```python
group = XWGroup({ "info": ... })   # Group, using auth, knows user roles

sub_groups = group.sub_groups                      # list[XWGroup]
group_collections = group.collections              # list[XWCollection]

entity_collection = group.collections["Collection Name"]  # XWCollection
entity = group.collections["Collection Name"].entities["entity_id"]  # XWEntity

entity["theme"] = "dark"
entity.update()  # triggers collection update

entity_collection.entity_update("entity_id", {"theme": "dark"})
group.collection_entity_update("collection_id", "entity_id", {"theme": "dark"})
```

- **External integrations**

```python
exonware.xwbase.x.firebase  # all Firebase APIs
exonware.xwbase.x.supabase  # all Supabase APIs
```

- **API URLs**

```text
https://api.xwbase.dev/    # main xwbase API
https://api.xwauth.dev/    # xwauth API
https://api.xwstorage.dev/ # xwstorage API
```

- **Idea assessment**
  - **Makes sense**
    - Exposing a consistent traversal API (`groups → collections → entities`) mirrors Firebase-style DX and is very discoverable.
  - **Needs refinement**
    - Clarify the relationship between `xwbase` and lower-level libs (direct usage vs via adapters).
  - **Probably bad**
    - Overloading the `x` namespace for external APIs (`x.firebase`, `x.supabase`) might be confusing; consider a clearer plugin registry name.

#### Improved DX version (aligned with current libs)

- **Lightweight `xwbase` wrapper package**: create a minimal `exonware.xwbase` that simply re-exports orchestrated flows using existing libs (`xwauth`, `xwstorage`, `xwdata`, `xwentity`, `xwui`).
- **Clear plugin namespace**: instead of `xwbase.x.firebase`, define an `xwbase.providers` or `xwbase.integrations` namespace that can include Firebase, Supabase, etc.
- **Starter blueprints**: ship a couple of ready-made blueprints (auth + posts, auth + analytics) that match realistic combos you already have in examples and tests.

---

### 17. Library: `xwai`

**DX maturity: 🟡 YELLOW (good abstraction, needs model/tooling contracts).**

**Status: ❌ Not included as a standalone service yet.** There is no dedicated `xwai` repo; AI behavior currently lives across tools, bots, and external model APIs.  
To approximate `xwai` today, you integrate existing AI providers directly in your services/bots and later wrap those integrations behind a shared `XWAIAgent`-like facade.

- **Role**
  - AI layer using `xwbase` server, acting as an AI broker to send/receive prompts.

- **DX goals / examples**

```python
xwai_agent = XWAIAgent(provider="xwai", api_token=env.xwai_key)
```

- **Idea assessment**
  - **Makes sense**
    - Having a shared AI gateway for bots, flows, and apps is useful.
  - **Needs refinement**
    - Decide how models, prompts, and tools are configured; align with standard patterns like OpenAI-style clients.

#### Improved DX version (aligned with current libs)

- **Thin AI client over existing providers**: define `XWAIAgent` as a simple facade over the model APIs you already use (OpenAI, local models, etc.), with a consistent call signature.
- **Tooling + prompt presets**: expose helpers for common flows already used in your projects (chat completion, embedding, RAG) so products like `manam.ai` can plug them in quickly.
- **Tight link to `xwflow` and `xwbots`**: design `XWAIAgent` so it can be easily called by flow steps and bot commands without extra glue code.

---

### 18. Library: `xwflow`

**DX maturity: 🟡 YELLOW (direction is good, graph semantics open).**

**Status: ❌ Not included yet.** There is no `xwflow` package at the root; flow/orchestration logic is spread across other tools and ad-hoc glue.  
To get similar results now, you can orchestrate flows using `xwbots`-like patterns plus external tools (e.g. n8n, Airflow) and eventually migrate them into a dedicated `xwflow` lib.

- **Role**
  - n8n-like system for flows: library, API, and UI.

- **Idea assessment**
  - **Makes sense**
    - Leveraging `xwbase`, `xwentity`, and `xwaction` to define flows is a strong direction.
  - **Needs refinement**
    - Nail down the graph model for steps and error handling; consider re-using `xwnode` strategies.

#### Improved DX version (aligned with current libs)

- **Flow as `XWNode` graphs**: base the flow graph model directly on `xwnode` strategies, so each step is a node and edges reflect transitions, retries, and error paths.
- **Step library from existing actions**: seed `xwflow` with steps that wrap the `xwaction` primitives you already have (load data, run query, send notification, call AI).
- **First-class dev tooling**: integrate with `xwui` to offer a visual editor and inspector that reads/writes the same flow definitions used in code.

---

### 19. Library: `xwbots`

**DX maturity: 🟡 YELLOW (rich concept, composition vs inheritance needs work).**

**Status: ❌ Not included as a top-level lib yet.** There is no `xwbots` package in the repo; bot behavior is implemented via other libraries and services.  
For now, you can build bots using `xwapi`, `xwauth`, and AI integrations directly; later, consolidate the patterns into a dedicated `xwbots` framework.

- **Role**
  - Bot framework with different bot types:
    - `XWBotCommand`
    - `XWBotPersona`
    - `XWBotAgentic`
  - Goal: **bot creator** (UI, WebApp, Backend).

- **DX goals / examples**

```python
karizma_bot_config_path = "url"

karizma_bot_config = XWData(path=karizma_bot_config_path)
karizma_bot = XWBotCommand(config=karizma_bot_config)
karizma_bot.start()
```

- **Core classes**

```python
XWBotCommand = XWBot + XWChatAgent


class XWApiEntity(AApiEntity):
    # init: stop all commands except: start/stop/restart/pause/resume/kill
    # start: load config and start listening to all commands
    # stop: stop listening to commands; only way forward is start
    # restart: run stop, then start
    # pause: stop executing but keep config
    # resume: start executing commands again
    # health: give status
    # kill: stop the agent completely
    ...


class XWApiAgent(AApiAgent, XWApiEntity):
    # Used to get things from other APIs
    ...


class XWApiServer(AApiServer, XWApiEntity):
    # Used to serve actions: can be Web API or other ways…
    ...


class XWBot(ABot, XWApi):
    # Specify how the bot will run: Chat Agents or Server.
    # Should be available on multiple chats:
    # internal process, native XWChatServer, TelegramBot, TelegramUser, WhatsApp, ...
    agent = XWChatAgent()  # make the server available on chat


class XWBotCommand(ABotCommand, XWBot):
    # Built-in commands
    commands_builtin: list[XWCommand]
    # API commands
    commands_api: list[XWCommand]
    # All commands
    commands = commands_builtin + commands_api
    # Trigger/Event/DateTime based tasks using commands


class XWBotAI(ABotAI, XWBot):
    llm_engine = XWAILLM()  # give it a token and make it work
    # LLM engines, list / knowledge base


class XWBotPersona(ABotPersona, XWBotAI):
    # MD file for persona
    ...


class XWBotAgent(ABotAgent, XWBotCommand, XWBotPersona):
    # Take a chat, convert it to commands and arguments,
    # ask for more info until full command, then execute.
    # Can execute more than one command.
    ...
```

- **Idea assessment**
  - **Makes sense**
    - Splitting bots into **command**, **persona**, and **agentic** layers maps well to how modern AI bots are built.
    - Having a unified server/bot abstraction (chat and HTTP) is very compelling.
  - **Needs refinement**
    - Avoid deep multi-inheritance (`XWBotAgent(ABotAgent, XWBotCommand, XWBotPersona)`); consider composition or mixins for clarity.
  - **Probably bad**
    - Starting/stopping bots purely from class-level config (without explicit lifecycle objects) might be hard to manage in production.

#### Improved DX version (aligned with current libs)

- **Composition over inheritance**: model bots as small objects that *contain* a transport (HTTP/chat), a persona, and a command router, instead of inheriting from multiple bases.
- **Reuse `xwapi`/`xwauth` patterns**: have bot backends authenticate and talk to APIs using the same client patterns you already use elsewhere (agents/providers).
- **Standard lifecycle hooks**: define explicit `start()`, `pause()`, `resume()`, `kill()` hooks that map onto the ideas already sketched for `XWApiEntity`, and surface them consistently in code and docs.

---

### 20. API servers (e.g. `xwauth` / others)

**DX maturity: 🟡 YELLOW (decorator model is good, base classes need shaping).**

**Status: 🧩 Partially included.** `xwapi` and `xwauth` already implement real API servers, often using FastAPI and decorators for actions.  
To fully realize this idea, standardize the `XWAction` decorator and base server mixins across all API services so they behave consistently.

- **DX goals / examples**

```python
class XWAuthServer(AAuthServer, XWApiServer):
    # Define all actions here via mixin design pattern,
    # using XWAction decorator for each action.
    #
    # XWAction decorator should enforce safety:
    #   - if "safe" mode is on, only allow execution through context or self,
    #   - prevent arbitrary script access beyond the object.
    #
    # Specify that the server engine is FastAPI (or similar).
    ...
```

- **Idea assessment**
  - **Makes sense**
    - Decorating actions and enforcing execution safety at the decorator level is a strong pattern.
  - **Needs refinement**
    - The relationship between `AAuthServer`, `XWApiServer`, and concrete services should be documented and consistent.

#### Improved DX version (aligned with current libs)

- **Unified base server class**: extract the shared pieces already living in `xwapi`/`xwauth` into a single `XWApiServerBase` that handles wiring, auth, and error handling.
- **Standard `@xwaction` decorator**: publish the decorator that current services use as the official way to define safe, schema-aware API methods.
- **Generated OpenAPI + clients**: leverage FastAPI/OpenAPI to auto-generate docs and clients, aligning with the existing pattern of strong docs in each lib.

---

### 21. Library: `xwterminal`

**DX maturity: 🟢 GREEN (small, focused, easy to ship).**

**Status: ❌ Not included as its own repo yet.** There is no `xwterminal` package at the root; CLI/terminal behavior currently lives under `xwsystem` and individual tools.  
To simulate `xwterminal` today, you expose commands via existing CLIs (e.g. `xwsystem`/tool scripts) and later unify them under a dedicated terminal experience.

- **Role**
  - Terminal for Exonware tools and CI.

- **Idea assessment**
  - **Makes sense**
    - Having a dedicated CLI/terminal experience is good for DX.

#### Improved DX version (aligned with current libs)

- **Wrap existing scripts into a single `xw` CLI**: pull together the Python scripts and tools scattered across repos into one entry point (e.g. `python -m exonware.xwterminal` or `xw` command).
- **Task-focused commands**: base commands on the most common flows you already automate (create project, run tests, benchmark, inspect data).
- **Extensible plugin system**: allow other libs (`xwdata`, `xwformats`, `xwui`) to register subcommands, mirroring how connectors and strategies are registered today.

---

### 22. Library: `xwlazy`

**DX maturity: 🟡 YELLOW (very valuable, mode matrix needs simplification).**

**Status: ✅ Already included and usable.** `xwlazy` exists as a concrete lib with modes and integration points (including `xwdata_integration.py`).  
You can start using it now for lazy loading/installation; future work is mostly simplifying the mode matrix into clearer presets (e.g. dev/ci/prod).

- **Role**
  - Lazy loading and on-demand install/uninstall of libs.
  - Used by `xwsystem` as a base for almost everything.

- **DX goals / examples**

```python
# Lazy modes

lazy        # enable lazy lib loading
lazy-smart  # lazy lib load and install lib on usage
lazy-full   # lazy lib load and install all libs on start
lazy-clean  # monitor, install, load, and uninstall
lazy-auto   # smart: install anything small, uninstall after finishing anything big

exonware.conf.lazy = "lite" | "smart" | "full" | "clean" | "auto"
```

- **Release notes (legacy plan)**
  - `xwlazy` v1.0 with async quiet install/uninstall, possibly as a mode.
  - Modes can be set in `__init__` or via meta info: `xwlazy-load-install-uninstall`, with tags to enable many configs.
  - There should be a mode that installs all packages from the first run (checks requirements and installs based on config).

- **Idea assessment**
  - **Makes sense**
    - Having lazy modes to optimize dev vs prod experiences is excellent DX.
  - **Needs refinement**
    - The number of modes is high; grouping them (e.g. dev/ci/prod presets) might be clearer.
  - **Probably bad**
    - Aggressively uninstalling dependencies in ways that surprise users (e.g. `lazy-clean`) if not clearly communicated.

#### Improved DX version (aligned with current libs)

- **Mode presets that map to real use cases**: collapse `lazy-*` variants into presets like `dev`, `ci`, `prod` that internally configure modes in ways you already use.
- **Observable behavior**: reuse the logging patterns from `xwsystem` so that installations/uninstalls show up clearly in logs and are easy to debug.
- **Tight integration with `xwdata` and `xwformats`**: keep `xwlazy` focused on speeding up heavy libs like `xwdata`/`xwformats`, instead of trying to manage every dependency in the stack.

---

### 23. Products and apps

**DX maturity: 🟡 YELLOW (good north stars, depend on core libs stabilizing).**

**Status: ❌ Not fully implemented as products yet.** `manam.ai` and the full “bot creator” platform are **targets** built on top of the core libs, not ready-made apps in this repo.  
To move toward them, you can already prototype using `xwdata`, `xwentity`, `xwui`, `xwauth`, and your preferred AI stack as the foundation.

- **manam.ai**
  - App to explain dreams.
  - Built on top of `xwai`/`xwbase`/`xwbots`.

- **Ultimate goal**
  - Build a **bot creator**: UI + WebApp + Backend, powered by:
    - `xwbase` (data/auth)
    - `xwai` (models)
    - `xwflow` (orchestration)
    - `xwbots` (bot runtime)

#### Improved DX version (aligned with current libs)

- **Start from existing building blocks**: implement early versions of `manam.ai` and the bot creator using today’s `xwdata`, `xwentity`, `xwui`, `xwauth`, and AI integrations, without waiting for `xwbase`/`xwflow`/`xwbots` packages.
- **Feed learnings back into libs**: capture what works best in those real apps and fold it back into the generic libs as new helpers, presets, and patterns.
- **Use the platform to dogfood DX**: treat these apps as “xwlib showcases” that demonstrate best practices and help iterate on the APIs in a realistic environment.

---

### 24. Naming and legacy notes

**DX maturity: 🟢 GREEN (clear and actionable).**

**Status: 🧩 Partially applied.** The codebase already leans heavily into `xw*`/`XW*` naming, but a few legacy names and aliases still exist.  
Concrete next step: add deprecation notes for old `X*`/alias imports and guide users toward the canonical `xw*` module and `XW*` class names.

- Some older code used `X*` names instead of `XW*`.  
  For DX and discoverability, it is better to:
  - Standardize on `xw*` module names and `XW*` classes.
  - Provide **aliases only where migration is needed**, and deprecate legacy names gradually.

