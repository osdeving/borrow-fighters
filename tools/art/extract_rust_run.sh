#!/usr/bin/env bash
# Extract the nine generated run components. Joint/scale edits stay in catalog.json.
set -euo pipefail
cd "$(dirname "$0")/../.."
directory=assets/adventure/locomotion
mkdir -p "$directory/run"
matte=$(mktemp --suffix=.png)
trap 'rm -f "$matte"' EXIT
convert "$directory/source/run-rig.png" -alpha on -channel A \
  -fx '(r > g*1.6 && b > g*1.6 && b > r*0.6) ? 0 : 1' +channel "$matte"
while read -r name crop; do
  convert "$matte" -crop "$crop" +repage "$directory/run/$name.png"
done <<'PARTS'
body 320x455+90+15
near-arm 315x262+503+132
far-arm 315x260+915+132
near-thigh 180x304+136+500
near-shin 153x280+552+507
near-boot 267x192+911+612
far-thigh 196x321+106+842
far-shin 165x280+539+872
far-boot 282x200+899+977
PARTS
