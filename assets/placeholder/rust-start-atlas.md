# Rust Start Atlas

## Status

Placeholder candidato extraido de `assets/references/rust-start-anim.png`.

## Arquivos

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/rust-start.sprite.json`
- `assets/references/rust-start-anim.png`

## Uso

Atlas cinematografico usado no clip `spawn`, antes de liberar os controles no inicio da luta.

O script remove do frame `spawn_13` o notebook isolado que parecia sobra de frame anterior e remove da sequencia o frame fonte em que o notebook isolado nao funcionava como pose de personagem.

## Geracao

```bash
python3 tools/art/extract_start_animation_atlases.py
```

O script tambem gera uma previa local em `tmp/art/rust-start-atlas-preview.png`, que nao precisa ser versionada.

## Limitacoes

- A fonte ainda e uma imagem composta, nao um arquivo autorado em camadas.
- O notebook isolado do frame `spawn_13` e removido durante a extracao para preservar continuidade.
- O clip `spawn` nao deve carregar regra de combate; ele apenas bloqueia gameplay ate a intro terminar.
