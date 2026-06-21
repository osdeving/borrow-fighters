# Rust Fighter Sprite Reference

## Status

Referencia visual fornecida para exploracao. Nao e arte final validada.

## Arquivo

- `assets/references/sprinte-rust.png`

## Uso

Imagem de referencia com multiplas poses do personagem Rust. Ela nao deve ser usada diretamente no runtime porque:

- o fundo xadrez esta pintado no RGB, nao em alpha real;
- os labels fazem parte da imagem;
- os frames nao estao em uma grade regular;
- pivots e baselines precisam ser normalizados.

O asset runtime derivado fica em:

- `assets/placeholder/rust-fighter-atlas.png`
- `assets/placeholder/rust-fighter.sprite.json`

## Fonte

Fornecida localmente durante a exploracao de direcao de arte.
