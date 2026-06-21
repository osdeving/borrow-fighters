# Rust Fighter Atlas

## Status

Placeholder candidato extraido de referencia.

## Arquivos

- `assets/placeholder/rust-fighter-atlas.png`
- `assets/placeholder/rust-fighter.sprite.json`
- `assets/placeholder/rust-gear-projectile.png`

## Uso

Atlas limpo com alpha real, frames em celulas padronizadas e metadata de animacao.

Este asset ainda nao esta plugado no runtime. Ele existe para validar:

- limpeza do fundo xadrez pintado;
- remocao dos labels da imagem original;
- normalizacao de pivot e baseline;
- formato de metadata para o futuro motor de sprites.

## Geracao

```bash
python3 tools/art/extract_rust_sprite_atlas.py
```

O script tambem gera uma previa local em `tmp/art/rust-fighter-atlas-preview.png`, que nao precisa ser versionada.

## Animacoes extraidas

- `idle`
- `walk`
- `crouch`
- `jump`
- `punch_light`
- `punch_heavy`
- `kick`
- `block`
- `hit`
- `taunt`
- `special`
- `projectile`

## Limitacoes

- A fonte ainda e uma imagem composta, nao um arquivo autorado em camadas.
- Alguns efeitos vem grudados aos frames originais e podem exigir redesenho manual depois.
- O motor atual ainda usa `fighter-greybox-spritesheet.png`; carregar este atlas depende de uma proxima etapa no renderer.
