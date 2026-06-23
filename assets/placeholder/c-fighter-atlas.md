# C Fighter Atlas

## Status

Placeholder jogavel extraido de atlas com chroma key para validar o personagem C no runtime, no Combat Lab e no Sprite Combat Viewer.

## Arquivos

- `assets/references/langc-03.png`
- `assets/references/langc-04.png`
- `assets/placeholder/c-fighter-atlas.png`
- `assets/placeholder/c-fighter.sprite.json`
- `assets/placeholder/c-bitstream-projectile.png`

## Geracao

```bash
python3 tools/art/extract_c_sprite_atlases.py
```

O script remove o chroma key magenta das referencias, descarta as legendas desenhadas no atlas original e gera um atlas runtime com alpha real. A grade final usa celulas `384x256`, `scale = 1.5467` no runtime `1280x720` e clips de luta para idle, walk, crouch, jump, block, hit, knockdown, taunt, victory, socos, chutes, special e projectile. Esse `1.5467` preserva o ajuste relativo anterior de `1.16` migrado por `4/3`.

O frame `projectile_0` e substituido por um bitstream placeholder gerado localmente, com glifos `0` e `1` grandes o suficiente para leitura durante a luta.

## Uso

Este asset serve para testar:

- C como personagem selecionavel no menu e por CLI;
- continuidade de frames com mais animacao que o greybox original;
- escala humanoide contra Rust e Duke;
- origem visual do projectile no clip `special`;
- leitura clara de `0` e `1` saindo do personagem no projectile;
- leitura de ataques no Combat Lab com o kit proprio de fundamentos de C.

## Limitacoes

- Ainda e placeholder, nao arte final.
- O kit terrestre ja possui frame data propria, mas ataques aereos ainda usam os movimentos universais do prototipo.
- Hitboxes e hurtboxes por frame ainda precisam ser revisadas no Sprite Combat Viewer.
- O projectile foi separado do sprite para facilitar ajuste de origem, tamanho e velocidade.
