#!/usr/bin/env bash
# Extract the authored front/back depth walk without rewriting pivots or clips.
set -euo pipefail
cd "$(dirname "$0")/../.."
directory=assets/adventure/locomotion
mkdir -p "$directory/crossing"
matte=$(mktemp --suffix=.png)
trap 'rm -f "$matte"' EXIT
convert "$directory/source/crossing-walk.png" -alpha on -channel A \
  -fx '(r > g*1.6 && b > g*1.6 && b > r*0.6) ? 0 : 1' +channel "$matte"
rows=(20 330 642 946)
heights=(304 304 304 308)
for view in back front; do
  start=0
  if [[ "$view" == front ]]; then start=2; fi
  for index in {0..7}; do
    column=$((index % 4))
    row=$((start + index / 4))
    left=$((7 + column * 314))
    printf -v output '%s/crossing/%s-%02d.png' "$directory" "$view" "$index"
    convert "$matte" -crop "300x${heights[row]}+$left+${rows[row]}" +repage "$output"
  done
done
