#!/bin/bash
# Simple script to push the docs.

set -ex

TMPDIR="/tmp/rust-docs-$$"

mkdir $TMPDIR
cargo doc
cp -R ./target/doc/* $TMPDIR
git checkout gh-pages
rm -rf ./*
cp -R $TMPDIR/* ./
git add -A
git commit -m 'Update docs'
git push origin gh-pages
rm -rf $TMPDIR
git checkout master
