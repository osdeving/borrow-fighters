# ADR 0015 — Apresentação cinematográfica e vida dos cenários

## Status

Aceito para a expansão e o polimento solicitados pelo usuário.

## Contexto

O slice possui fontes dependentes do host, cenários bitmap e especiais físicos
de assinatura. A nova rodada exige consistência visual, atores de cenário e um
segundo especial por personagem que ocupe visualmente a tela sem ampliar o dano.

## Decisão

Incorporar fontes com licença e atlas de alta resolução filtrados na fronteira
Raylib. Menus continuam usando `MenuLayout` compartilhado por desenho e input.

Adicionar `CinematicSpecial` ao sistema de golpes existente, com seis `MoveSpec`
e input próprio. Um snapshot derivado do ataque expõe identidade e relógio ao
renderer. O combate usa exclusivamente a geometria local do `MoveSpec`; atlas de
ator e efeitos visuais não substituem essas boxes. Cancelamento remove o snapshot
automaticamente. O renderer suprime a cinematográfica ao encerrar a partida.

Compor VFX de tela inteira usando geometria animada de programação junto às poses
existentes. Simulação e showcase compartilham relógio, pausa e frame-step.

Carregar dois atlas opcionais de cenário, com alpha e retângulos explícitos quando
necessário. Um módulo de apresentação desenha atores e detalhes por arena a partir
do relógio visual; não cria entidades de combate ou aleatoriedade na simulação.
A flag `ShowStageLife` controla essa camada. O fundo revisado do Sirius retira o
animal estático anterior; sua fonte histórica fica preservada.

## Consequências

- A apresentação ganha movimento mantendo o núcleo de combate testável.
- Fontes e arte passam a acompanhar o projeto, com procedência documentada.
- Reaproveitar poses reduz produção sem impedir futuras animações específicas.
- Não são necessários ECS, scripts, pipeline novo ou framework de interface.

Veja [plano, direção e critérios](../22-presentation-and-brazilian-stage-life.md).
