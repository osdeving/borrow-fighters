# 12 — Guia Técnico de Combate

## Objetivo

Este documento ajuda devs e agentes de IA a encontrar rapidamente onde o combate vive no código, como testar golpes isolados e quais docs atualizar quando o sistema mudar.

Sempre que um código novo alterar combate, personagens, input de combate, Combat Lab, hitbox/hurtbox, projectile, frame data ou sprites ligados a golpes, atualize este guia ou explique no PR por que não foi necessário.

## Sistemas

| Sistema | Responsabilidade | Código principal | Testes |
|---|---|---|---|
| Combat runtime | Estado de lutador, movimento, defesa, ataque ativo, stun, dano e hurtbox | [`src/combat/fighter.rs`](../src/combat/fighter.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs), [`tests/traditional_moves.rs`](../tests/traditional_moves.rs) |
| Cinematic presentation data | Go: identidade e relógio do ataque local vivo | [`src/combat/cinematic.rs`](../src/combat/cinematic.rs) | [`tests/cinematic_specials.rs`](../tests/cinematic_specials.rs) |
| Authored super sequence | Rust/Duke/C/C++/Python: captura, fases, contatos e restauração | [`src/combat/super_sequence.rs`](../src/combat/super_sequence.rs), [`src/game/world/supers.rs`](../src/game/world/supers.rs) | [`tests/authored_super_sequences.rs`](../tests/authored_super_sequences.rs) |
| Combat data | Frame data, dano, guard rule, hit reaction e hitbox dos golpes próximos | [`src/combat/move_data.rs`](../src/combat/move_data.rs) | [`tests/move_data.rs`](../tests/move_data.rs), [`tests/traditional_moves.rs`](../tests/traditional_moves.rs) |
| Move runtime | Enum runtime `AttackKind` e compatibilidade com `MoveSpec` | [`src/combat/move_set.rs`](../src/combat/move_set.rs) | [`tests/move_data.rs`](../tests/move_data.rs) |
| Projectile | Projétil horizontal, dano, guard rule, hit reaction, velocidade, spawn e timing do especial | [`src/combat/projectile.rs`](../src/combat/projectile.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs) |
| Collision | Interseção simples de retângulos | [`src/combat/collision.rs`](../src/combat/collision.rs), [`src/math/rect.rs`](../src/math/rect.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Character data | Registro de personagens, listas de golpes e identidade de loadout | [`src/characters/mod.rs`](../src/characters/mod.rs) | [`tests/characters.rs`](../tests/characters.rs), [`tests/character_identity_tuning.rs`](../tests/character_identity_tuning.rs) |
| Character body metrics | Manifesto data-driven para largura, altura em pe e altura abaixada | [`src/characters/body_metrics.rs`](../src/characters/body_metrics.rs), [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json) | [`tests/characters.rs`](../tests/characters.rs) |
| Match runtime | Instancia lutadores a partir de personagens, bloqueia intro/contagem, resolve hits, projéteis e vitória | [`src/game/world.rs`](../src/game/world.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Combat log | Eventos compactos de diagnóstico para reproduzir bugs de luta | [`src/game/combat_log.rs`](../src/game/combat_log.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| CPU playtest | Heurística determinística para mover, defender e exercitar golpes básicos/tradicionais | [`src/game/ai.rs`](../src/game/ai.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/cpu_traditional_moves.rs`](../tests/cpu_traditional_moves.rs) |
| Arena runtime | Identidade, contexto, rotação e seleção manual de arenas do protótipo | [`src/game/arena.rs`](../src/game/arena.rs) | [`tests/arena_rotation.rs`](../tests/arena_rotation.rs) |
| Audio domain | Cues, eventos de gameplay, manifesto JSON e matching de bindings | [`src/audio/mod.rs`](../src/audio/mod.rs) | [`tests/audio_manifest.rs`](../tests/audio_manifest.rs) |
| Audio Raylib boundary | Carrega clips existentes e toca eventos resolvidos por manifesto | [`src/engine/audio.rs`](../src/engine/audio.rs) | Teste manual via jogo |
| Lore data | Livro de história e fichas de roster carregados de JSON | [`src/lore/mod.rs`](../src/lore/mod.rs), [`assets/lore/story.json`](../assets/lore/story.json) | [`tests/lore_book.rs`](../tests/lore_book.rs) |
| App scene state | Maquina de estados de alto nivel para menu, luta e laboratorios | [`src/app.rs`](../src/app.rs), [`src/scenes/mod.rs`](../src/scenes/mod.rs) | Smoke tests via CLI |
| Combat Lab state | Cena isolada para playback de golpes, pause, frame step e leitura de vantagem | [`src/scenes/combat_lab.rs`](../src/scenes/combat_lab.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab analysis | Cálculo de vantagem estimada, pushback e dummy de contato | [`src/scenes/combat_lab_analysis.rs`](../src/scenes/combat_lab_analysis.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab render | Orquestra Raylib da cena isolada, sprites, grid e projéteis | [`src/engine/render/combat_lab.rs`](../src/engine/render/combat_lab.rs) | Teste manual via Combat Lab |
| Move Showcase | Demonstrações contextuais de todos os golpes e defesas contra um adversário no World real | [`src/scenes/move_showcase.rs`](../src/scenes/move_showcase.rs), [`src/engine/render/move_showcase.rs`](../src/engine/render/move_showcase.rs) | [`tests/move_showcase.rs`](../tests/move_showcase.rs), teste manual via `Training -> Move Showcase` |
| Combat debug UI | Boxes, pivot, dummy, overlay e texto de timing do laboratório | [`src/ui/combat_debug.rs`](../src/ui/combat_debug.rs) | Teste manual via Combat Lab |
| Sprite Combat Viewer | Ferramenta isolada para carregar atlas em runtime, ver grid, pivot, bounds e preparar boxes data-driven | [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), [`src/scenes/sprite_viewer/combat_edit.rs`](../src/scenes/sprite_viewer/combat_edit.rs), [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs) | [`tests/sprite_viewer.rs`](../tests/sprite_viewer.rs), teste manual via `--tool sprite-viewer` |
| Sprite Studio | App externo Tauri 1.8 + React para editar manifestos sem depender de Raylib | [`tools/sprite-studio`](../tools/sprite-studio) | `pnpm build`; `pnpm tauri build --debug`; desktop requer pre-requisitos Tauri |
| Input | Teclado/gamepad para luta e ferramentas; mouse, teclado e gamepad para menus | [`src/engine/input.rs`](../src/engine/input.rs), [`src/engine/gamepad.rs`](../src/engine/gamepad.rs) | [`tests/cli.rs`](../tests/cli.rs), [`tests/feature_flags.rs`](../tests/feature_flags.rs) |
| Sprite runtime | Manifest JSON, seleção de clips, relógios visuais e projeção de metadata baseline | [`src/engine/sprites/`](../src/engine/sprites), [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs) | [`tests/sprite_manifest.rs`](../tests/sprite_manifest.rs), [`tests/sprite_selection.rs`](../tests/sprite_selection.rs), [`tests/sprite_playback.rs`](../tests/sprite_playback.rs) |

## Técnica Atual

### Estado de Cenas

O loop principal em [`src/app.rs`](../src/app.rs) usa `AppScene` de [`src/scenes/mod.rs`](../src/scenes/mod.rs) como maquina de estados simples:

- `Preferences`: menu principal e submenus de versus, treino, lore/roster, opções e Como jogar;
- `Fight`: luta normal com fixed timestep, IA, audio events e renderer de arena;
- `CombatLab`: cena isolada para testar golpes e frame data;
- `MoveShowcase`: cena de treino com dois atores, situações contextuais, golpes e defesas reais;
- `SpriteViewer`: ferramenta de sprite em loop proprio, fora do fluxo normal de luta.

Transicoes novas devem passar por esse enum em vez de espalhar flags soltas no loop. Se a nova tela for ferramenta temporaria, prefira loop isolado como o Sprite Viewer; se fizer parte do jogo, trate como cena normal. `Esc` tem comportamento de voltar dentro do jogo; o bootstrap em [`src/main.rs`](../src/main.rs) desativa a tecla padrão de fechamento do Raylib com `set_exit_key(None)`.

### Mouse e Fechamento da Janela

O compartilhamento da geometria de menus entre desenho e input está registrado na [ADR 0012](adr/0012-shared-menu-pointer-layout.md).

O menu recebe hover, clique esquerdo e clique direito por [`src/engine/input.rs`](../src/engine/input.rs). Desenho e detecção de linhas compartilham a geometria de [`src/ui/menu_layout.rs`](../src/ui/menu_layout.rs). [`PreferencesMenu`](../src/scenes/preferences.rs) usa movimento real do mouse para mudar a seleção; um ponteiro parado não disputa a seleção com teclado/gamepad. Clique esquerdo ativa a linha sob o ponteiro, inclusive `Exit` e `Back`, alterna flags ou avança valores. Clique direito volta valores ajustáveis. Cliques fora das linhas são ignorados, e a proteção de entrada nas transições também cobre o mouse.

O cursor nativo usa `show_cursor()`, que preserva sua posição. Não chamar `enable_cursor()` a cada quadro: no Raylib 6 essa função também centraliza o ponteiro, impedindo o movimento e o acesso ao botão de fechar. O overlay `Linker` em WSL acompanha a posição real e só aparece com a janela em foco e o mouse na área cliente. `Esc` mantém a função de voltar; o botão nativo de fechar e `Exit` encerram o jogo. A navegação por mouse é coberta em [`tests/feature_flags.rs`](../tests/feature_flags.rs); cursor e fechamento exigem também verificação com janela real.

### Fluxo de Início de Luta

O guia de primeira abertura reutiliza `MenuPage::HowToPlay` em
[`src/scenes/preferences.rs`](../src/scenes/preferences.rs) e a geometria compartilhada
de menus. O desenho fica em [`src/engine/render/onboarding.rs`](../src/engine/render/onboarding.rs).
`PlayMode` configura somente as duas flags CPU: contra CPU (manual/CPU), duelo local
(manual/manual) e demonstração (CPU/CPU). `App` começa em manual/CPU; o default
de `FeatureFlags` usado pelos testes de combate continua CPU/CPU. Escolher um modo
reinicia o mundo; fechar o guia só retorna ao menu, sem alterar o modo em andamento.

`App` grava o marcador `onboarding-v1.seen` em `runtime_paths::data_dir()` ao sair
do guia. O marcador evita repetir a apresentação; preferências e modo não são
persistidos. Falha de escrita não interrompe o jogo. CLI direto para luta ou
ferramentas ignora o guia e não grava o marcador. A cobertura de navegação está em
[`tests/feature_flags.rs`](../tests/feature_flags.rs); primeira abertura, repetição,
bypass e falha de armazenamento têm testes unitários em `app.rs`.

O início de luta fica em [`src/game/world.rs`](../src/game/world.rs), não no renderer. `World::new_greybox_with_intro` liga primeiro `spawn_intro_timer` para a entrada cinematográfica e também prepara `countdown_timer`.

O matchup inicial vem de [`LaunchOptions.match_options`](../src/cli.rs), que aceita `--p1`/`--player-one` e `--p2`/`--player-two` para a luta normal. O submenu `Versus Setup` da demo cicla Player 1 e Player 2 entre Rust, Duke/Java, C, Python e C++; Go/Gopher continua no enum e nas ferramentas, mas não entra no ciclo público por enquanto. O mesmo submenu também permite escolher a arena da próxima luta usando os nomes e locais expostos por [`ArenaId`](../src/game/arena.rs). [`App`](../src/app.rs) marca matchup ou arena como pendente e recria o mundo ao começar a próxima luta. `LaunchOptions.start_fight` vem de `--fight`/`--skip-menu` e permite iniciar direto em `AppScene::Fight`. [`App`](../src/app.rs) preserva essa escolha no primeiro mundo e em `restart_match`, chamando `World::new_greybox_with_intro_for_characters`.

Enquanto `spawn_intro_active` ou `countdown_active` estiverem ativos, `World::update_with_flags` atualiza apenas timers e feedback transitório; movimento, ataques, projéteis e IA não avançam gameplay. A contagem visual usa os labels `11`, `10`, `01` e `Fight!`, expostos por `World::countdown_label`. Os eventos de áudio correspondentes são `match.countdown.11`, `match.countdown.10`, `match.countdown.01` e `match.countdown.fight`.

O desenho da contagem fica em [`src/engine/render.rs`](../src/engine/render.rs), que só consulta `World::countdown_label`. A troca de arena é decisão de [`src/app.rs`](../src/app.rs): depois que `World::outcome` aparece, a arena atual permanece na pose de vitória e só avança quando uma nova luta é iniciada por restart ou pelo menu. Se o jogador escolher uma arena manualmente em `Versus Setup`, essa escolha vale para a próxima luta e desliga o avanço automático naquele restart.

### Hitbox e Hurtbox

Usamos retângulos axis-aligned, ou AABB, representados por [`Rect`](../src/math/rect.rs). Ainda não há física avançada, polígonos, capsule collision ou ECS.

Hurtboxes:

- `Fighter::body_parts` separa o corpo em cabeça, torso e pernas.
- `Fighter::hurtboxes` aplica `inset_rect` nessas partes para criar áreas vulneráveis.
- `Fighter::hurtbox` ainda existe como hurtbox grosseira para usos simples.

Hitboxes:

- `MoveSpec.hitbox` define largura, altura e offset vertical local do golpe.
- `Fighter::attack_box_for` posiciona essa caixa na frente do corpo conforme `Facing`.
- `Fighter::active_attack` só retorna hitbox ofensiva quando o frame atual está dentro da janela ativa.
- `combat::collision::hitbox_hits_hurtbox` usa interseção AABB.

Essa técnica foi escolhida porque é legível, testável sem Raylib e suficiente para o Prototype 0.1. Quando o frame do manifesto baseline declara `frames[].combat`, o runtime projeta esses dados para coordenadas de mundo em [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs). A resolução da luta usa `frames[].combat.hitboxes[]` e `frames[].combat.hurtboxes[]` quando essas listas existem; rasteiras usam o `MoveSpec` revisado e especiais de assinatura usam entidades físicas `SignatureEffect`, e ações baixas usam as hurtboxes físicas para preservar sua postura (ver [ADR 0013](adr/0013-contextual-showcase-and-mvp-combat.md)); se estiverem ausentes ou vazias, volta para `MoveSpec.hitbox` e `Fighter::hurtboxes()`. A decisão inicial está em [`ADR 0007`](adr/0007-sprite-frame-combat-runtime.md); a separação entre arte candidata e metadata baseline está em [`ADR 0010`](adr/0010-reviewed-action-sprite-production.md).

Rust, Duke, Go, C, Python e C++ ja possuem `combat.projectile_origin` no primeiro frame do clip `special`. Esse ponto e projetado por [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs) e usado por [`src/game/world.rs`](../src/game/world.rs) ao criar o projectile, para evitar que o poder nasca desalinhado da mao. Os manifests baseline de luta tambem declaram clips runtime para os nove golpes proximos (`punch_light`, `punch_heavy`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick`, `throw`) e para `hit` durante hitstun. Rust `Borrow Jab`, heavy punch e kick ja possuem hitboxes de frame; os valores ainda reproduzem o alcance do `MoveSpec` para migrar com baixo risco. Python e C++ raster high-res ainda usam fallback de `MoveSpec` para hitboxes dos golpes proximos ate uma calibracao propria no Sprite Studio. Hitboxes/hurtboxes restantes devem ser calibradas no Sprite Studio, com o Sprite Combat Viewer Raylib apenas como ferramenta temporaria ate a limpeza dedicada.

### Escala Visual e Pivot

A configuração padrão procura `assets/candidates/<key>/<key>-fighter.sprite.json`, com `key` igual a `rust`, `duke`, `go`, `c`, `python` ou `cpp`. Apenas conjuntos válidos e completos, com texturas carregáveis e 25 clips nos cinco personagens do MVP (20 em Go), substituem o desenho em luta, Move Showcase e Combat Lab. `BORROW_FIGHTERS_SPRITE_CANDIDATES=0` permite comparar o baseline preservado; `1` seleciona explicitamente a arte revisada. Ausencia/falha/incompletude mantem o baseline do personagem; a lista de clips faltantes aparece no terminal. Conjuntos parciais sao revisados diretamente no Viewer/Studio. A lista e os comandos estao no [pipeline de sprites](11-sprite-pipeline.md#revisao-de-candidatos-no-runtime).

O loader mantem `SpriteAtlasAsset.manifest` para desenho e `combat_manifest` para as boxes e a origem de projectile existentes. `App::sync_world_sprite_combat` usa o segundo, assim como o overlay da luta. Nenhum campo experimental de combate no candidato e promovido por ligar a variavel de ambiente.

`fighter_clip_elapsed_seconds` avanca reacoes desde o impacto e defesa/agachamento/salto desde a entrada no estado. Golpes e especiais mantem seus relogios atuais; `idle`/`walk` usam o tempo global. `fighter_combat_clip_elapsed_seconds` preserva o sampling antigo para nao trocar boxes: zero em stun, ultimo quadro em crouch, tres tempos por velocidade em jump. `crouch_block` tem desenho proprio e consulta a metadata baseline de `block`. A duracao visual do salto deve acompanhar subida, apice e queda reais, sem retunar gravidade ou velocidade.

O resultado da luta forca `victory` ou `defeat` com tempo iniciado no resultado, preservando o primeiro quadro mesmo em partidas longas; empate usa derrota nos dois lados. O renderer desliga efeitos persistentes de dano/guarda/ataque nessas poses, mantendo o estado de combate congelado. `spawn` pode vir do atlas principal e tem prioridade sobre o atlas de entrada separado. Clips antigos continuam com aliases visuais documentados no pipeline.

Sprites runtime usam `borrow-fighters.sprite.v1` em [`src/engine/sprites/manifest.rs`](../src/engine/sprites/manifest.rs). O campo `scale` controla o tamanho visual do atlas em jogo; `frames[].pivot` ancora cada frame no corpo do lutador.

O desenho fica em [`src/engine/sprites/draw.rs`](../src/engine/sprites/draw.rs). O renderer consulta `manifest.scale` e multiplica frame e pivot pelo mesmo valor. Portanto, escala e pivot salvos no manifesto afetam luta e Combat Lab sem recompilar. O loader em [`src/engine/assets.rs`](../src/engine/assets.rs) carrega `manifest.image` e qualquer `frames[].image`; o desenho escolhe a textura pelo frame atual. Isso permite compor personagens grandes em dois ou mais spritesheets, como `cpp-fighter-atlas-a.png` + `cpp-fighter-atlas-b.png`, mantendo um único `*.sprite.json`.

O corpo fisico base fica em [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json), carregado por [`src/characters/body_metrics.rs`](../src/characters/body_metrics.rs). Esse manifesto controla:

- `width`: largura do corpo para colisao corpo-corpo e hurtbox base;
- `standing_height`: altura em pe;
- `crouch_height`: altura abaixada.

`FighterBodyMetrics` e consumido por [`Fighter`](../src/combat/fighter.rs). Rust, Duke/Java, Go, C, Python e C++ usam o corpo padrao `101,3 x 224 / crouch 128` neste corte, migrado da base anterior por `RESOLUTION_SCALE = 4 / 3`. Go continua nao-humano visualmente, mas o atlas foi normalizado para caber na mesma escala jogavel de Rust/C em vez de ganhar hurtbox larga por causa de proporcao de placeholder. Python e C++ devem passar pela mesma revisao de escala/pivot dos outros humanoides antes de qualquer metrica propria. Se o arquivo falhar ao carregar no app, o jogo usa os defaults do `CharacterSpec` e emite warning.

O padrao de tamanho em tela fica em [`docs/17-visual-scale-and-stage-metrics.md`](17-visual-scale-and-stage-metrics.md). Em resumo:

- Rust atual e a referencia visual aprovada;
- humanoides devem ficar em torno de `247` a `280 px` de altura visivel em idle;
- personagens nao-humanos, como Go, devem mirar a mesma faixa principal quando a diferenca de tamanho nao for parte do gameplay;
- a arena atual tem `1194,7 px` jogaveis, cerca de `11,8` larguras de corpo padrao.

Use o Sprite Studio para edicao visual confortavel e validacao de runtime. O Sprite Combat Viewer embutido continua disponivel apenas ate a limpeza dedicada:

```bash
cd tools/sprite-studio && pnpm tauri dev
cargo run -- --tool sprite-viewer --manifest assets/placeholder/go-fighter.sprite.json --clip idle --character go --move light_punch
cargo run -- --tool sprite-viewer --manifest assets/placeholder/c-fighter.sprite.json --clip idle --character c --move light_punch
```

Atalhos de calibracao:

- `=` / `-`: ajusta `scale` do manifesto;
- `Setas`: move `pivot` do frame atual em 1 px;
- `Shift+Setas`: move `pivot` em 8 px;
- `Ctrl+Setas`: ajusta `width` e `standing_height` do corpo fisico;
- `Ctrl+Shift+Setas`: ajusta `crouch_height`;
- `Ctrl+S`: salva manifestos de tuning alterados;
- `F5`: recarrega manifesto e atlas.

### Frame Data

O jogo usa fixed timestep de 60 FPS em [`src/config.rs`](../src/config.rs). A linguagem de tuning do combate deve ser frame, não segundo.

- `FrameCount` fica em [`src/combat/frame.rs`](../src/combat/frame.rs).
- Golpes próximos usam `AttackFrameData` e `whiff_recovery` dentro de `MoveSpec`.
- Projectile/special usa `ProjectileFrameData` em [`src/combat/projectile.rs`](../src/combat/projectile.rs).
- O Combat Lab mostra frame atual, fase e janela ativa.

### Defesa, Guard Rule e Stun

`GuardRule` e `HitReaction` ficam em [`src/combat/move_data.rs`](../src/combat/move_data.rs). O corte atual já usa a linguagem mínima de defesa para golpes tradicionais:

- `GuardRule::Mid` bloqueia com defesa em pé ou abaixada.
- `GuardRule::Projectile` bloqueia com defesa em pé ou abaixada.
- `GuardRule::Low` exige defesa + abaixar.
- `GuardRule::High` exige defesa em pé; defesa abaixada perde para overhead.
- `GuardRule::Throw` é explicitamente não bloqueável.

Golpes jogáveis atuais usam essas regras assim:

| Golpe | Regra | Resposta mínima |
|---|---|---|
| `LightPunch`, `HeavyPunch`, `Kick`, `RustBorrowJab`, `RustLifetimeAntiAir`, `DukeBoilerplatePoke`, `GoGoroutineJab`, `GoDeferKick`, `RisingAntiAir` | `Mid` | defender, espaçar, punir whiff |
| `SweepKick`, `DukeGarbageCollectorSweep` | `Low` | defender abaixado, pular, ficar fora do alcance |
| `OverheadPunch`, `DukeAbstractFactoryOverhead`, `GoChannelOverhead`, `AirPunch`, `AirKick`, `GoHopkick` | `High` | defender em pé, andar fora, anti-air contra salto |
| `CloseThrow`, `RustOwnershipThrow`, `DukeEnterpriseThrow` | `Throw` | sair do alcance, pular, interromper startup |
| Projectile | `Projectile` | defender, pular, aproximar durante cooldown |

`HitReaction` contém `hitstun`, `blockstun`, `hit_pushback` e `block_pushback`. Ao receber um hit, [`Fighter::take_hit`](../src/combat/fighter.rs) calcula se a defesa bloqueia aquele `GuardRule`, aplica dano reduzido quando bloqueado, liga o timer correspondente e retorna um `DamageResult` com dano, bloqueio e pushback:

- `hitstun_timer`: interrompe ataque atual, seleciona o clip correspondente à reação e impede iniciar ação.
- `blockstun_timer`: mantém o lutador em defesa e impede iniciar ação.
- ambos são expostos para debug/testes por `hitstun_remaining_frames`, `blockstun_remaining_frames`, `in_hitstun` e `in_blockstun`.

A defesa baixa conserva a postura durante blockstun mesmo se o botão for solto. Chip é limitado para deixar pelo menos 1 HP. Acerto interrompe conjuração de projétil; a conjuração também trava novos ataques durante sua própria animação.

Rasteiras iniciam `Knockdown` no chão, com recuperação protegida de 36 frames. Agarrões passam por captura de 12 frames e voo de 40 frames antes dessa recuperação; o destino troca os lados no centro e usa espaço seguro no canto. Ganchos/anti-air e os especiais de C, Python e C++ usam `Launched`, com gravidade até aterrissar; golpes pesados usam `HeavyHit`, e impactos leves usam `Hit`. Captura, voo e chão têm clips próprios e proteção contra novos hits. KO espera a aterrissagem. Agarrões falham contra saltos, stun e os seis primeiros frames depois de levantar. Ver [regras e arte da rodada](21-signature-spectacle-and-throws.md).
- `hit_pushback` e `block_pushback`: deslocamento horizontal em pixels aplicado ao defensor, com block pushback menor que hit pushback no tuning atual.

O match runtime em [`src/game/world.rs`](../src/game/world.rs) passa `guard_rule` e `hit_reaction` de `ActiveAttack` ou `Projectile` para o defensor. O próprio `World` aplica o pushback, porque é ele quem sabe de qual lado está atacante, defensor e projétil. O spark e o número de dano usam o centro da interseção real entre a hitbox e a hurtbox atingidas, antes da reação e do pushback; uma rasteira mostra impacto nas pernas. Depois do deslocamento, `Fighter::clamp_to_arena` mantém o defensor dentro da arena. Feature flags de dano ainda impedem dano, stun e pushback quando desativadas.

### Dados de Golpes

Os golpes próximos atuais estão em [`src/combat/move_data.rs`](../src/combat/move_data.rs):

- `LightPunch`
- `HeavyPunch`
- `Kick`
- `SweepKick`
- `OverheadPunch`
- `RisingAntiAir`
- `AirPunch`
- `AirKick`
- `CloseThrow`
- `RustBorrowJab`
- `RustLifetimeAntiAir`
- `RustOwnershipThrow`
- `DukeBoilerplatePoke`
- `DukeGarbageCollectorSweep`
- `DukeAbstractFactoryOverhead`
- `DukeEnterpriseThrow`
- `GoGoroutineJab`
- `GoDeferKick`
- `GoChannelOverhead`
- `GoHopkick`
- `CPointerJab`
- `CUnsafePoke`
- `CNullStepKick`
- `CSegfaultSweep`
- `CStackOverflow`
- `CInterruptVector`
- `CUndefinedThrow`
- `PythonSnakeBite`
- `PythonDataStrike`
- `PythonHeelKick`
- `PythonIndentSweep`
- `PythonTracebackOverhead`
- `PythonVisionAntiAir`
- `PythonConstrictThrow`
- `CppReferenceJab`
- `CppTemplateStrike`
- `CppOperatorKick`
- `CppVectorSweep`
- `CppVirtualOverhead`
- `CppExceptionAntiAir`
- `CppMoveThrow`

`DEFAULT_CLOSE_RANGE_MOVE_IDS` define a lista padrão genérica usada por construtores e testes que não selecionam personagem. `CharacterSpec.move_ids` define o loadout real de cada personagem.

`Fighter` carrega `move_ids` próprios. Quando um botão de golpe é pressionado, `FighterInput::requested_move_spec` escolhe o `MoveInputKind` a partir de botão, direção, abaixar, defesa e estado aéreo. Depois `move_spec_for_input` procura no loadout o primeiro `MoveSpec` com aquele input. Se não houver `MoveId` compatível, o input daquele golpe não inicia ataque. Isso permite que o mesmo botão resolva para golpes diferentes por personagem sem alterar profundamente `Fighter`.

### Especiais cinematográficos adicionais

`MoveInputKind::CinematicSpecial` identifica uma segunda ação temática, independente de `SignatureSpecial` e do projétil. Os seis personagens conservam seus `MoveId` estáveis para inputs, catálogo e bindings de áudio. A [ADR0016](adr/0016-authored-super-sequences.md) introduziu a captura autoral para Rust, Duke, C e C++; a [ADR0017](adr/0017-reaction-clocks-and-arena-mutation.md) amplia o roteiro de C++, adiciona Python e torna persistente a mutação de arena de Rust.

`World::try_start_super` aceita o comando apenas com o atacante vivo, livre e no chão, fora de intro/countdown/agarrão. Guarda mantida não impede `LB+RT`. A aceitação limpa as ações dos atores, registra posições e defesa do alvo, emite `SuperStart` e começa no tick zero. Duas solicitações elegíveis no mesmo tick anulam ambas, sem vencedor arbitrário por slot; a próxima solicitação pode iniciar normalmente. A captura é garantida a qualquer distância, e inputs posteriores não alteram a defesa capturada nem aceleram fases. Um alvo no ar permanece parado no freeze inicial e desce continuamente até a âncora de chão nos ticks 8–20, conservando X; as fases e contatos seguintes encontram ambos os atores no piso, inclusive na corrida de C++.

| Personagem / golpe | MoveId estável | Dano / chip | Contatos (ticks) | Duração a 60 Hz |
|---|---|---|---|---|
| Rust / Ownership Eclipse | `RustOwnershipEclipse` | 28 / 7 | 212 | 300f / 5s |
| Java / Garbage Collector | `DukeJvmOverdrive` | 32 / 8 | 264 | 350f / 5,83s |
| C / General Protection Fault / #GP | `CKernelPanic` | 32 / 8 | 222 | 320f / 5,33s |
| C++ / Undefined Behavior: Footgun | `CppTemplateSingularity` | 36 / 10 | 344–414, a cada 10f: 8 × 3; 424: 6; reboot 566: 6 | 632f / 10,53s |
| Python / import devour | `PythonEventHorizon` | 32 / 8 | deglutição 304 | 528f / 8,8s |

Os intervalos de fase são semiabertos `[início, fim)`, declarados em `super_spec(character)`. Todos começam em `Freeze` 0–8. Rust: apagão 8–11, construção 11–125, carga 125–205, pulso 205–235, restauração 235–300. Duke: lixo caindo 8–60, assentado 60–84, 12 coletas 84–228 (cadência 12f), queda gigante 228–264, impacto 264–302 e restauração 302–350. C: terminais 8–105, tela azul 105–180, BIOS 180–222, reboot 222–260 e restauração 260–320. C++: notebook/código 8–148, tiro no pé 148–202 (disparo 184), saltos 202–250, raiva 250–280, corrida 280–344, rajada 344–424, final 424–450, terminais herdados 450–498, tela azul 498–538, BIOS 538–566, reboot 566–596 e restauração 596–632.

Python: preparação 8–62, transformação 62–158, crescimento 158–230, boca aberta 230–270, bote 270–304, deglutição 304–344, reversão 344–400, celebração 400–488 (salto até 444, paz depois) e restauração 488–528. `target_hidden()` informa ao renderer que o alvo está invisível em 304–388, inclusive quando guardou. Ele continua existindo no `World`, com HP e reação reais. No tick 388, `resume_super_target_reaction()` reinicia a reação de retorno sem outro contato, dano ou evento de hit.

A construção de Rust revela Sirius sobre a arena atual em blocos. No tick `RUST_ARENA_COMMIT_TICK` (125), o `World` grava `arena_override = Some(Sirius)`. `effective_arena(base)` retorna a identidade efetiva, e `arena_override()` permite inspecionar a mutação. Ela permanece depois do super, guarda e KO; não altera o argumento base nem as preferências globais. Recriar o `World` no reset, novo round ou replay remove a mutação. App, renderer e áudio devem consumir a mesma arena efetiva para nome, música e vida ambiente.

`World::super_sequence()` expõe `SuperSequence` com personagem, MoveId, label, slots atacante/alvo, `guarded`, `target_crouching`, tick/duração, posição original e atual dos pés de cada ator e facing. `phase()`, `phase_span()`, `phase_progress()` e `progress()` orientam renderer e áudio. `World.elapsed_seconds` continua avançando para apresentação; o gameplay comum não avança enquanto há sessão. Não existem hitboxes ofensivas de tela nem projéteis decorativos com dano. C++ desloca sua posição física continuamente de 280 a 344 até o gap corporal mínimo diante do alvo, inclusive nos cantos e na orientação inversa.

Somente os `SuperContact` aplicam dano. Guarda em qualquer altura causa `max(dano / 4, 1)` por contato, limitado a manter pelo menos 1 HP. Flags de invencibilidade continuam respeitadas. O tiro no próprio pé não retira HP. Contatos fortes sem bloqueio lançam o alvo, que progride por voo, queda e recuperação protegida de 36f. O relógio da reação continua avançando durante a sequência: vítimas vivas conseguem completar a recuperação antes da restauração, e novos contatos reiniciam o recoil. Vítimas derrotadas permanecem caídas após aterrissar. A vida pode chegar a zero durante a sessão, mas `resolve_outcome` só anuncia KO depois de `SuperEnd`. Recriar `World` remove sessão, contatos pendentes e relógio. A fronteira do app pausa/retoma música pelo estado da sessão e limpa a pausa ao sair da cena.

Go / Million Goroutines (`GoMillionGoroutines`) preserva `AttackKind::CinematicSpecial` local: 25 de dano, ativos inclusivos 32–39, duração 94f e alcance `world_px(100)`. É mid, hitstun 28f, blockstun 16f, pushback H/B `world_px(72/28)`, 18f extras de whiff e hitbox local de altura `world_px(142)`. Pode errar à distância e sofrer interrupção; metadata não amplia o alcance. `Fighter::cinematic_special()` expõe esse relógio para sua apresentação. Os antigos timings `MoveSpec` dos cinco supers autorais são dados legados de pose/compatibilidade; o runtime usa `super_spec` como autoridade de fases e dano.

Teclado: `Y` P1 e `]` P2. `Right Shift` conserva o soco forte P2. Gamepad: segurar `LB` e pressionar `RT`; `RT` sem `LB` continua assinatura. `PendingFighterInput` preserva a borda até o próximo tick e a consome uma única vez em catch-up. A CPU continua selecionando cinematográficos ocasionalmente pela heurística de alcance local; a aceitação dos cinco supers não depende dessa distância.

```bash
cargo run -- --showcase --character rust --move cinematic_special --repeat
cargo run -- --showcase --character cpp --move cinematic_special --repeat --reverse
cargo run -- --lab combat --character c --move cinematic_special
```

CLI também aceita `cinematic`, `cinematic-special` e `ultimate`. O Combat Lab mantém um `World` completo para os cinco supers, acessível por `super_preview_world()`, reproduz ambos os atores e disponibiliza os cues reais por `take_super_audio_events()`. Não apresenta dummy, alcance melee nem vantagem fictícia para capturas. Go mantém o dummy local. Showcase prepara o alvo à distância nos cinco supers e calcula `scenario_frames()` como 30f de preparação + duração da sessão + 90f de observação; os outros exemplos conservam 260f. Pausa e avanço por frame preservam o relógio de cada modo. Há 16 situações por personagem da demo e 15 para Go, que não possui a assinatura anterior.

[`tests/authored_super_sequences.rs`](../tests/authored_super_sequences.rs) verifica ambas as orientações e slots, distância/cantos, fases e contatos, corrida física, guarda capturada/chip, preservação de HP no treino, KO adiado, comandos simultâneos, alvo aéreo, emissão única de áudio e reset, mutação persistente para Sirius e retorno de Python sem dano adicional. [`tests/cinematic_specials.rs`](../tests/cinematic_specials.rs) preserva a matriz local de Go e verifica CLI, pause/frame-step e replay dos seis personagens. [`tests/move_showcase.rs`](../tests/move_showcase.rs) valida contatos reais de todos os golpes com e sem metadata.

As flags `PlayerOneTakesDamage` e `PlayerTwoTakesDamage` controlam somente a
redução de vida. O contato continua aplicando reação, interrupção, defesa,
empurrão, captura/lançamento e feedback sonoro. Chip também fica em zero com a
flag desligada. Não usar `damage > 0` como condição para ativar resposta a um
contato confirmado; isso deixava os personagens em idle no treino sem dano.

### Combat Log

O log de combate fica em [`src/game/combat_log.rs`](../src/game/combat_log.rs) e é preenchido por [`World`](../src/game/world.rs). Ele registra eventos compactos como início de round, countdown, ataque iniciado, whiff, hit/block resolvido, projectile disparado, projectile resolvido e fim de luta.

Use `World::combat_log()` em testes ou ferramentas de debug para inspecionar a sequência atual, e `World::clear_combat_log()` quando um teste quiser isolar uma janela específica. O log é limitado por `COMBAT_LOG_CAPACITY` para não crescer indefinidamente. Ele não substitui `AudioEvent`: áudio é feedback; `CombatLog` é rastreio técnico.

Mapeamento atual de input:

| Input | `MoveInputKind` |
|---|---|
| `F` P1 / `Enter` ou `O` P2 | `LightPunch` |
| `H` P1 / `Right Shift` ou `P` P2 | `HeavyPunch` |
| `V` P1 / `;` ou `/` P2 | `Kick` |
| Abaixar + chute | `Sweep` |
| Abaixar + soco forte | `AntiAir` |
| Frente + soco forte | `Overhead` |
| Defender + soco fraco | `Throw` |
| `Y` P1 / `]` P2 / `LB+RT` | `CinematicSpecial` |
| No ar + soco fraco/forte | `AirPunch` |
| No ar + chute | `AirKick` |

`AttackKind` em [`src/combat/move_set.rs`](../src/combat/move_set.rs) ainda existe como camada runtime de compatibilidade para sprites, debug e categorias visuais. [`src/engine/sprites/selection.rs`](../src/engine/sprites/selection.rs) converte cada `AttackKind` em um clip visual proprio, enquanto o dano, a hitbox e o frame data durante uma luta vêm do `MoveSpec` concreto guardado no estado de ataque.

### Whiff Recovery

`MoveSpec.whiff_recovery` define o lockout aplicado quando um golpe próximo termina sem acertar. O fluxo fica em [`Fighter::update`](../src/combat/fighter.rs):

1. ataque inicia e roda `AttackFrameData`;
2. se [`World`](../src/game/world.rs) registra contato, `mark_attack_hit` impede whiff recovery;
3. se a duração acaba sem contato, `Fighter` limpa o ataque atual e liga `whiff_recovery_timer`;
4. enquanto `in_whiff_recovery` estiver ativo, o lutador não anda, não pula, não defende, não inicia outro golpe e não dispara projectile.

O debug visual mostra `WHIFF xx` quando `Mostrar debug de combate` está ligado. O Combat Lab mostra `whiff` no overlay para comparar custo de erro com `rec` em contato.

### Áudio de Combate

O combate não toca arquivos diretamente. O fluxo atual é:

1. [`Fighter::update`](../src/combat/fighter.rs) retorna eventos de início de golpe e whiff.
2. [`World`](../src/game/world.rs) transforma esses eventos em `AudioEvent` com `PlayerSlot`, `CharacterId` e `MoveId`.
3. `World` também emite eventos de contagem pré-luta, hit, block, dor, projectile e vitória.
4. [`App`](../src/app.rs) drena `World::take_audio_events` depois de cada fixed update.
5. [`src/engine/audio.rs`](../src/engine/audio.rs) resolve bindings em [`assets/audio/audio_manifest.json`](../assets/audio/audio_manifest.json) e toca clips existentes via Raylib.

Cues relevantes para combate:

| Cue | Quando usar |
|---|---|
| `match.countdown.11` | primeira etapa da contagem visual binária |
| `match.countdown.10` | segunda etapa da contagem visual binária |
| `match.countdown.01` | terceira etapa da contagem visual binária |
| `match.countdown.fight` | liberação da luta |
| `fighter.attack.start` | voz/esforço no início do golpe |
| `fighter.attack.whiff` | som seco quando um golpe termina sem contato |
| `fighter.projectile.cast` | carga/disparo do especial |
| `combat.hit` | impacto de golpe próximo |
| `combat.block` | impacto em defesa |
| `fighter.hurt` | voz de dano do defensor |
| `fighter.block` | esforço de defesa do defensor |
| `projectile.impact` | impacto do projétil |

Ao adicionar golpe novo:

- crie ou reutilize um `MoveId` com `audio_key`;
- confira se `tests/audio_manifest.rs` aceita a chave;
- adicione binding no manifesto quando houver clip planejado;
- documente variações em [`docs/14-audio-pipeline.md`](14-audio-pipeline.md) se criar cue nova.

### Dados de Personagens

Personagens ficam em [`src/characters/mod.rs`](../src/characters/mod.rs). Cada `CharacterSpec` contém:

- `display_name`: nome para UI/lab;
- `fighter_name`: nome curto usado pelo lutador;
- `archetype`: intenção de gameplay;
- `stats.max_health`: vida máxima usada na criação do `Fighter`;
- `move_ids`: golpes próximos disponíveis no loadout;
- `projectile`: `ProjectileSpec` usado para dano, tamanho, velocidade, cooldown, reação e limite de alcance do especial.

Hoje `Rust` usa `RustBorrowJab`, `RustLifetimeAntiAir` e `RustOwnershipThrow` para reforçar leitura técnica: golpes mais rápidos, menores e menos danosos. `Duke` usa `DukeBoilerplatePoke`, `DukeGarbageCollectorSweep`, `DukeAbstractFactoryOverhead` e `DukeEnterpriseThrow` para reforçar midrange pressure: mais alcance/dano, startup maior e whiff mais punível. `Go` usa `GoGoroutineJab`, `GoDeferKick`, `GoChannelOverhead` e `GoHopkick` para validar rushdown com atlas placeholder: menos vida, ações mais rápidas e alcance menor. `C` usa `CPointerJab`, `CUnsafePoke`, `CNullStepKick`, `CSegfaultSweep`, `CStackOverflow`, `CInterruptVector` e `CUndefinedThrow` para jogar fundamentos com alcance/risco. `Python` usa `PythonSnakeBite`, `PythonDataStrike`, `PythonHeelKick`, `PythonIndentSweep`, `PythonTracebackOverhead`, `PythonVisionAntiAir` e `PythonConstrictThrow` para jogar como punisher ágil de dano moderado.

Os especiais de projectile ficam em [`src/combat/projectile.rs`](../src/combat/projectile.rs) como `RUST_PROJECTILE_SPEC`, `DUKE_PROJECTILE_SPEC`, `GO_PROJECTILE_SPEC`, `C_PROJECTILE_SPEC` e `PYTHON_PROJECTILE_SPEC`. `Fighter::projectile_spec` alimenta `Projectile::from_fighter`, o Combat Lab e o overlay técnico, então alterar um spec muda luta real e lab no mesmo caminho.

`World::new_with_characters` e `World::new_greybox_with_intro_for_characters` aceitam qualquer `CharacterId`; a luta padrão ainda instancia Rust x Duke. O submenu `Versus Setup` da demo cicla Rust, Duke/Java, C, Python e C++ para personagens e percorre as arenas com nomes contextualizados. Go/Gopher continua testável por `--p1`/`--p2`, Combat Lab e Sprite Viewer, mas fica fora do menu público por enquanto.

A intenção de gameplay por golpe vive em [`docs/15-character-combat-matrix.md`](15-character-combat-matrix.md). Atualize essa matriz quando alterar frame data, alcance, dano, guard rule, projectile ou loadout de personagem.

### Move Showcase

O showcase abre por `Training -> Move Showcase` ou diretamente:

```bash
cargo run -- --showcase --character rust
cargo run -- --showcase --character cpp --move signature_special --repeat --reverse
```

[`MoveShowcase`](../src/scenes/move_showcase.rs) executa dois lutadores em um `World` real. Cada personagem da demo apresenta doze ataques (incluindo projétil, especial de assinatura e cinematográfico) e quatro exemplos de defesa. Rasteira enfrenta guarda alta; overhead enfrenta guarda baixa; anti-air recebe um salto de aproximação; agarrão precisa alcançar um alvo em guarda no chão. Dano, bloqueio, projéteis, reações e áudio vêm da resolução normal do jogo. O painel descreve a situação e confirma o contato ocorrido.

A [renderização](../src/engine/render/move_showcase.rs) mostra ambos os atores sem volumes de debug. `Tab` / `Shift+Tab` alternam demonstrações, `Enter` repete, `Espaço` pausa, `.` avança um frame, `Home` reinicia, `L` alterna repetição e `X` inverte os lados. `PageDown` / `PageUp` alternam os cinco personagens desta rodada; `Esc` volta ao menu. Os cenários e as opções são preservados ao trocar de personagem.

### Combat Lab

Abrir o laboratório:

```bash
cargo run -- --fight --p1 go --p2 duke
cargo run -- --fight --p1 c --p2 rust
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move sweep
cargo run -- --lab combat --character duke --move throw
cargo run -- --lab combat --character go --move kick
cargo run -- --lab combat --character go --move air_kick
cargo run -- --lab combat --character c --move heavy_punch
cargo run -- --lab combat --character c --move projectile
cargo run -- --lab combat --character python --move light_punch
cargo run -- --lab combat --character rust --pose block
cargo run -- --lab combat --character duke --pose victory
cargo run -- --lab combat --character rust --pose spawn
cargo run -- --lab combat --character rust --pose defeat
cargo run -- --lab combat --character rust --pose crouch_block
```

O mesmo laboratório também pode ser aberto pelo menu principal em `Training -> Combat Lab`. Nesse fluxo, `Esc` volta ao menu sem fechar a janela.

`App` fornece ao Lab o mesmo `combat_manifest` baseline usado pelo `World`, incluindo com arte candidata habilitada. O Lab projeta as caixas com o sampling de combate existente e consulta a origem do especial em tempo zero, com fallback para `Fighter`/`Projectile` quando não há metadata. Move Showcase conserva esse baseline ao avançar e repetir golpes. As estimativas de vantagem e o posicionamento automático do dummy continuam calculados por MoveSpec; não são uma simulação de contato com caixas de atlas.

Valores aceitos:

| Flag | Valores |
|---|---|
| `--fight`, `--skip-menu` | sem valor; inicia direto na luta normal |
| `--p1`, `--player-one` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx`, `cpp.cpp` |
| `--p2`, `--player-two` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx`, `cpp.cpp` |
| `--character` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx`, `cpp.cpp` |
| `--move` | `light_punch`, `heavy_punch`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick`, `throw`, `projectile` |
| `--pose` | `move`, `idle`, `crouch`, `jump`, `block`, `hit`, `victory`, `spawn`, `defeat`, `crouch_block` |
| `--tool` | `sprite-viewer` |
| `--manifest` | caminho para um JSON `borrow-fighters.sprite.v1` |
| `--clip` | nome de clip presente no manifesto |

`--pose move` é o modo padrão e reproduz o golpe selecionado por `--move`. As outras poses fixam o estado físico e reproduzem o clip para inspecionar desenho, pivot e hurtbox sem depender de uma luta real. Pause, frame step e reinício também funcionam nessas poses.

Teclas:

| Ação | Tecla |
|---|---|
| Próximo golpe | `Tab` |
| Golpe anterior | `Shift+Tab` |
| Próxima pose | `PageDown` |
| Pose anterior | `PageUp` |
| Repetir golpe | `Enter` |
| Pausar/continuar | `Espaço` |
| Avançar 1 frame | `.` |
| Voltar ao frame 0 | `Home` |
| Alternar hurtbox | `H` |
| Alternar hitbox | `B` |
| Alternar pivot/eixos | `P` |
| Alternar dummy de contato | `D` |
| Alternar fundo de arena | `A` |
| Voltar ao menu quando aberto por `Training` | `Esc` |

Use o Combat Lab para conferir:

- se a mão ou o pé está alinhado com a hitbox;
- se o projectile nasce na altura correta;
- se o pivot está no chão e no centro esperado;
- se startup, active e recovery batem com a tabela;
- se `adv hit/block`, `rec`, `push H/B` e `gap H/B` fazem sentido para o golpe selecionado;
- se a hurtbox muda de modo previsível quando o estado muda.

No overlay do Combat Lab:

- `adv hit` e `adv block` são estimativas em frames: `stun do defensor - recovery restante do atacante após o contato`;
- `rec` é o recovery restante do atacante depois do primeiro frame de contato estimado;
- `whiff` é o lockout extra quando o golpe termina sem contato;
- `cd` aparece em projectile e indica cooldown restante, não recovery de ação;
- `push H/B` mostra pushback em hit e block;
- `gap H/B` mostra a distância corpo-corpo estimada depois do pushback;
- `D` liga um dummy posicionado no ponto em que o golpe selecionado deve conectar, para validar alcance e espaçamento visualmente.

Poses atuais:

- `move`: reproduz o golpe selecionado;
- `idle`: pose neutra;
- `crouch`: aplica estado de abaixar e hurtbox menor;
- `jump`: posiciona o lutador no ar para conferir corpo/pivot;
- `block`: aplica estado de defesa;
- `hit`: força clip visual `hit` quando o manifest possui esse clip;
- `victory`: força `victory`, com fallback para `taunt`/`idle`;
- `spawn`: reproduz entrada do atlas principal ou separado;
- `defeat`: força `defeat`, com fallback para `hit`/`idle`;
- `crouch_block`: aplica defesa e agachamento e força o clip correspondente, com fallback para `block`/`idle`.

### Sprite Combat Viewer

O Sprite Combat Viewer Raylib e ferramenta temporaria. A direcao aprovada em [`docs/adr/0008-external-sprite-studio-tooling.md`](adr/0008-external-sprite-studio-tooling.md) e mover a edicao rica para [`tools/sprite-studio`](../tools/sprite-studio). A paridade operacional ja existe; o viewer embutido deve ser removido em uma mudanca propria.

Abrir a ferramenta isolada de sprites:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
cargo run -- --tool sprite-viewer --manifest assets/placeholder/duke-fighter.sprite.json --clip special --character duke --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/c-fighter.sprite.json --clip special --character c --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/python-fighter.sprite.json --clip punch_light --character python --move light_punch
```

O viewer roda fora do loop normal de luta. Pela CLI, [`src/app.rs`](../src/app.rs) desvia para esse modo antes de carregar `GameAssets` e áudio. Pelo menu, `Training -> Sprite Viewer` abre o mesmo loop com o atlas de C em `special/projectile` como ponto de partida, e `Esc` volta ao menu. O estado testável fica em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), e o desenho Raylib fica em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs).

`--character` e `--move` ativam a camada runtime de combate no viewer. Sem `--character`, o viewer tenta inferir Rust, Duke, Go, C ou Python pelo nome do manifesto. Essa camada usa `CharacterSpec`, `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`, então ela reflete os dados de combate atuais.

O viewer tambem entende metadata opcional `frames[].combat` no manifesto. Essa metadata e projetada para tela em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), desenhada em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs), e validada em [`tests/sprite_manifest.rs`](../tests/sprite_manifest.rs). As coordenadas ficam em pixels locais do frame do atlas:

```json
"combat": {
  "hurtboxes": [{ "x": 10, "y": 8, "w": 48, "h": 96, "label": "body" }],
  "hitboxes": [{ "x": 62, "y": 38, "w": 28, "h": 22, "label": "strike" }],
  "projectile_origin": { "x": 84, "y": 44 }
}
```

`frames[].combat` do manifesto baseline ja alimenta a luta real de forma incremental. Quando um frame possui `hitboxes`, elas substituem a hitbox ofensiva de `MoveSpec`, exceto nas rasteiras e nos especiais de assinatura. Hurtboxes de metadata substituem as compostas de `Fighter`, exceto nas ações baixas revisadas. A geometria autoritativa dessas exceções é descrita nas ADRs 0013 e 0014. Quando o clip `special` possui `projectile_origin`, o projectile nasce desse ponto projetado para o mundo. Campos ausentes mantem fallback para `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`, entao personagens sem metadata continuam jogaveis. Metadata editada em um arquivo candidato continua apenas nesse arquivo; ativar o desenho candidato nao substitui `combat_manifest`.

Teclas:

| Ação | Tecla |
|---|---|
| Inspecionar coordenada local/atlas | Mouse sobre o sprite |
| Arrastar personagem | Mouse esquerdo |
| Próximo clip | `Tab` |
| Clip anterior | `Shift+Tab` |
| Sincronizar clip com golpe | `Enter` |
| Próximo personagem de combate | `C` |
| Personagem de combate anterior | `Shift+C` |
| Próximo golpe | `]` |
| Golpe anterior | `[` |
| Próximo frame | `.` |
| Frame anterior | `,` |
| Pausar/continuar | `Espaço` |
| Zoom | Mouse wheel |
| Resetar zoom | `0` |
| Aumentar `scale` do manifesto | `=` |
| Diminuir `scale` do manifesto | `-` |
| Mover `pivot` do frame atual | `Setas` ou `Shift+Setas` |
| Ajustar largura/altura do corpo físico | `Ctrl+Setas` |
| Ajustar altura abaixada do corpo físico | `Ctrl+Shift+Setas` |
| Gerar rascunho de `frames[].combat` pelo overlay runtime | `N` |
| Adicionar hurtbox no frame atual | `H` |
| Adicionar hitbox no frame atual | `J` |
| Remover box/origem sob o mouse ou último item | `Delete` |
| Mover hurtbox/hitbox/origem de projectile do frame | Mouse esquerdo nas boxes/alças |
| Redimensionar hurtbox/hitbox do frame | Mouse esquerdo nos cantos da box |
| Salvar manifestos de tuning | `Ctrl+S` |
| Mostrar/esconder dummy | `O` |
| Mostrar/esconder boxes de combate | `M` |
| Mostrar/esconder trajetória de projectile | `T` |
| Recarregar manifesto e atlas | `F5` |
| Salvar screenshot | `F12` |
| Alternar grade | `G` |
| Alternar pivot | `P` |
| Alternar bounds | `B` |
| Resetar posição | `R` |
| Voltar ao menu quando aberto por `Training` | `Esc` |

O corte atual mostra atlas, pivot, frame bounds, dummy espelhado, distância entre anchors, coordenada local/atlas do cursor, `trimmed_bounds`, `source_crop`, hurtbox atual do corpo, hitbox do golpe selecionado, origem/caixa de projectile, trajetória prevista de projectile, metadata `frames[].combat` e timeline inferior com fase aproximada de startup/active/recovery quando `--character` e `--move` estao presentes. A coordenada do cursor é a referência prática para preencher `frames[].combat`: `local x,y` entra no JSON do frame; `atlas x,y` serve para conferir a posição no PNG.

O viewer já possui edição visual para essa metadata: `N` substitui a metadata do frame atual por um rascunho vindo do overlay runtime, o mouse move boxes/origem, cantos das boxes redimensionam hurtboxes/hitboxes, e `Ctrl+S` persiste o manifesto. Essa edição fica em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs) e é coberta por [`tests/sprite_viewer.rs`](../tests/sprite_viewer.rs). A projeção runtime fica em [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs), a resolução de hits fica em [`src/game/world.rs`](../src/game/world.rs), e a renderização das alças fica em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs).

O personagem e o golpe podem ser trocados em runtime com `C`/`Shift+C` e `[`/`]`, sem reabrir o comando. `Enter` tenta selecionar o clip mais provável para o golpe atual. `F5` recarrega manifesto e atlas para iteração com ferramenta externa aberta; `F12` salva screenshot em `captures/sprite-viewer-capture.png` dentro dos dados do usuário para anexar em PR/issue. A evolução restante está rastreada em [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) e na issue [#15](https://github.com/osdeving/borrow-fighters/issues/15).

## Captura de Gameplay

A pasta de dados é `%LOCALAPPDATA%\BorrowFighters` no Windows ou
`$XDG_DATA_HOME/borrow-fighters` no Linux (fallback
`~/.local/share/borrow-fighters`). `BORROW_FIGHTERS_DATA_DIR` aceita um caminho
absoluto alternativo. FFmpeg é opcional; no Windows a gravação atual captura
somente vídeo. Linux usa PulseAudio para o áudio conforme a configuração abaixo.

Atalhos globais:

- `F9`: inicia gravação local da janela atual;
- `F10`: para a gravação e salva o MP4 em `captures/` dentro dos dados do usuário.
- Menu `Options` -> `Local Recording`: inicia/para pelo menu quando teclas de função não chegam ao jogo.

O código fica em [`src/engine/video_capture.rs`](../src/engine/video_capture.rs). A técnica usada é manter a captura fora do gameplay: `App` só detecta os atalhos, desenha a cena normalmente e, quando há gravação ativa, envia o framebuffer renderizado pelo Raylib para `ffmpeg` como `rawvideo`. O áudio vem do PulseAudio e o MP4 é finalizado ao fechar o pipe de vídeo.

O overlay global de REC fica em [`src/engine/render.rs`](../src/engine/render.rs), enquanto os atalhos vêm de [`src/engine/input.rs`](../src/engine/input.rs). No Sprite Combat Viewer, os mesmos atalhos são lidos no loop isolado de [`src/app.rs`](../src/app.rs) para permitir gravar revisão de atlas, pivot, hitbox, hurtbox e projectile origin.

No WSLg, `x11grab` pode produzir vídeo preto em janelas aceleradas. Por isso a captura usa frames internos do Raylib em vez de capturar a janela pelo desktop.

No WSLg, a fonte de áudio padrão atual é `RDPSink.monitor`. Em outro ambiente PulseAudio/PipeWire, descubra a fonte com `pactl list short sources` e rode:

```bash
BORROW_FIGHTERS_CAPTURE_AUDIO_SOURCE=<fonte> cargo run
```

Para validar o motor de captura sem depender de automação de teclado, rode:

```bash
BORROW_FIGHTERS_CAPTURE_SMOKE_SECONDS=8 cargo run -- --fight
```

Esse hook inicia a gravação automaticamente, para depois do número de segundos informado e usa o mesmo pipeline de `F9`/`F10`: render texture do Raylib, áudio PulseAudio e saída em `captures/` dentro dos dados do usuário.

## Cabeçalho de Arquivos

Arquivos Rust novos devem começar com:

```rust
//! Frase curta dizendo o que o arquivo faz.
//!
//! System: Nome do sistema maior. Explica qual motor/módulo possui este arquivo
//! e o que não pertence aqui.
```

Exemplos de sistemas:

- `Combat runtime`
- `Combat data`
- `Character data`
- `Combat Lab scene`
- `Raylib render boundary`
- `Sprite runtime`
- `Application bootstrap`

## Checklist ao Alterar Combate

1. Atualize ou adicione teste em `tests/`.
2. Atualize este guia se mudar arquivo, comando, tecla, técnica ou dado relevante.
3. Atualize [`docs/08-code-architecture.md`](08-code-architecture.md) se mudar árvore ou fronteira.
4. Atualize [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md) se concluir fase ou mudar backlog.
5. Atualize [`docs/14-audio-pipeline.md`](14-audio-pipeline.md) se mudar cue, binding, evento ou manifesto de áudio.
6. Atualize [`CHANGELOG.md`](../CHANGELOG.md).
7. Se a mudança for estrutural e durável, atualize ou crie ADR em [`docs/adr/`](adr/).

## Comandos de Validação

```bash
cargo fmt
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

Checks de documentação usados no CI:

```bash
ruby -e 'require "yaml"; Dir[".github/**/*.yml", ".github/**/*.yaml", ".agents/**/*.yaml", ".claude/**/*.yaml"].sort.each { |f| YAML.load_file(f); puts "ok #{f}" }'
```

```bash
ruby -e 'bad = []; Dir["{README.md,CONTRIBUTING.md,CHANGELOG.md,AGENTS.md,CLAUDE.md,docs/**/*.md,.agents/**/*.md,.claude/**/*.md}"].each { |file| text = File.read(file); text.scan(/\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)/).flatten.each { |link| next if link =~ %r{^[a-z]+://}; path = File.expand_path(link, File.dirname(file)); bad << "#{file}: #{link}" unless File.exist?(path) } }; if bad.empty? then puts "markdown links ok" else warn bad.join("\n"); exit 1 end'
```

A vantagem mostrada pelo Combat Lab é estimada pelo `MoveSpec`; não inclui os 36 frames de knockdown. Para verificar queda, levantar e nova ação, use o showcase ou a luta real. O Lab rejeita `signature_special` para Go, que não possui essa ação.

Assinaturas no Combat Lab são prévias do ator: sem dummy de contato, hitbox de melee ou vantagem estimada. Para efeitos, trajeto e contato real, use Move Showcase. O Lab mantém a mesma geometria física baixa das rasteiras.

## Reações por contato — piloto Python/C++

A [rodada 25](25-python-cpp-contact-reactions.md) aplica desenhos próprios à dupla.
Cada contato no World escolhe `ContactReactionProfile` e reinicia um relógio
visual. O renderer distribui os quadros do clip `reaction_*` por essa janela,
independentemente da duração física do stun. O primeiro quadro já é impacto.
A rajada de C++ usa nove frames de resposta por pancada, com agenda compartilhada
entre pose ofensiva e perfil do defensor. Queda, get-up, guarda e KO têm poses
próprias; `combat_manifest`, dano e caixas não dependem desses novos desenhos.

Para inspecionar as duas personagens juntas, tanto `--character cpp` quanto
`--character python` no showcase usam a outra como adversária. `X` troca os lados.
O exemplo `capture_pair_reactions` percorre os doze ataques e quatro situações
de defesa de cada personagem; `--supers-only` limita a passagem aos dois supers
e `--reverse` espelha o par. Seu JSON registra o perfil e o desenho a cada tick.

```sh
cargo run --example capture_pair_reactions -- --output /tmp/pair-reactions
cargo run --example capture_pair_reactions -- --reverse --output /tmp/pair-reactions-left
cargo run --example capture_pair_reactions -- --supers-only --no-snapshots --output /tmp/pair-supers --video /tmp/pair-supers.mp4
```

A matriz de testes por contato é complementada por revisão visual do par em
movimento, incluindo o início e a recuperação entre pancadas consecutivas.
A arte desse padrão para os demais personagens permanece fora desta rodada.
