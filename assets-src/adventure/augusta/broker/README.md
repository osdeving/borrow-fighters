# broker — corrida inteira

Fonte criada com imagegen integrado a partir de `assets/adventure/actors/broker/sprites/flee.png`; identidade, roupa e pintura originais como referencia, sem Blender. Prompts usados em `prompts.md`.

`run-sheet.png`: seis poses completas em grid 3×2 de 512×512, alpha original. Uma correcao dirigida alternou os bracos na segunda metade. Receita registra pivots/escala; o builder recorta sem resize, deformacao ou retoque. Runtime usa `sprites/run_*.png` nas chaves run/start; ataque, guardas, queda e contato permanecem nos artefatos existentes.

Reproduzir: `./dev art build assets-src/adventure/augusta/broker/recipe.json`. Revisao nativa curta registrada em `assets-generated/reviews/broker-run/`.
