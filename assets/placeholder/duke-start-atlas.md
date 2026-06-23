# Duke Start Atlas

## Status

Placeholder candidato extraido de `assets/references/duke-start-anim.png`.

## Arquivos

- `assets/placeholder/duke-start-atlas.png`
- `assets/placeholder/duke-start.sprite.json`
- `assets/references/duke-start-anim.png`

## Uso

Atlas cinematografico usado no clip `spawn`, antes de liberar os controles no inicio da luta.

O script restaura mesa e xicara nos frames em que a referencia tinha falhas de continuidade e remove componentes isolados nos frames `spawn_09`/`spawn_10` que pareciam sobras de frame anterior, mantendo os detritos centrais de impacto.

## Geracao

```bash
python3 tools/art/extract_start_animation_atlases.py
```

O script tambem gera uma previa local em `tmp/art/duke-start-atlas-preview.png`, que nao precisa ser versionada.

## Limitacoes

- A fonte ainda e uma imagem composta, nao um arquivo autorado em camadas.
- Mesa e xicara corrigidas sao patches temporarios ate existir export artistico com camadas.
- O clip `spawn` nao deve carregar regra de combate; ele apenas bloqueia gameplay ate a intro terminar.
