# Injective protobuf definitions and generated code

This repository contains the canonical Injective proto source files and pre-compiled generated code for the supported languages:

- C++ (`cpp_protos.zip`)
- C# (`csharp_protos.zip`)
- Java (`java_protos.zip`)
- Python (`python_protos.zip`)
- Rust (`rust_protos.zip`)

Use the zip files as drop-in generated code for your project. The `all_protos/` directory is the source-of-truth snapshot of the proto files that were used to produce the current zips.

## Original proto files

All original proto files can be found in the `all_protos/` folder. This directory is regenerated from `proto/` every time `make generate` (or `make run-full`) is executed and reflects exactly the proto inputs used for the last published compilation.

## Maven dependencies for Java

If you are using the Injective proto components in Java you might want to use the following Maven dependencies (or use it as the base for your own Maven configuration):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.test</groupId>
    <artifactId>injective_proto</artifactId>
    <version>1.0-SNAPSHOT</version>

    <properties>
        <maven.compiler.source>1.8</maven.compiler.source>
        <maven.compiler.target>1.8</maven.compiler.target>
    </properties>

    <dependencies>
        <dependency>
            <groupId>com.google.protobuf</groupId>
            <artifactId>protobuf-java</artifactId>
            <version>3.25.1</version>
        </dependency>

        <dependency>
            <groupId>io.grpc</groupId>
            <artifactId>grpc-protobuf</artifactId>
            <version>1.59.0</version>
        </dependency>

        <dependency>
            <groupId>io.grpc</groupId>
            <artifactId>grpc-stub</artifactId>
            <version>1.59.0</version>
        </dependency>

        <dependency>
            <groupId>javax.annotation</groupId>
            <artifactId>javax.annotation-api</artifactId>
            <version>1.3.2</version>
        </dependency>
    </dependencies>

</project>
```

---

## Maintainer guide

This section documents the process for updating the proto definitions and regenerating all compiled outputs when a new Injective chain version is released.

### Prerequisites

The following tools must be available on `PATH` before running any `make` targets:

- [`buf`](https://buf.build/docs/installation) CLI — used for proto export and remote code generation.
- `python3` — required by `scripts/normalize_generated.py` (normalization of C# and Rust outputs).
- `git` — used to clone the source repositories.
- `zip` — used by `make pack` to produce the output archives.
- Internet access to `github.com` — `buf export` and `git clone` fetch from InjectiveLabs forks hosted there.

No local `protoc`, `grpc.tools`, or Rust protoc plugins are needed. All code generation is handled by pinned Buf remote plugins declared in `buf.gen.yaml`.

### Repository layout during a build

The Makefile distinguishes between transient build artifacts and committed outputs.

**Transient directories** (created during the build, cleaned up at the end of `make run-full`, never committed):

| Path | Purpose |
|------|---------|
| `proto/` | Merged proto sources assembled from all upstream repositories |
| `third_party/` | Raw output of `buf export` for dependency protos (cosmos-sdk, ibc-go, wasmd, cometbft, ics23) |
| `injective-core/` | Shallow clone of [InjectiveLabs/injective-core](https://github.com/InjectiveLabs/injective-core) |
| `injective-indexer/` | Shallow clone of [InjectiveLabs/injective-indexer](https://github.com/InjectiveLabs/injective-indexer) |
| `cpp/`, `csharp/`, `java/`, `python/`, `rust/` | Per-language generated output before packing |

**Committed outputs** (updated by a maintainer and pushed as part of a version update):

| Path | Purpose |
|------|---------|
| `all_protos/` | Snapshot of the `proto/` tree used in the last generation |
| `cpp_protos.zip` | Generated C++ code |
| `csharp_protos.zip` | Generated C# code |
| `java_protos.zip` | Generated Java code |
| `python_protos.zip` | Generated Python code |
| `rust_protos.zip` | Generated Rust code |

### Version inputs

The top of `Makefile` contains six version variables. These are the only values a maintainer normally needs to change for a routine chain version update:

```makefile
COMETBFT_VERSION_TAG=v1.0.1-inj.7
COSMOS_SDK_VERSION_TAG=v0.50.14-inj.9
IBC_GO_VERSION_TAG=v8.7.0-inj.4
WASMD_VERSION_TAG=v0.53.3-inj.3
INJECTIVE_CORE_VERSION_TAG=v1.20.0-alpha.3
INJECTIVE_INDEXER_VERSION_TAG=v1.19.41
```

- `INJECTIVE_CORE_VERSION_TAG` — the new chain version. This tag is cloned from [InjectiveLabs/injective-core](https://github.com/InjectiveLabs/injective-core) and its `proto/injective/` tree is the primary source of Injective-specific protos.
- `INJECTIVE_INDEXER_VERSION_TAG` — the matching release of [InjectiveLabs/injective-indexer](https://github.com/InjectiveLabs/injective-indexer), which supplies the exchange/indexer RPC protos under `proto/exchange/`.
- `COMETBFT_VERSION_TAG`, `COSMOS_SDK_VERSION_TAG`, `IBC_GO_VERSION_TAG`, `WASMD_VERSION_TAG` — InjectiveLabs forks of the respective upstream libraries. Bump these to the versions pinned by the new `injective-core` release (check the `go.mod` or release notes of `injective-core`).
- `ics23` is fetched from upstream `cosmos/ics23` without a pinned tag and is not configurable here.

### Pinned codegen plugins

`buf.gen.yaml` pins all remote code generation plugins for reproducibility:

- `buf.build/protocolbuffers/*:v31.1` and `buf.build/grpc/*:v1.76.0` — core language plugins.
- `buf.build/community/neoeinstein-prost*:v0.5.0` — Rust prost/tonic plugins.

Update plugin versions in `buf.gen.yaml` only when deliberately upgrading the generated code style, not as part of a routine chain version bump.

---

### Step-by-step: updating for a new chain version

#### 1. Identify the target versions

From the `injective-core` release you are targeting, determine the correct versions of all dependencies:

- The `INJECTIVE_CORE_VERSION_TAG` is the release tag itself (e.g. `v1.20.0`).
- Find `COMETBFT_VERSION_TAG`, `COSMOS_SDK_VERSION_TAG`, `IBC_GO_VERSION_TAG`, and `WASMD_VERSION_TAG` in the `go.mod` of that `injective-core` tag, or in its release notes.
- Find `INJECTIVE_INDEXER_VERSION_TAG` from the indexer release corresponding to this chain version.

#### 2. Update version tags in the Makefile

Edit lines 1–6 of `Makefile` to set the six `*_VERSION_TAG` variables to the values identified in step 1.

#### 3. Run the full generation pipeline

```bash
make run-full
```

This single command runs the complete pipeline in sequence:

1. **`clean-all`** — removes any existing transient directories (`proto/`, `third_party/`, `injective-core/`, `injective-indexer/`, language output dirs) and existing `*_protos.zip` files.
2. **`clone-all`** — shallow-clones `injective-core` and `injective-indexer` at the pinned tags (skips if already present).
3. **`generate`**:
   - **`sync-protos`** assembles the unified `proto/` tree:
     - `download-protos`: runs `buf export` for cosmos-sdk, ibc-go, wasmd, cometbft (using the InjectiveLabs forks at the pinned tags), and ics23; copies the output into `third_party/`; then copies `injective-core/proto/injective` and everything from `third_party/` into `proto/`.
     - `download-indexer-protos`: copies all `.proto` files from `injective-indexer/api/gen/grpc` into `proto/exchange/`.
   - Runs `buf generate --template buf.gen.yaml` against `proto/`, which calls all pinned remote plugins and writes output into `cpp/`, `csharp/proto/`, `java/`, `python/`, and `rust/proto/`.
   - Runs **`normalize-generated`** (see [About normalize-generated](#about-normalize-generated) below).
   - Replaces `all_protos/` with a fresh copy of `proto/`.
4. **`pack`** — zips each language output directory into the five `*_protos.zip` archives.
5. **Final cleanup** — removes `injective-core/`, `injective-indexer/`, `proto/`, `third_party/`, and all per-language output directories so the working tree is clean.

#### 4. Verify the output

```bash
git status
```

The working tree should show changes only in:
- `Makefile` (the version tag bumps from step 2)
- `all_protos/` (updated proto snapshot)
- `cpp_protos.zip`, `csharp_protos.zip`, `java_protos.zip`, `python_protos.zip`, `rust_protos.zip`

If unexpected files appear in `git status`, investigate before committing.

#### 5. Commit and open a pull request

```bash
git checkout -b update-protos-v1.20.0
git add Makefile all_protos/ cpp_protos.zip csharp_protos.zip java_protos.zip python_protos.zip rust_protos.zip
git commit -m "chore: update protos for injective-core v1.20.0"
git push origin update-protos-v1.20.0
```

Open a pull request and merge it to `master` after review.

#### 6. Tag the release

After merging to `master`, create and push a git tag that matches the chain version:

```bash
git checkout master
git pull origin master
git tag v1.20.0
git push origin v1.20.0
```

This repository does not use GitHub releases — tags are the authoritative version markers.

---

### Granular Make targets

The following targets are available for iterative or partial runs:

- **`make run-full`** — full end-to-end pipeline (clean → clone → generate → pack → clean transient). The normal entry point for a routine version update.
- **`make generate`** — syncs protos, runs `buf generate`, normalizes output, and refreshes `all_protos/`. Assumes `injective-core` and `injective-indexer` are already cloned. Use this to re-run generation after manually adjusting proto files or `buf.gen.yaml` without re-cloning.
- **`make sync-protos`** — assembles `proto/` from all upstream sources (runs `download-protos` + `download-indexer-protos`). Requires the repos to already be cloned.
- **`make download-protos`** — exports dependency protos via `buf export` and assembles `proto/` excluding indexer protos. Requires `injective-core` to be cloned.
- **`make download-indexer-protos`** — copies exchange RPC protos from `injective-indexer` into `proto/exchange/`. Requires `injective-indexer` to be cloned.
- **`make clone-all`** — clones both `injective-core` and `injective-indexer` at their pinned tags (no-op if already present).
- **`make clone-injective-core`** — clones only `injective-core` (no-op if already present).
- **`make clone-injective-indexer`** — clones only `injective-indexer` (no-op if already present).
- **`make normalize-generated`** — runs `scripts/normalize_generated.py` against the existing language output dirs. Use this when iterating on the normalization script without re-running full generation.
- **`make pack`** — zips existing per-language output directories into `*_protos.zip`. Use this to repackage without re-generating.
- **`make clean-all`** — removes all transient directories and zip archives.

### About normalize-generated

`scripts/normalize_generated.py` post-processes the raw `buf generate` output to produce a consistent directory tree:

- **C#**: moves generated `.cs` files from the flat `buf` output into a directory tree that mirrors the source proto path (read from the `source:` annotation in each generated file).
- **Rust**: moves generated `.rs` files into a subdirectory matching the proto package's source path, and rewrites the `include!` paths in `rust/proto/mod.rs` accordingly. Also generates `exchange/mod.rs` for the flat exchange package files.
- **Compatibility shims**: copies `cosmos.crypto.multisig.v1beta1` generated files to the legacy `cosmos.crypto.multisig` path (both C# and Rust) for backward compatibility.
- **Rust symbol rename**: renames the `Validators` enum in `cosmos.staking.v1beta1` to `EnumValidators` to avoid a Rust naming conflict.

If you manually edit files under `rust/proto/` or `csharp/proto/`, run `make normalize-generated` afterwards to re-apply these transformations.

### Troubleshooting

- **`buf generate` fails with an authentication or permission error**: remote plugins on the Buf Schema Registry require an authenticated session. Run `buf registry login` and follow the prompts to authenticate with your Buf account before running any `make` target that invokes `buf generate`.
- **`buf export` fails with a 404 or "tag not found"**: the InjectiveLabs fork must have the tag you set in `Makefile` pushed before the export can succeed. Verify the tag exists on the remote before bumping the version.
- **`git clone` fails for `injective-core` or `injective-indexer`**: same cause — confirm the tag exists on the respective repository and that you have network access to `github.com`.
- **`make sync-protos` can be retried safely**: it starts by wiping `proto/` and `third_party/`, so it is safe to re-run after a partial failure.
- **`rust/proto/mod.rs` must not be deleted manually**: the Rust output relies on the `prost-crate` plugin emitting `include_file=mod.rs`. The normalization script rewrites paths inside this file; if it is missing, Rust consumers will not be able to resolve the generated modules.
- **Unexpected files after generation**: if `git status` shows files outside `Makefile`, `all_protos/`, and the five zips, the cleanup at the end of `make run-full` may not have completed. Run `make clean-all` and re-generate, or inspect the Makefile's `clean_*` macros to understand what is being deleted.
