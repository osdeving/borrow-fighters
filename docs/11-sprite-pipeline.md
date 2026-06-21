# 11 — Pipeline de Sprites

## Status

Em implementacao. O runtime ja possui um primeiro carregador de manifest para
testar atlas, clips, duracoes e pivots no personagem Rust.

## Objetivo

Permitir que artistas entreguem sprites com dados suficientes para o jogo renderizar animacoes sem hardcode de frame no codigo.

## Formato candidato

O formato inicial do projeto e `borrow-fighters.sprite.v1`, em JSON.

Exemplo real:

- `assets/placeholder/rust-fighter.sprite.json`
- `assets/placeholder/duke-fighter.sprite.json`
- `assets/placeholder/rust-start.sprite.json`
- `assets/placeholder/duke-start.sprite.json`

Campos principais:

- `schema`: versao do formato.
- `image`: PNG do atlas.
- `source`: imagem ou arquivo de origem.
- `cell`: tamanho padrao da celula.
- `default_pivot`: ponto de apoio padrao, normalmente perto do pe no chao.
- `frames`: retangulos no atlas, duracao e pivot por frame.
- `clips`: animacoes com lista ordenada de frames e flag `loop`.

## Convencoes

- O atlas runtime deve ter alpha real.
- Labels, guias, checkerboard e anotacoes nao entram no PNG runtime.
- Todo frame deve caber dentro do retangulo declarado.
- Todo frame deve ter pivot.
- Animacoes de ataque devem ter duracao por frame.
- Efeitos reutilizaveis, como projeteis, devem poder virar assets separados.

## Clips recomendados

- `spawn`
- `idle`
- `walk`
- `crouch`
- `jump`
- `block`
- `punch_light`
- `punch_heavy`
- `kick`
- `hit`
- `special`

Clips extras como `taunt`, `victory`, `defeat` e `projectile` podem existir, mas nao devem bloquear o prototipo.

`spawn` e reservado para entrada cinematografica no inicio da luta. Ele deve ser nao-loopavel e nao deve carregar regra de combate; o jogo apenas pausa os inputs ate a intro terminar.

## Aseprite e ferramentas externas

Se os artistas usarem Aseprite, o caminho ideal e exportar PNG + JSON do Aseprite e converter para `borrow-fighters.sprite.v1`, ou adaptar o motor para aceitar Aseprite JSON diretamente.

Por enquanto, preferimos um formato pequeno do projeto porque:

- facilita revisar no Git;
- evita depender de uma ferramenta especifica;
- deixa claro quais dados o jogo precisa;
- permite gerar metadata a partir de scripts locais.

## Implementacao atual

O primeiro corte vive em `src/engine/sprites/`:

1. `manifest.rs` le e valida JSON;
2. `animation.rs` seleciona frames por duracao;
3. `selection.rs` converte estado de lutador em clip;
4. `draw.rs` desenha atlas com pivot via Raylib.

O personagem Rust ja pode usar `assets/placeholder/rust-fighter.sprite.json`.
O Player 2/Duke ja pode usar `assets/placeholder/duke-fighter.sprite.json`.

O runtime tambem usa:

- `spawn` durante a entrada inicial de Rust e Duke;
- `special` por alguns frames quando o personagem dispara projectile;
- `taunt` quando o personagem vence a luta;
- fallback greybox quando um atlas nao carrega.

As animacoes de entrada atuais vivem em manifests separados para nao misturar frames cinematograficos grandes com o atlas principal de luta:

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/duke-start-atlas.png`
