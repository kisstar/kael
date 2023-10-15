#!/usr/bin/env bash

if [ -z "$1" ]; then
	echo "Please provide a tag."
	echo "Usage: ./scripts/release.sh v[X.Y.Z]"
	exit
fi

echo "Preparing $1..."

# update the changelog file
git cliff -t "$1" -o CHANGELOG.md
# create a signed tag
git add -A && git commit -m "chore(release): $1"
git tag -a "$1" -m "Release $1"

echo "Done!"
echo "Now push the commit (git push) and the tag (git push --tags)."
