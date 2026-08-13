# Build a Scrollcase box

Follow these steps to create, sign, build, verify, and run a small Linux CPU box.

## 1. Install Scrollcase CLI

```sh
npm install -g scrollcase
```

## 2. Initialize Scrollcase project

`init` creates the Scrollcase workspace, including a disposable runnable example.

```sh
scrollcase init
```

> **2.1:** In a real project at this point you would develop what the box does

## 3. Lock the project

```sh
scrollcase lock example-box/linux-x86_64-cpu
```

`lock` runs the pinned pixi against the scroll's `pixi.toml` and writes `pixi.lock` next to it. This is what makes a build reproducible and what the licence audit reads.

## 4. Git commit

Scrollcase refuses to build from a dirty Git working tree, so save the generated files in a local
commit first:

```sh
git add . && git commit -m "Lock Scrollcase demo"
```

This Codespace has no remote, so the commit stays here and cannot change the demo repository.

## 5. Sign and build

```sh
scrollcase keygen

scrollcase build example-box/linux-x86_64-cpu
```

## 6. Verify

```sh
scrollcase verify .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
```

**✓ That's it**

---

<br>

## How to run the box

There are 3 ways to run a Scrollcase box:

### a. Scrollcase CLI

```sh
scrollcase run .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
```

### b. Scrollcase consumers <small> (Node, Python or Rust) </small>

`init` also creates quick **Node, Python, and Rust** examples under `consumer-templates/`. Point one at
the built target and release hash to run the box from an application.

### c. Your custom implementation

<br>

---

### Docs quick-links:

[Overview](https://scrollcase.dev/getting-started/overview) ·
[Quickstart](https://scrollcase.dev/getting-started/quickstart) ·
[CLI reference](https://scrollcase.dev/reference/cli) ·
[Consumer APIs](https://scrollcase.dev/reference/api)
