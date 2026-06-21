# 11 — Pipeline de Sprites

## Status

Proposto. O runtime atual ainda nao carrega manifests de sprite.

## Objetivo

Permitir que artistas entreguem sprites com dados suficientes para o jogo renderizar animacoes sem hardcode de frame no codigo.

## Formato candidato

O formato inicial do projeto e `borrow-fighters.sprite.v1`, em JSON.

Exemplo real:

- `assets/placeholder/rust-fighter.sprite.json`

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

## Aseprite e ferramentas externas

Se os artistas usarem Aseprite, o caminho ideal e exportar PNG + JSON do Aseprite e converter para `borrow-fighters.sprite.v1`, ou adaptar o motor para aceitar Aseprite JSON diretamente.

Por enquanto, preferimos um formato pequeno do projeto porque:

- facilita revisar no Git;
- evita depender de uma ferramenta especifica;
- deixa claro quais dados o jogo precisa;
- permite gerar metadata a partir de scripts locais.

## Proxima etapa no codigo

Criar um carregador em `engine/sprites.rs` que:

1. leia o manifest JSON;
2. carregue o PNG indicado;
3. selecione clips por estado de gameplay;
4. avance frames por tempo;
5. renderize usando pivot em vez de centralizar a textura.

Isso deve substituir gradualmente o hardcode atual de `FRAME_WIDTH`, `FRAME_HEIGHT` e `FighterSpriteFrame::index`.
