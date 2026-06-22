# 17 — Escala Visual, Personagens e Arena

## Status

Padrao de prototipo. Deve ser revisado quando tivermos arte final, camera com zoom ou corpos fisicos por personagem.

## Objetivo

Evitar que sprites novos fiquem grandes, pequenos ou desalinhados em relacao ao combate. O tamanho do arquivo PNG pode variar; o tamanho em jogo deve vir do manifesto e das metricas de gameplay.

## Referencias

- M.U.G.E.N separa coordenadas de palco (`localcoord`), linha de chao (`zoffset`) e limites de camera/personagem (`boundleft`, `boundright`, `screenleft`, `screenright`): <https://www.elecbyte.com/mugendocs/bgs.html>.
- M.U.G.E.N tambem permite ajustar largura/altura do personagem com `xscale`/`yscale`, evitando redimensionar todos os sprites manualmente: <https://www.elecbyte.com/mugendocs/cns.html>.
- O loop de telas usa uma maquina de estados simples, alinhada ao padrao de FSM descrito em Game Programming Patterns: <https://gameprogrammingpatterns.com/state.html>.

Essas referencias nao ditam nossos numeros finais, mas confirmam a abordagem: coordenada de jogo, escala visual, chao, camera e dados de personagem precisam ser separados.

## Numeros Atuais

Runtime:

- janela: `960 x 540`;
- chao: `FLOOR_Y = 462`;
- arena jogavel: `ARENA_LEFT = 32`, `ARENA_RIGHT = 928`;
- largura jogavel: `896 px`;
- corpo fisico padrao em pe: `76 x 168 px`.

Isso da uma arena com cerca de `11,8` larguras de corpo (`896 / 76`). Para o prototipo, o alvo e manter a largura jogavel entre `10` e `13` corpos padrao. Se caminhar de um canto ao outro parecer curto demais, a arena esta pequena; se o contato demorar demais para acontecer, a arena esta grande.

## Tamanho Visual Alvo

O corpo fisico ainda e um retangulo comum para todos os personagens. O sprite pode ultrapassar esse corpo para cabelo, orelha, roupa, efeito e leitura visual, mas a area vulneravel principal deve continuar coerente com a hurtbox.

Para personagens humanoides, como Rust e Duke/Java:

- altura visivel em idle: `185` a `210 px`;
- largura visivel em idle: `110` a `150 px`;
- referencia aprovada: Rust atual aparece com cerca de `123 x 206 px` em idle;
- Java/Duke atual aparece com cerca de `121 x 188 px` em idle.

Para personagens baixos/largos ou nao-humanos, como Go/Gopher:

- altura visivel em idle: `150` a `185 px`;
- largura visivel em idle: ate `180 px` enquanto a hurtbox continuar legivel;
- se a largura passar muito disso, o personagem deve ganhar corpo fisico proprio ou a arte precisa ser redesenhada mais estreita.

Golpes, armas, projeteis e VFX podem passar desses limites. O que nao pode acontecer e o corpo base do personagem parecer em uma escala diferente da gameplay.

## Regra de Aceite Para Sprites

Um sprite candidato deve passar por estes checks antes de entrar como runtime:

- o pe ou ponto de apoio de idle/walk/golpes no chao deve variar no maximo alguns pixels entre frames;
- `pivot` deve cair no ponto de apoio do personagem, normalmente no chao entre os pes;
- `scale` deve deixar o corpo visivel dentro da faixa do tipo de personagem;
- `hurtbox` deve cobrir cabeca/torso/pernas que parecem vulneraveis, sem ficar muito menor que o desenho;
- `hitbox` deve cobrir a parte ativa do golpe, nao o frame inteiro;
- `projectile_origin` deve ficar visualmente perto da mao, arma ou emissor do poder;
- a arena deve continuar com leitura de distancia: aproximar, afastar, pular e punir whiff precisam caber na tela.

## Manifesto Como Fonte De Verdade

O schema `borrow-fighters.sprite.v1` usa:

- `scale`: escala visual runtime do atlas;
- `frames[].pivot`: ponto local de apoio por frame;
- `frames[].combat.hurtboxes[]`: metadata visual opcional para hurtbox por frame;
- `frames[].combat.hitboxes[]`: metadata visual opcional para hitbox por frame;
- `frames[].combat.projectile_origin`: ponto local opcional de origem de projectile.

O renderer de luta e o Sprite Combat Viewer consomem o mesmo `scale` e o mesmo `pivot`. Portanto, ajuste salvo no viewer aparece no jogo sem recompilar.

O corpo fisico de personagem usa outro manifesto:

- arquivo: [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json);
- schema: `borrow-fighters.character-body-metrics.v1`;
- campos por personagem: `width`, `standing_height`, `crouch_height`.

Esse manifesto alimenta a luta normal e o Sprite Combat Viewer. Ele deve ser usado quando a silhueta visual exige corpo diferente, como Go/Gopher. Hitboxes de golpes e origem de projectile continuam em dados de golpe ou `frames[].combat`; corpo fisico nao deve virar substituto para alcance de ataque.

Fluxo recomendado:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/go-fighter.sprite.json --clip idle --character go --move light_punch
```

No viewer:

- `=` aumenta `scale` do manifesto;
- `-` diminui `scale` do manifesto;
- `Setas` movem o `pivot` do frame atual em 1 px;
- `Shift+Setas` movem o `pivot` em 8 px;
- `Ctrl+Setas` ajustam `width` e `standing_height` do corpo fisico;
- `Ctrl+Shift+Setas` ajustam `crouch_height`;
- `Ctrl+S` salva os manifestos alterados;
- `F5` recarrega manifesto e atlas;
- mouse sobre o sprite mostra coordenada local/atlas para preencher dados de box.

## Correcoes Aplicadas No Go

O Go ficou grande visualmente porque o Gopher e muito mais largo que o corpo humanoide usado pelo greybox. A correcao atual reduziu `assets/placeholder/go-fighter.sprite.json` para `scale = 0.88` e definiu o corpo fisico do Go em `assets/tuning/character-body-metrics.json` como `width = 92`, `standing_height = 156` e `crouch_height = 88`.

Rust e Duke/Java foram normalizados para `scale = 1.0` porque esse era o tamanho efetivo que o jogo ja mostrava antes do renderer passar a respeitar `scale` do manifesto.

## Ainda Em Aberto

- Criar alcas visuais de hitbox/hurtbox no Sprite Combat Viewer, alem do ajuste atual de `scale` e `pivot`.
- Definir se metricas de arena entram em um manifesto proprio ou continuam em `src/config.rs` ate a camera evoluir.
- Validar a faixa de altura/largura em playtest, nao apenas por comparacao visual.
