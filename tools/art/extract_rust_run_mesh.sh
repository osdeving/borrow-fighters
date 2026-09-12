#!/usr/bin/env bash
# Extract the selected imagegen import plate; never redraw the character in code.
set -euo pipefail
cd "$(dirname "$0")/../.."
convert assets/adventure/locomotion/source/run-mesh/leg-keyed.png \
  -alpha on -channel A -fx '(r>g*1.3 && b>g*1.3 && b>0.25)?0:1' +channel \
  -trim +repage -background black -alpha background \
  assets/adventure/locomotion/run-mesh/leg.png
