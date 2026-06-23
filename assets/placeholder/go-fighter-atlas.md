# Go Fighter Atlas

## Status

Placeholder candidato gerado por IA para validar Go/Gopher no Sprite Combat Viewer.

## Arquivos

- `assets/placeholder/go-fighter-atlas.png`
- `assets/placeholder/go-fighter.sprite.json`
- `assets/placeholder/go-start-atlas.png`
- `assets/placeholder/go-start.sprite.json`
- `assets/placeholder/go-channel-projectile.png`

## Uso

Atlas com alpha real, grade 6x6, celulas `384x256`, pivot padrao `112,236` e clips basicos de luta. A versao atual foi comprimida horizontalmente em torno do pivot, teve ilhas soltas de frames limpas e usa `scale = 1.08` para preservar o Gopher sem deixar o corpo visual baixo/largo demais perto de Rust e C.

Este asset serve para testar:

- leitura do Gopher como personagem de luta;
- consistencia de olhos e proporcoes entre frames;
- baseline/pivot no Sprite Combat Viewer;
- golpes curtos de rushdown;
- entrada cinematografica com fan-out/fan-in de concorrencia;
- origem visual do projectile no clip `special`.

## Atribuicao

O Go Gopher original foi criado por Renee French. Este arquivo e um derivado candidato para prototipo e nao representa arte final nem endosso oficial do projeto Go.

Antes de usar em release publica, revisar licenca/atribuicao nos docs de assets.

## Limitacoes

- Gerado como atlas raster, nao como arquivo em camadas.
- Ainda precisa de revisao manual de hitbox/hurtbox por frame.
- Ja esta conectado ao runtime normal do Go, mas continua sendo placeholder de proporcao, leitura e pipeline.
