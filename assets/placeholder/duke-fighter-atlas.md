# Duke Fighter Atlas

## Status

Placeholder candidato extraido de referencia.

## Arquivos

- `assets/placeholder/duke-fighter-atlas.png`
- `assets/placeholder/duke-fighter.sprite.json`
- `assets/placeholder/duke-bean-projectile.png`
- `assets/placeholder/duke-fighter-atlas.before-white-cleanup.png`: backup do atlas antes da limpeza do corpo branco.

## Uso

Atlas limpo com alpha real, frames em celulas padronizadas e metadata de animacao.

Este asset entra no runtime como o primeiro visual atlas-driven do Player 2 enquanto Java/Duke ainda nao tem arte final aprovada.

A extracao aplica uma limpeza conservadora na parte branca do corpo para remover manchas escuras pequenas sem apagar contorno, logo, olho, arma ou efeitos.

## Geracao

```bash
python3 tools/art/extract_duke_sprite_atlas.py
```

O script tambem gera uma previa local em `tmp/art/duke-fighter-atlas-preview.png`, que nao precisa ser versionada.

## Animacoes Extraidas

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
- O especial e mais largo que os sprites comuns por causa da arma.
- Os projeteis de graos ainda sao placeholder e podem virar efeitos separados depois.
