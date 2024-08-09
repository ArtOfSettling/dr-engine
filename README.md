# Dread Engine

**NOTE**
For now, this project is stand-alone build-able, however, it should primarily be built through the [dread](https://github.com/ArtOfSettling/dread) repo, where it is added as a git submodule. This will change in the future, when this package will ultimately be pushed to [crates.io](https://crates.io).

## Overview

This package represents the engine. This package will build a static library that can be used by anyone who wants to create games with this engine. See [Dread Runtime](https://github.com/ArtOfSettling/d-runtime) or [Dread Editor](https://github.com/ArtOfSettling/dr-editor) for reference.

## Prerequisites

1. Install CMake

## How to build

1. `cargo build` / `cargo build --release` {depending on the profile you want to build}
