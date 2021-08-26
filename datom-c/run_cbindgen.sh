#!/bin/bash

set -euxo pipefail

cbindgen --config cbindgen.toml --crate datom-c --output datom.h
