# Build a Scrollcase box

This Codespace starts as an empty Scrollcase project in a Git repository. Follow these steps to
create, build, sign, and verify a small Linux CPU box.

## 1. Install Scrollcase

```sh
npm install --global scrollcase
```

See [Installation](https://scrollcase.dev/getting-started/installation) for other installation
options.

## 2. [Initialize the project](https://scrollcase.dev/reference/cli#init)

```sh
scrollcase init --install-toolchain < /dev/null
```

This creates a runnable `example-box` and installs its project-local build toolchain. Optional
consumer packages are skipped, while their example templates are still created.

## 3. [Lock the environment](https://scrollcase.dev/reference/cli#lock)

```sh
scrollcase lock example-box/linux-x86_64-cpu
```

Commit the generated project and lock before building:

```sh
git add .
git commit -m "Initialize Scrollcase example"
git status --short
```

The final command must print nothing: the scroll, lock, generated entry point, and consumer
templates now belong to the demo commit. This is what lets Scrollcase record honest provenance;
`--allow-dirty` is neither needed nor used in this walkthrough.

## 4. [Create a signing key](https://scrollcase.dev/reference/cli#keygen)

```sh
scrollcase keygen
```

The private key stays under the ignored `.scrollcase/` directory. Never commit or share it.

## 5. [Build the box](https://scrollcase.dev/reference/cli#build)

```sh
scrollcase build example-box/linux-x86_64-cpu --weights embed
```

Scrollcase installs only the locked environment, runs the self-test with the box's own Python,
creates the deterministic archive, and signs its release document.

## 6. [Verify the result](https://scrollcase.dev/reference/cli#verify)

```sh
scrollcase verify .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json --self-test
```

The signed release, archive hash, archive entries, manifest, and contained Python runtime are now
verified.

## Explore further

- Run the box with [`scrollcase run`](https://scrollcase.dev/reference/cli#run).
- Or use the shorter [box-run demo](https://scrollcase.dev/demos/box-run-demo) if you only want to
  verify and run an already-built box.
- Create a real project scroll with [`scrollcase new scroll`](https://scrollcase.dev/reference/cli#new).
- Diagnose a build machine with [`scrollcase doctor`](https://scrollcase.dev/reference/cli#doctor).
- Review dependency licences with [`scrollcase audit`](https://scrollcase.dev/reference/cli#audit).
- Read the complete [Quickstart](https://scrollcase.dev/getting-started/quickstart).

## Run the generated consumers

After the build, replace `<target>` and `<hash>` in the generated template you want to use. The
non-interactive `init` command above creates all three templates but skips their optional dependency
installation, so install the consumer you want before running it.

### Node

```sh
npm install scrollcase
npm install --save-dev tsx typescript
npx tsx consumer-templates/run-box.ts
```

### Python

```sh
python -m pip install scrollcase-consumer
python consumer-templates/run_box.py
```

### Rust

The Rust template requires Rust and Cargo, which are not installed by this Codespace.

```sh
cargo add --manifest-path consumer-templates/rust/Cargo.toml scrollcase-consumer
cargo run --manifest-path consumer-templates/rust/Cargo.toml
```

Each consumer verifies the local signed release and box before running its signed entry point. See
the [consumer APIs](https://scrollcase.dev/reference/api) for application integration details.
