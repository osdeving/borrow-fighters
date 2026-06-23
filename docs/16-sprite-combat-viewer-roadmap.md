# 16 — Roadmap do Sprite Combat Viewer

## Status

Implementado como ferramenta Raylib temporaria. O sucessor e o Sprite Studio externo em Tauri + React, documentado em [`docs/18-sprite-studio.md`](18-sprite-studio.md) e decidido em [`docs/adr/0008-external-sprite-studio-tooling.md`](adr/0008-external-sprite-studio-tooling.md).

O Sprite Studio ja possui paridade operacional para abrir, editar, validar e exportar review de manifestos. O viewer Raylib deve ser removido em uma mudanca propria.

Issue de rastreio: [#15](https://github.com/osdeving/borrow-fighters/issues/15).

## Objetivo

Criar uma ferramenta leve para artistas e devs verificarem sprites, pivots, alinhamento visual, escala e, nas proximas fases, hitbox, hurtbox, origem de projectile e dummy de contato sem entrar no fluxo normal de luta.

O viewer nao deve ser tratado como editor final. Ele foi uma ferramenta plugavel para reduzir tentativa e erro ao criar atlas e ajustar manifestos. A direcao atual e mover a experiencia de edicao rica para o Sprite Studio externo.

## Por Que Nao Basta o Combat Lab

O Combat Lab valida golpes dentro de uma cena de combate. Ele e bom para frame data, vantagem, pushback e dummy de contato, mas nao resolve bem o trabalho do artista quando a pergunta e:

- o pivot do frame esta no lugar correto?
- o personagem cabe dentro do retangulo declarado?
- o atlas carrega em runtime sem recompilar?
- a escala do personagem faz sentido contra a grade?
- o frame atual tem sujeira, corte ruim ou continuidade quebrada?
- o hit/hurt box futuro precisa ser ajustado por frame?

O Sprite Combat Viewer fica um nivel antes: ele inspeciona o asset e seus dados.

## Pesquisa de GUI

Raylib puro foi suficiente para grade, linhas, mouse drag, texto e atalhos de teclado. A partir do momento em que a ferramenta passou a precisar de painel lateral, checkboxes, sliders, listas, inputs editaveis e fluxo de artista, a decisao mudou para Tauri + React.

Decisao atual:

- manter Raylib viewer apenas ate a branch de remocao dedicada;
- evoluir UI rica no Sprite Studio;
- nao introduzir `raygui`, `egui` ou `imgui` dentro do jogo neste momento;
- remover o viewer Raylib quando o Sprite Studio conseguir salvar manifestos aceitos pelo jogo.

Referencias:

- [raygui](https://github.com/raysan5/raygui): GUI immediate-mode para raylib.
- [`raylib` crate](https://crates.io/crates/raylib): binding Rust usado pelo projeto; a versao 6.0 possui feature `raygui`.

## Corte Implementado Agora

Comando:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
```

Responsabilidades atuais:

- carregar manifesto `borrow-fighters.sprite.v1` em runtime;
- resolver o PNG do atlas relativo ao manifesto;
- abrir uma cena isolada do jogo normal;
- mostrar grade de tela inteira e linha de chao;
- desenhar o frame atual do atlas com pivot e escala do manifesto;
- exibir retangulo do frame, `trimmed_bounds` e `source_crop` quando existirem;
- permitir arrastar o personagem com mouse para testar encaixe;
- permitir arrastar um dummy espelhado para comparar escala, distancia e continuidade contra um oponente;
- aplicar zoom visual com mouse wheel sem alterar o manifesto;
- ajustar `scale` do manifesto com teclado;
- mover `pivot` do frame atual com teclado;
- ajustar `width`, `standing_height` e `crouch_height` do personagem selecionado;
- salvar manifestos de tuning sob comando explicito;
- recarregar manifesto e atlas com `F5`;
- salvar screenshot de review em `target/sprite-viewer-capture.png` com `F12`;
- aceitar `--character` e `--move` para desenhar hurtbox atual, hitbox do golpe e origem/caixa de projectile com os dados de combate existentes;
- navegar clips e frames;
- mostrar path do manifesto, path da imagem, frame atual, pivot, anchor e escala.

Atalhos:

| Acao | Tecla |
|---|---|
| Arrastar personagem | Mouse esquerdo |
| Proximo clip | `Tab` |
| Clip anterior | `Shift+Tab` |
| Proximo frame | `.` |
| Frame anterior | `,` |
| Pausar/continuar | `Espaco` |
| Zoom | Mouse wheel |
| Resetar zoom | `0` |
| Aumentar `scale` do manifesto | `=` |
| Diminuir `scale` do manifesto | `-` |
| Mover `pivot` do frame atual | `Setas` |
| Mover `pivot` em passos maiores | `Shift+Setas` |
| Ajustar largura/altura do corpo fisico | `Ctrl+Setas` |
| Ajustar altura abaixada do corpo fisico | `Ctrl+Shift+Setas` |
| Gerar rascunho de `frames[].combat` pelo overlay runtime | `N` |
| Adicionar hurtbox no frame atual | `H` |
| Adicionar hitbox no frame atual | `J` |
| Remover box/origem sob o mouse ou ultimo item | `Delete` |
| Mover hurtbox/hitbox/origem de projectile do frame | Mouse esquerdo nas boxes/alcas |
| Redimensionar hurtbox/hitbox do frame | Mouse esquerdo nos cantos da box |
| Salvar manifestos de tuning | `Ctrl+S` |
| Mostrar/esconder dummy | `O` |
| Mostrar/esconder boxes de combate | `M` |
| Mostrar/esconder trajetoria de projectile | `T` |
| Recarregar manifesto e atlas | `F5` |
| Salvar screenshot | `F12` |
| Alternar grade | `G` |
| Alternar pivot | `P` |
| Alternar bounds | `B` |
| Resetar posicao | `R` |

Codigo principal:

- [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs): estado testavel, manifesto, frame atual, playback e drag.
- [`src/scenes/sprite_viewer/combat_edit.rs`](../src/scenes/sprite_viewer/combat_edit.rs): helpers puros para criar, redimensionar e limpar boxes de combate do frame.
- [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs): desenho Raylib da ferramenta e feedback de manifesto sujo.
- [`src/cli.rs`](../src/cli.rs): modo `--tool sprite-viewer`.
- [`src/app.rs`](../src/app.rs): desvio para o loop isolado da ferramenta.
- [`tests/sprite_viewer.rs`](../tests/sprite_viewer.rs): testes do estado sem abrir janela.

## Roadmap

### Fase 1 — Viewer de Sprite

Status: em andamento.

Entregas:

- abrir manifesto e atlas em runtime;
- grid, chao, pivot e bounds;
- drag do personagem e dummy com mouse;
- dummy espelhado com distancia entre anchors;
- zoom visual por mouse wheel;
- hot reload manual de manifesto e atlas;
- screenshot de review;
- navegacao de clips/frames;
- testes de estado.

Falta:

- legenda visual mais clara para cada guia.
- pan de camera quando o personagem ficar maior que a tela;
- UI clicavel se a lista de comandos ficar grande demais.

### Fase 2 — Viewer de Combate

Objetivo: enxergar boxes reais, nao apenas bounds de sprite.

Status: em andamento. O viewer ja projeta a hurtbox atual do corpo, a hitbox do golpe selecionado e a caixa/origem de projectile usando `CharacterSpec`, `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`. Ele tambem desenha metadata opcional `frames[].combat` quando o manifesto declara hurtbox, hitbox ou origem de projectile por frame, permite alternar personagem/golpe sem reiniciar, mostra uma trajetoria prevista simples de projectile e exibe coordenada local/atlas do mouse sobre o frame atual.

Entregas planejadas:

- carregar dois personagens reais na mesma tela;
- mover personagem e dummy com mouse;
- alternar personagem/clip/golpe;
- desenhar hurtbox e hitbox reais do golpe selecionado por frame;
- mostrar origem de projectile e trajetoria prevista completa;
- disparar golpe em step frame, sem depender da luta completa.

Dependencia tecnica:

- validar se `frames[].combat` no manifesto de sprite e suficiente ou se os dados finais de combate precisam migrar para arquivo lateral de personagem.

### Fase 3 — Ajuste Data-Driven

Objetivo: permitir que arte e combate sejam ajustados por dados revisaveis no Git.

Entregas planejadas:

- schema opcional de `hurtboxes`, `hitboxes` e `projectile_origin` por frame;
- validacao de manifesto para boxes fora do frame;
- import/export manual de offsets;
- checklist visual de aceitacao de atlas.

Ja existe:

- ajuste e persistencia de `scale` do manifesto com `=`/`-` e `Ctrl+S`;
- ajuste e persistencia de `frames[].pivot` do frame atual com `Setas`/`Shift+Setas` e `Ctrl+S`;
- ajuste e persistencia de corpo fisico por personagem em [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json);
- `frames[].combat.hurtboxes[]`, `frames[].combat.hitboxes[]` e `frames[].combat.projectile_origin` no schema `borrow-fighters.sprite.v1`;
- origem de projectile calibrada no primeiro frame do clip `special` de Rust, Duke, Go e C;
- hitboxes iniciais do Rust `Borrow Jab`, heavy punch e kick, ainda equivalentes ao alcance greybox atual;
- validacao em [`src/engine/sprites/manifest.rs`](../src/engine/sprites/manifest.rs);
- projecao testavel em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs);
- desenho no viewer em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs);
- timeline inferior com coloracao aproximada por fase de golpe.
- alternancia runtime de personagem/golpe com `C`/`Shift+C` e `[`/`]`;
- preview simples de trajetoria de projectile com `T`.
- inspetor de coordenada local/atlas do cursor para preencher `frames[].combat`;
- geracao de rascunho de `frames[].combat` com `N`, usando o overlay runtime do golpe selecionado;
- criacao de hurtbox/hitbox no frame atual com `H`/`J`;
- remocao de box/origem com `Delete`;
- alcas visuais para mover e redimensionar `frames[].combat.hurtboxes[]` e `frames[].combat.hitboxes[]`;
- alca visual para mover `frames[].combat.projectile_origin`;
- sincronizacao manual entre golpe e clip visual com `Enter`.

Regra importante: a ferramenta so salva manifestos por comando explicito (`Ctrl+S`). Escala, pivot, corpo fisico e metadata visual de hitbox/hurtbox/origem ja sao editaveis. A luta real ja consome `frames[].combat` quando houver hitboxes, hurtboxes ou origem de projectile no frame, com fallback para os dados greybox atuais.

### Fase 4 — Produtividade Para Artistas

Entregas planejadas:

- painel com controles clicaveis se `raygui` for adotado;
- preset de escala por personagem;
- overlay comparativo contra greybox de altura alvo;
- marca de chao, centro, alcance e margem segura;
- refinamento de selecao para multiplas boxes sobrepostas;
- comandos para renomear boxes sem editar JSON manualmente;
- manifesto ou config data-driven para metricas de arena, se a camera deixar de ser fixa;
- export de screenshot para PR/review de arte.

### Fase 5 — Integracao Com Gameplay

Entregas planejadas:

- abrir diretamente um `MoveId`/golpe;
- reproduzir startup/active/recovery com frame data real;
- comparar visual do sprite contra hitbox/hurtbox de cada frame;
- salvar notas tecnicas para a issue/PR.

Ja existe:

- [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs) projeta `frames[].combat` para mundo com `scale`, `pivot` e `Facing`;
- [`src/game/world.rs`](../src/game/world.rs) usa hitboxes/hurtboxes de sprite quando presentes, com fallback para `MoveSpec` e `Fighter::hurtboxes`;
- `projectile_origin` do clip `special` define de onde o projectile nasce quando o frame possui esse dado;
- debug visual da luta usa as mesmas boxes projetadas quando `Mostrar debug de combate` esta ligado.

## Decisoes Em Aberto

- A edicao visual de boxes deve acontecer no manifesto de sprite ou em arquivo lateral por personagem?
- O Combat Lab deve reaproveitar o mesmo estado do viewer ou apenas os mesmos dados?
- Go e C ja possuem atlas placeholder; falta revisar boxes por frame e criterios de aceite visual.
- `raygui` entra como feature opcional ou evitamos dependency feature ate a ferramenta exigir?

## Criterios de Aceite Para Contribuidores

Um atlas candidato deve:

- carregar sem erro pelo Sprite Combat Viewer;
- manter pivot consistente entre idle, crouch, jump e golpes;
- nao cortar partes importantes do personagem dentro do frame declarado;
- manter o pe/anchor estavel quando a pose nao implica deslocamento;
- indicar visualmente de onde sairia projectile ou golpe ativo, mesmo antes do dado final existir;
- vir acompanhado do manifesto JSON e, quando possivel, screenshot do viewer no PR.
