# C Start Atlas

## Status

Placeholder de entrada cinematografica do C, extraido da linha `INTRO` de `assets/references/langc-03.png`.

## Arquivos

- `assets/references/langc-03.png`
- `assets/placeholder/c-start-atlas.png`
- `assets/placeholder/c-start.sprite.json`

## Geracao

```bash
python3 tools/art/extract_c_sprite_atlases.py
```

O manifesto possui o clip `spawn` com 7 frames nao loopaveis. O jogo usa esse clip no inicio da luta antes da contagem `11`, `10`, `01`, `Fight!`.

O gerador adiciona pequenas particulas `0`/`1` sobre alguns frames para deixar a entrada mais cinematografica e conectar o C ao bitstream sem cobrir a silhueta.

## Limitacoes

- Ainda e placeholder.
- O timing pode precisar de repeticao ou duracao especifica por frame depois de playtest.
- O atlas de entrada nao deve conter regra de combate; ele existe apenas para apresentacao antes do round.
