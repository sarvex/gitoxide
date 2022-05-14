#!/bin/bash
set -eu -o pipefail

mkdir repo && cd repo
git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3
git commit -q --allow-empty -m c4
git branch dt1
git branch d1

mkdir -p .git/refs/remotes/origin

cp .git/refs/heads/main .git/refs/remotes/origin/
cp .git/refs/heads/main .git/refs/d1

echo "ref: refs/remotes/origin/main" > .git/refs/remotes/origin/HEAD
echo "notahexsha" > .git/refs/broken

echo "ref: refs/heads/multi-link-target1" > .git/refs/multi-link
echo "ref: refs/tags/multi-link-target2" > .git/refs/heads/multi-link-target1
echo "ref: refs/remotes/origin/multi-link-target3" > .git/refs/tags/multi-link-target2
git rev-parse HEAD > .git/refs/remotes/origin/multi-link-target3


echo "ref: refs/loop-b" > .git/refs/loop-a
echo "ref: refs/loop-a" > .git/refs/loop-b

git tag t1
git tag -m "tag object" dt1

git bisect start && git bisect bad HEAD

git worktree add ../w-detached HEAD~1
(
  cd ../w-detached
  git bisect start
  git bisect bad HEAD
)

git worktree add ../w1
(
  cd ../w1
  git reset --hard HEAD~2

  git bisect start
  git bisect bad HEAD
)

if [ "${1:-}"  = "packed" ]; then
  git pack-refs --all --prune
fi
