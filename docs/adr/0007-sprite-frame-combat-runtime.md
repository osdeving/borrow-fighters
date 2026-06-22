# ADR 0007 — Metadata de Combate por Frame no Runtime

## Status

Proposto.

## Contexto

O protótipo começou com hitboxes e hurtboxes greybox calculadas por código. Isso foi suficiente para validar golpes, dano e defesa, mas começou a limitar o encaixe de sprites maiores e assimétricos.

O Sprite Combat Viewer já permite editar `frames[].combat` em coordenadas locais do atlas. Faltava decidir quando esses dados deixam de ser apenas anotação visual e passam a participar da luta.

## Decisão

Usar `frames[].combat` como override incremental de colisão no runtime:

- `frames[].combat.hitboxes[]` substitui a hitbox ofensiva calculada por `MoveSpec` quando a lista existe no frame atual;
- `frames[].combat.hurtboxes[]` substitui as hurtboxes compostas de `Fighter` quando a lista existe no frame atual;
- `frames[].combat.projectile_origin` no clip `special` pode definir a origem do projectile;
- campos ausentes ou listas vazias mantêm fallback para `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`.

A projeção de coordenadas locais do frame para coordenadas de mundo fica em `src/engine/sprites/combat.rs`, porque depende de sprite manifest, clip, pivot, escala e facing. A resolução de luta permanece em `src/game/world.rs`.

## Alternativas consideradas

### Continuar só com greybox

Mais simples e estável, mas mantém desalinhamento entre arte e colisão quando os personagens deixam de ser retângulos pequenos.

### Migrar todo combate para dados externos agora

Daria flexibilidade máxima, mas criaria uma engine de dados antes de termos sprites finais e playtest suficiente.

### Arquivo lateral de combate por personagem

Pode ser melhor no futuro, especialmente se o manifesto de sprite ficar grande demais. Por enquanto, manter o dado no frame reduz navegação e deixa a revisão visual mais direta.

## Consequências

### Positivas

- Artistas e devs conseguem alinhar colisão ao frame visual sem recompilar.
- O jogo continua funcionando quando um sprite não tem metadata.
- O debug visual da luta pode mostrar a mesma box usada pela resolução.
- `projectile_origin` reduz desalinhamento entre mão do sprite e projectile.

### Negativas

- O manifesto de sprite passa a carregar dado de gameplay experimental.
- Boxes ruins no JSON podem afetar balanceamento imediatamente.
- Ainda precisamos de critério de revisão para aceitar metadata por personagem.

## Critério de revisão

Revisar se:

- os manifests ficarem difíceis de revisar no Git;
- o time precisar versionar boxes separadas por personagem/move set;
- o Combat Lab precisar de playback por `MoveId` mais forte que a seleção atual por clip;
- balanceamento começar a exigir dados por golpe que não pertencem ao atlas.
