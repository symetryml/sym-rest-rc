# SymetryML Rest client in Rust

This repository contains an implementation of basic functionality for a rest client interacting with the
SymetryML rest server. [https://docs61.symetryml.io/symetryml-rest-client/rest-documentation]

Usage. See [COMMANDS.md](./COMMANDS.md) for details or use the help:

```
sym-rest-rc -h
```

# Functionalities

For now only a small subset is implemented, but the major Rest endpoint are implemented:

1. Create a SymetryProject: `create` command.
1. Push data to to a project: `learn` command.
1. Build a model: `build` command.
1. Check status of a model build request - Since building a model is an asynchronous method: `job` command.
1. Make prediction using an existing model: `predict` command.

