# ADR 0006 — Escala Runtime de Sprites e Estado de Cenas

## Status

Aceita para o Prototype 0.1.

## Contexto

Os sprites de Rust, Duke/Java e Go vieram de origens diferentes e com proporcoes diferentes. O Go, por ser um Gopher largo, ficou visualmente maior que a gameplay mesmo usando o mesmo corpo fisico do greybox.

Antes desta decisao, o Sprite Combat Viewer lia `scale` do manifesto, mas o renderer da luta desenhava todos os atlas com escala `1.0`. Isso criava uma divergencia: o artista ajustava no viewer, mas o jogo nao refletia o mesmo dado.

O loop principal tambem estava organizado por condicionais de cena. Como o projeto esta ganhando menu, luta, laboratorios e ferramentas, precisamos manter transicoes explicitas.

## Decisao

1. `borrow-fighters.sprite.v1` passa a tratar `scale` como escala visual runtime do atlas.
2. `frames[].pivot` continua sendo o ponto local de apoio por frame e pode ser ajustado no Sprite Combat Viewer.
3. O renderer de luta usa o mesmo `scale` e `pivot` do manifesto que o Sprite Combat Viewer usa.
4. Corpo fisico base de personagem passa a vir de [`assets/tuning/character-body-metrics.json`](../../assets/tuning/character-body-metrics.json), com fallback nos defaults do `CharacterSpec`.
5. O Sprite Combat Viewer permite ajustar `scale`, mover o `pivot` do frame atual, ajustar `width`/`standing_height`/`crouch_height` do personagem e salvar os manifestos sob comando explicito.
6. A escala padrao aprovada no prototipo fica documentada em [`docs/17-visual-scale-and-stage-metrics.md`](../17-visual-scale-and-stage-metrics.md).
7. O loop principal usa `AppScene` como maquina de estados explicita para `Preferences`, `Fight` e `CombatLab`; `SpriteViewer` permanece em loop isolado de ferramenta.

## Consequencias

- O tamanho do PNG pode variar sem redefinir a escala em jogo.
- Ajustes de escala/pivot podem ser revisados em PR como JSON.
- Ajustes de corpo fisico tambem podem ser revisados em PR como JSON.
- O viewer e a luta passam a concordar sobre o tamanho visual do personagem.
- Sprites muito largos, como Go/Gopher, podem usar corpo fisico proprio sem alterar todo o codigo de `Fighter`.
- Hitbox/hurtbox por frame ainda nao sao editadas visualmente por alcas; por enquanto, o viewer mostra dados e permite preencher coordenadas locais.
- Mudancas futuras de camera/arena devem preservar a separacao entre coordenada de mundo, escala visual e dados de personagem.

## Referencias

- M.U.G.E.N Stage Backgrounds: `localcoord`, `zoffset`, bounds e screen bounds — <https://www.elecbyte.com/mugendocs/bgs.html>
- M.U.G.E.N CNS: `xscale`/`yscale` para escala de personagem — <https://www.elecbyte.com/mugendocs/cns.html>
- Game Programming Patterns: State — <https://gameprogrammingpatterns.com/state.html>
