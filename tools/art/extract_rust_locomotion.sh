#!/usr/bin/env bash
# Extract independent walk/kick PNGs from the retained generated chroma source.
set -euo pipefail
cd "$(dirname "$0")/../.."
directory=assets/adventure/locomotion
mkdir -p "$directory/frames"
# Remove the matte and its antialiased magenta edge without touching the
# charcoal/orange palette. Do not use this key on unrelated character art.
convert "$directory/source/rust-walk-kick.png" -alpha on -channel A \
  -fx '(r > g*1.6 && b > g*1.6 && b > r*0.6) ? 0 : 1' +channel \
  -crop 362x362 +repage "$directory/frames/pose-%02d.png"
# Tips of the next row extend over the mathematical cell boundary. Retain each
# measured sole plus two pixels, then restore the uniform transparent canvas.
heights=(355 355 355 355 348 348 348 348 347 347 347 340)
for index in "${!heights[@]}"; do
  printf -v frame '%s/frames/pose-%02d.png' "$directory" "$index"
  convert "$frame" -crop "362x${heights[index]}+0+0" +repage \
    -background none -gravity North -extent 362x362 "$frame"
done
