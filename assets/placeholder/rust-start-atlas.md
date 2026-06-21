# Rust Start Atlas

## Status

Placeholder candidato extraido de `assets/references/rust-start-anim.png`.

## Arquivos

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/rust-start.sprite.json`
- `assets/references/rust-start-anim.png`

## Uso

Atlas cinematografico usado no clip `spawn`, antes de liberar os controles no inicio da luta.

O script corrige a continuidade do notebook no frame `spawn_13` e remove da sequencia o frame fonte em que o notebook isolado nao funcionava como pose de personagem.

## Geracao

```bash
python3 tools/art/extract_start_animation_atlases.py
```

O script tambem gera uma previa local em `tmp/art/rust-start-atlas-preview.png`, que nao precisa ser versionada.

## Limitacoes

- A fonte ainda e uma imagem composta, nao um arquivo autorado em camadas.
- O notebook corrigido e aplicado como patch de celula durante a extracao.
- O clip `spawn` nao deve carregar regra de combate; ele apenas bloqueia gameplay ate a intro terminar.
