# Julia — corrida inteira

Fonte criada com a ferramenta imagegen integrada, a partir de `assets/adventure/actors/julia/sprites/idle.png` e `run.png`. Identidade, roupas, bolsa e estilo pintado preservados como referencia; sem Blender. Prompts usados em `prompts.md`.

`run-sheet.png`: seis poses inteiras, grid 3×2 de 512×512, alpha original. Uma revisao dirigida alternou os bracos na segunda metade da passada. Pivots e escala ficam na receita; o builder copia pixels sem resize, deformacao ou retoque. Revisao estatica e piloto nativo de 120 quadros concluidos em `assets-generated/reviews/julia-run/`; folha run conferida nos dois tamanhos.

Reproduzir: `./dev art build assets-src/adventure/augusta/julia/recipe.json`. Runtime usa os recortes gerados em `assets/adventure/actors/julia/sprites/run_*.png`, registrados em `rig.json` e nas chaves de `clips.json`. Combate, demais poses e contato Julia/broker permanecem nos artefatos anteriores.
