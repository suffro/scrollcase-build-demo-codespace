# Build a Scrollcase box

Paste these commands into the terminal to create, sign, build, verify, and run a small Linux CPU
box.

## 1. Initialize

```sh
scrollcase init
scrollcase lock example-box/linux-x86_64-cpu
```

## 2. Save the locked project

```sh
git add . && git commit -m "Lock Scrollcase demo"
```

This commit exists only inside your unpublished template Codespace. It has no remote and cannot
change the Scrollcase demo repository.

## 3. Sign and build

```sh
scrollcase keygen
scrollcase build example-box/linux-x86_64-cpu
```

## 4. Verify and run

```sh
scrollcase verify .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
scrollcase run .scrollcase/dist/boxes/example-box/1.0.0/linux-x86_64-cpu/*.release.json
```

## Use a consumer

`init` also creates quick Node, Python, and Rust examples under `consumer-templates/`. Point one at
the built target and release hash to run the box from an application.

[Quickstart](https://scrollcase.dev/getting-started/quickstart) ·
[CLI reference](https://scrollcase.dev/reference/cli) ·
[Consumer APIs](https://scrollcase.dev/reference/api)
