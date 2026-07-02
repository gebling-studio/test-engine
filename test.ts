#!/usr/bin/env bun

import { run } from "./build/run";

const thisPath = import.meta.dir;

run("cargo test --all");
run("cargo test -p valid");

process.chdir("deps/ui/ui_views/");
run("cargo test --all");
process.chdir(thisPath);

process.chdir("deps/text/");
run("cargo test --all");
process.chdir(thisPath);
