#!/bin/bash
# Simple script to push the docs.

REPO="git@github.com:SBSTP/loirc.git"
TMPDIR="/tmp/rust-docs-$$"

echo "REPO: $REPO"
echo "TMPDIR: $TMPDIR"

mkdir $TMPDIR
cd $TMPDIR

git clone $REPO repo
mkdir docs

cd repo
if cargo doc ; then
    cp -R target/doc/* ../docs
    git checkout gh-pages
    rm -rf *
    cp -R ../docs/* ./
    git add -A
    git commit -m 'Update docs'
    git push origin gh-pages
fi

rm -rf $TMPDIR
