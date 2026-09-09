# Prompts de produção

Ferramenta: `imagegen` integrada, sem CLI/API de fallback. Referências: imagens de cachorro e meme fornecidas pelo usuário; bitmap Sirius existente no repositório. Os PNGs finais foram copiados para esta pasta com o alpha original. Recortes e pivôs são dados do renderer.

## caramelo-run.png

```text
Use case: stylized-concept. Asset type: production sprite sheet for a 2D Brazilian fighting game, transparent PNG. Primary request: the SAME recognizable Brazilian vira-lata caramelo short-haired mongrel in the supplied dog reference, running side view toward RIGHT, four successive clearly distinct phases of a run cycle. Input image: reference only for dog anatomy, coat and identity, not a background to copy. Short smooth ochre/caramel fur, lean medium build, floppy triangular ears, darker muzzle, thin long upward tail, long legs, absolutely not fluffy and not a corgi or golden retriever. Clean detailed painted game-art shading, realistic canine anatomy matched to illustrated urban game arenas. Layout: exactly four equal-width cells in ONE horizontal row, each dog centered at exactly same scale, baseline and facing right, generous padding so no feet/tail/ears cross cell boundary. Four distinct running poses: stretched flight, front contact, gathered underbody, rear push-off. Full dogs including all paws. No text, no grid, no border, no ground, no shadow, no added props. Truly transparent alpha background. Wide sheet 1536x1024 or similar; place all four dogs in the central horizontal band, identical common baseline.
```

## jessica-gesture.png (geração)

```text
Use case: stylized-concept. Asset type: 2D fighting game background NPC animation sheet with true transparent alpha. Reference image: provided meme screenshot, use her distinctive clothing, hairstyle, stance and gesture as visual reference. Primary request: three successive full-body poses of the same Brazilian female bystander reenacting the recognizable 'Já acabou, Jéssica?' stance in a nonviolent background cameo. Red sleeveless sports shirt with broad WHITE DIAGONAL SASH across torso and tiny blue detail; black fitted trousers; plain white sneakers; long straight dark brown loose hair. Ordinary proportions, relaxed casual street presence, arms low then raised slightly out to the sides with palms outward in a questioning shrug. Exactly THREE poses in one horizontal row, same character, equal scale, same baseline, separate generous margins, no overlap. Pose 1 hands down, pose 2 opens forearms palms upwards questioning, pose 3 more pronounced sideward questioning gesture with slight head tilt. Clean semi-realistic painted game-art shading that blends with detailed illustrated Sao Paulo city arena. Full feet and hair visible, no background, no shadows, NO words, NO watermark, NO grid. True transparent alpha. Wide 3-column layout; each centered within its equal-width cell; no weapons, no fight, no injuries.
```

## jessica-gesture.png (alpha)

```text
Edit: background-extraction. Keep the exact three full-body character drawings, clothing, poses, placement, scale, colors, and 1536x1024 composition UNCHANGED. Remove ALL the black/brown/grey backdrop and glow around them and shadows under feet. Replace it with actual TRANSPARENT alpha pixels across all areas outside the three characters. This is a production PNG sprite sheet, not a picture of a sheet. No solid color or checkered backdrop, real transparent alpha background. Preserve hair edges, fingers and white shoes. Nothing else changes.
```

## arena-sirius-clean.png

```text
Edit: precise-object-edit. Remove ONLY the seated furry brown dog in the lower left foreground of this Brazilian science arena (approximately x85..195 y408..515 on 960x540 image), including its dog shadow, and naturally reconstruct the pavement slab pattern, seams and ambient tree shadows behind that small region. Preserve exactly the existing entire scene composition, architecture, signage, people, lighting, color, perspective, arena geometry, 16:9 framing. No other changes. No new animals or objects. Production fighting game background.
```

