# Build a Scrollcase box

Paste these commands into the terminal to create, sign, build, verify, and run a small Linux CPU
box.

## 1. Install Scrollcase

```sh
npm install -g scrollcase
```

## 2. Initialize

```sh
scrollcase init
scrollcase lock example-box/linux-x86_64-cpu
```

## 3. Save the locked project

Scrollcase refuses to build from a dirty Git working tree, so save the generated files in a local
commit first:

```sh
git add . && git commit -m "Lock Scrollcase demo"
```

This Codespace has no remote, so the commit stays here and cannot change the demo repository.

## 4. Sign and build

```sh
scrollcase keygen
scrollcase build example-box/linux-x86_64-cpu
```

## 5. Verify

```sh
scrollcase verify .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
```

## How to run the box

```sh
scrollcase run .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
```

`init` also creates quick Node, Python, and Rust examples under `consumer-templates/`. Point one at
the built target and release hash to run the box from an application.

[Quickstart](https://scrollcase.dev/getting-started/quickstart) ·
[CLI reference](https://scrollcase.dev/reference/cli) ·
[Consumer APIs](https://scrollcase.dev/reference/api)
