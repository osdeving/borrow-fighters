# Python Start Atlas Placeholder

Placeholder candidato gerado a partir de `assets/references/python-start-atlas-source.png`.

Arquivos gerados:

- `assets/placeholder/python-start-atlas.png`
- `assets/placeholder/python-start.sprite.json`
- `tmp/art/python-start-atlas-preview.png` (previa local, nao versionada)

Uso:

```bash
python3 tools/art/build_python_start_atlas.py
```

O manifesto possui o clip `spawn` com 12 frames nao loopaveis. A animacao mostra a personagem Python trazendo um cavalete, abrindo um grafico de barras colorido e apontando a relacao entre variaveis antes da contagem `11`, `10`, `01`, `Fight!`.

Notas:

- O source foi gerado como folha de sprites em chroma key, convertido para alpha e salvo em `assets/references/`.
- O script remove os numeros da folha fonte antes de gerar o atlas runtime.
- A animacao e cinematografica; nao deve carregar hitbox/hurtbox de gameplay.
- O asset ainda e placeholder e precisa de revisao final de escala, leitura e continuidade antes de virar arte aprovada.
