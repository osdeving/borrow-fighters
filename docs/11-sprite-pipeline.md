# 11 — Pipeline de Sprites

## Status

Em implementacao. O runtime ja carrega manifests para Rust, Duke, animacoes de entrada, clips de luta, pivots, duracoes por frame e fallback greybox.

## Objetivo

Permitir que artistas entreguem sprites com dados suficientes para o jogo renderizar animacoes sem hardcode de frame no codigo, mantendo pivots e hurtboxes ajustaveis quando a arte exigir.

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

`spawn` e reservado para entrada cinematografica no inicio da luta. Ele deve ser nao-loopavel e nao deve carregar regra de combate; o jogo pausa os inputs durante a intro e depois durante a contagem `11`, `10`, `01`, `Fight!`.

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

O personagem Rust usa `assets/placeholder/rust-fighter.sprite.json`.
O Player 2/Duke usa `assets/placeholder/duke-fighter.sprite.json`.

O runtime tambem usa:

- `spawn` durante a entrada inicial de Rust e Duke;
- `special` por alguns frames quando o personagem dispara projectile;
- `taunt` quando o personagem vence a luta;
- fallback greybox quando um atlas nao carrega.

As animacoes de entrada atuais vivem em manifests separados para nao misturar frames cinematograficos grandes com o atlas principal de luta:

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/duke-start-atlas.png`

Assets relacionados ao slice atual:

- `assets/placeholder/rust-gear-projectile.png`
- `assets/placeholder/duke-bean-projectile.png`
- `assets/placeholder/arena-sirius.png`
- `assets/placeholder/arena-fortaleza.png`
- `assets/placeholder/arena-java-street.png`
- `assets/placeholder/arena-terminal-compiler-lab.png`

As ferramentas locais ficam em `tools/art/` e devem ser tratadas como utilitarios de prototipo, nao como pipeline final.

## Pontos ainda em aberto

- Definir se o formato v1 vira padrao permanente ou ponte para Aseprite JSON.
- Mover hurtbox/hitbox por frame para dados externos quando a arte estabilizar.
- Definir escala base dos personagens e tamanho minimo legivel para tela 16:9.
- Criar criterio visual para aceitar atlas de personagem como "candidato" em vez de placeholder.
