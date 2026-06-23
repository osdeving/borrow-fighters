# 12 — Guia Técnico de Combate

## Objetivo

Este documento ajuda devs e agentes de IA a encontrar rapidamente onde o combate vive no código, como testar golpes isolados e quais docs atualizar quando o sistema mudar.

Sempre que um código novo alterar combate, personagens, input de combate, Combat Lab, hitbox/hurtbox, projectile, frame data ou sprites ligados a golpes, atualize este guia ou explique no PR por que não foi necessário.

## Sistemas

| Sistema | Responsabilidade | Código principal | Testes |
|---|---|---|---|
| Combat runtime | Estado de lutador, movimento, defesa, ataque ativo, stun, dano e hurtbox | [`src/combat/fighter.rs`](../src/combat/fighter.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs), [`tests/traditional_moves.rs`](../tests/traditional_moves.rs) |
| Combat data | Frame data, dano, guard rule, hit reaction e hitbox dos golpes próximos | [`src/combat/move_data.rs`](../src/combat/move_data.rs) | [`tests/move_data.rs`](../tests/move_data.rs), [`tests/traditional_moves.rs`](../tests/traditional_moves.rs) |
| Move runtime | Enum runtime `AttackKind` e compatibilidade com `MoveSpec` | [`src/combat/move_set.rs`](../src/combat/move_set.rs) | [`tests/move_data.rs`](../tests/move_data.rs) |
| Projectile | Projétil horizontal, dano, guard rule, hit reaction, velocidade, spawn e timing do especial | [`src/combat/projectile.rs`](../src/combat/projectile.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs) |
| Collision | Interseção simples de retângulos | [`src/combat/collision.rs`](../src/combat/collision.rs), [`src/math/rect.rs`](../src/math/rect.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Character data | Registro de personagens, listas de golpes e identidade de loadout | [`src/characters/mod.rs`](../src/characters/mod.rs) | [`tests/characters.rs`](../tests/characters.rs), [`tests/character_identity_tuning.rs`](../tests/character_identity_tuning.rs) |
| Character body metrics | Manifesto data-driven para largura, altura em pe e altura abaixada | [`src/characters/body_metrics.rs`](../src/characters/body_metrics.rs), [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json) | [`tests/characters.rs`](../tests/characters.rs) |
| Match runtime | Instancia lutadores a partir de personagens, bloqueia intro/contagem, resolve hits, projéteis e vitória | [`src/game/world.rs`](../src/game/world.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Combat log | Eventos compactos de diagnóstico para reproduzir bugs de luta | [`src/game/combat_log.rs`](../src/game/combat_log.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| CPU playtest | Heurística determinística para mover, defender e exercitar golpes básicos/tradicionais | [`src/game/ai.rs`](../src/game/ai.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/cpu_traditional_moves.rs`](../tests/cpu_traditional_moves.rs) |
| Arena runtime | Identidade e rotação de arenas do protótipo | [`src/game/arena.rs`](../src/game/arena.rs) | [`tests/arena_rotation.rs`](../tests/arena_rotation.rs) |
| Audio domain | Cues, eventos de gameplay, manifesto JSON e matching de bindings | [`src/audio/mod.rs`](../src/audio/mod.rs) | [`tests/audio_manifest.rs`](../tests/audio_manifest.rs) |
| Audio Raylib boundary | Carrega clips existentes e toca eventos resolvidos por manifesto | [`src/engine/audio.rs`](../src/engine/audio.rs) | Teste manual via jogo |
| App scene state | Maquina de estados de alto nivel para menu, luta e laboratorios | [`src/app.rs`](../src/app.rs), [`src/scenes/mod.rs`](../src/scenes/mod.rs) | Smoke tests via CLI |
| Combat Lab state | Cena isolada para playback de golpes, pause, frame step e leitura de vantagem | [`src/scenes/combat_lab.rs`](../src/scenes/combat_lab.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab analysis | Cálculo de vantagem estimada, pushback e dummy de contato | [`src/scenes/combat_lab_analysis.rs`](../src/scenes/combat_lab_analysis.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab render | Orquestra Raylib da cena isolada, sprites, grid e projéteis | [`src/engine/render/combat_lab.rs`](../src/engine/render/combat_lab.rs) | Teste manual via Combat Lab |
| Combat debug UI | Boxes, pivot, dummy, overlay e texto de timing do laboratório | [`src/ui/combat_debug.rs`](../src/ui/combat_debug.rs) | Teste manual via Combat Lab |
| Sprite Combat Viewer | Ferramenta isolada para carregar atlas em runtime, ver grid, pivot, bounds e preparar boxes data-driven | [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), [`src/scenes/sprite_viewer/combat_edit.rs`](../src/scenes/sprite_viewer/combat_edit.rs), [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs) | [`tests/sprite_viewer.rs`](../tests/sprite_viewer.rs), teste manual via `--tool sprite-viewer` |
| Sprite Studio | App externo Tauri 1.8 + React para editar manifestos sem depender de Raylib | [`tools/sprite-studio`](../tools/sprite-studio) | `pnpm build`; `pnpm tauri build --debug`; desktop requer pre-requisitos Tauri |
| Input | Teclado/gamepad para luta, menu, Sprite Viewer e Combat Lab | [`src/engine/input.rs`](../src/engine/input.rs), [`src/engine/gamepad.rs`](../src/engine/gamepad.rs) | [`tests/cli.rs`](../tests/cli.rs), [`tests/feature_flags.rs`](../tests/feature_flags.rs) |
| Sprite runtime | Manifest JSON, clip selection, projeção de `frames[].combat` e desenho por pivot | [`src/engine/sprites/`](../src/engine/sprites), [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs) | [`tests/sprite_manifest.rs`](../tests/sprite_manifest.rs), [`tests/sprite_selection.rs`](../tests/sprite_selection.rs) |

## Técnica Atual

### Estado de Cenas

O loop principal em [`src/app.rs`](../src/app.rs) usa `AppScene` de [`src/scenes/mod.rs`](../src/scenes/mod.rs) como maquina de estados simples:

- `Preferences`: menu principal e submenus de versus, treino e opções;
- `Fight`: luta normal com fixed timestep, IA, audio events e renderer de arena;
- `CombatLab`: cena isolada para testar golpes e frame data;
- `SpriteViewer`: ferramenta de sprite em loop proprio, fora do fluxo normal de luta.

Transicoes novas devem passar por esse enum em vez de espalhar flags soltas no loop. Se a nova tela for ferramenta temporaria, prefira loop isolado como o Sprite Viewer; se fizer parte do jogo, trate como cena normal. `Esc` tem comportamento de voltar dentro do jogo; o bootstrap em [`src/main.rs`](../src/main.rs) desativa a tecla padrão de fechamento do Raylib com `set_exit_key(None)`.

### Fluxo de Início de Luta

O início de luta fica em [`src/game/world.rs`](../src/game/world.rs), não no renderer. `World::new_greybox_with_intro` liga primeiro `spawn_intro_timer` para a entrada cinematográfica e também prepara `countdown_timer`.

O matchup inicial vem de [`LaunchOptions.match_options`](../src/cli.rs), que aceita `--p1`/`--player-one` e `--p2`/`--player-two` para a luta normal. O submenu `Versus Setup` também pode ciclar Player 1 e Player 2 entre Rust, Duke, Go e C; [`App`](../src/app.rs) marca essa escolha como pendente e recria o mundo ao começar a próxima luta. `LaunchOptions.start_fight` vem de `--fight`/`--skip-menu` e permite iniciar direto em `AppScene::Fight`. [`App`](../src/app.rs) preserva essa escolha no primeiro mundo e em `restart_match`, chamando `World::new_greybox_with_intro_for_characters`.

Enquanto `spawn_intro_active` ou `countdown_active` estiverem ativos, `World::update_with_flags` atualiza apenas timers e feedback transitório; movimento, ataques, projéteis e IA não avançam gameplay. A contagem visual usa os labels `11`, `10`, `01` e `Fight!`, expostos por `World::countdown_label`. Os eventos de áudio correspondentes são `match.countdown.11`, `match.countdown.10`, `match.countdown.01` e `match.countdown.fight`.

O desenho da contagem fica em [`src/engine/render.rs`](../src/engine/render.rs), que só consulta `World::countdown_label`. A troca de arena é decisão de [`src/app.rs`](../src/app.rs): depois que `World::outcome` aparece, a arena atual permanece na pose de vitória e só avança quando uma nova luta é iniciada por restart ou pelo menu.

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

Essa técnica foi escolhida porque é legível, testável sem Raylib e suficiente para o Prototype 0.1. Quando o frame visual declara `frames[].combat`, o runtime projeta esses dados para coordenadas de mundo em [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs). A resolução da luta usa `frames[].combat.hitboxes[]` e `frames[].combat.hurtboxes[]` quando essas listas existem; se estiverem ausentes ou vazias, volta para `MoveSpec.hitbox` e `Fighter::hurtboxes()`. A decisão está registrada em [`docs/adr/0007-sprite-frame-combat-runtime.md`](adr/0007-sprite-frame-combat-runtime.md).

Rust, Duke, Go e C ja possuem `combat.projectile_origin` no primeiro frame do clip `special`. Esse ponto e projetado por [`src/engine/sprites/combat.rs`](../src/engine/sprites/combat.rs) e usado por [`src/game/world.rs`](../src/game/world.rs) ao criar o projectile, para evitar que o poder nasca desalinhado da mao. Rust `Borrow Jab`, heavy punch e kick ja possuem hitboxes de frame; os valores ainda reproduzem o alcance do `MoveSpec` para migrar com baixo risco. Hitboxes/hurtboxes restantes devem ser calibradas no Sprite Studio, com o Sprite Combat Viewer Raylib apenas como ferramenta temporaria ate a limpeza dedicada.

### Escala Visual e Pivot

Sprites runtime usam `borrow-fighters.sprite.v1` em [`src/engine/sprites/manifest.rs`](../src/engine/sprites/manifest.rs). O campo `scale` controla o tamanho visual do atlas em jogo; `frames[].pivot` ancora cada frame no corpo do lutador.

O desenho fica em [`src/engine/sprites/draw.rs`](../src/engine/sprites/draw.rs). O renderer consulta `manifest.scale` e multiplica frame e pivot pelo mesmo valor. Portanto, escala e pivot salvos no manifesto afetam luta, Combat Lab e Sprite Combat Viewer sem recompilar.

O corpo fisico base fica em [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json), carregado por [`src/characters/body_metrics.rs`](../src/characters/body_metrics.rs). Esse manifesto controla:

- `width`: largura do corpo para colisao corpo-corpo e hurtbox base;
- `standing_height`: altura em pe;
- `crouch_height`: altura abaixada.

`FighterBodyMetrics` e consumido por [`Fighter`](../src/combat/fighter.rs). Rust, Duke/Java, Go e C usam o corpo padrao `101,3 x 224 / crouch 128` neste corte, migrado da base anterior por `RESOLUTION_SCALE = 4 / 3`. Go continua nao-humano visualmente, mas o atlas foi normalizado para caber na mesma escala jogavel de Rust/C em vez de ganhar hurtbox larga por causa de proporcao de placeholder. Se o arquivo falhar ao carregar no app, o jogo usa os defaults do `CharacterSpec` e emite warning.

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

- `hitstun_timer`: interrompe ataque atual, troca clip para `hit` e impede iniciar ação.
- `blockstun_timer`: mantém o lutador em defesa e impede iniciar ação.
- ambos são expostos para debug/testes por `hitstun_remaining_frames`, `blockstun_remaining_frames`, `in_hitstun` e `in_blockstun`.
- `hit_pushback` e `block_pushback`: deslocamento horizontal em pixels aplicado ao defensor, com block pushback menor que hit pushback no tuning atual.

O match runtime em [`src/game/world.rs`](../src/game/world.rs) passa `guard_rule` e `hit_reaction` de `ActiveAttack` ou `Projectile` para o defensor. O próprio `World` aplica o pushback, porque é ele quem sabe de qual lado está atacante, defensor e projétil. Depois do deslocamento, `Fighter::clamp_to_arena` mantém o defensor dentro da arena. Feature flags de dano ainda impedem dano, stun e pushback quando desativadas.

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

`DEFAULT_CLOSE_RANGE_MOVE_IDS` define a lista padrão genérica usada por construtores e testes que não selecionam personagem. `CharacterSpec.move_ids` define o loadout real de cada personagem.

`Fighter` carrega `move_ids` próprios. Quando um botão de golpe é pressionado, `FighterInput::requested_move_spec` escolhe o `MoveInputKind` a partir de botão, direção, abaixar, defesa e estado aéreo. Depois `move_spec_for_input` procura no loadout o primeiro `MoveSpec` com aquele input. Se não houver `MoveId` compatível, o input daquele golpe não inicia ataque. Isso permite que o mesmo botão resolva para golpes diferentes por personagem sem alterar profundamente `Fighter`.

### Combat Log

O log de combate fica em [`src/game/combat_log.rs`](../src/game/combat_log.rs) e é preenchido por [`World`](../src/game/world.rs). Ele registra eventos compactos como início de round, countdown, ataque iniciado, whiff, hit/block resolvido, projectile disparado, projectile resolvido e fim de luta.

Use `World::combat_log()` em testes ou ferramentas de debug para inspecionar a sequência atual, e `World::clear_combat_log()` quando um teste quiser isolar uma janela específica. O log é limitado por `COMBAT_LOG_CAPACITY` para não crescer indefinidamente. Ele não substitui `AudioEvent`: áudio é feedback; `CombatLog` é rastreio técnico.

Mapeamento atual de input:

| Input | `MoveInputKind` |
|---|---|
| `F` / `X` | `LightPunch` |
| `H` / `Y` | `HeavyPunch` |
| `V` / `B` | `Kick` |
| Abaixar + chute | `Sweep` |
| Abaixar + soco forte | `AntiAir` |
| Frente + soco forte | `Overhead` |
| Defender + soco fraco | `Throw` |
| No ar + soco fraco/forte | `AirPunch` |
| No ar + chute | `AirKick` |

`AttackKind` em [`src/combat/move_set.rs`](../src/combat/move_set.rs) ainda existe como camada runtime de compatibilidade para sprites, debug e categorias visuais. O dano, a hitbox e o frame data durante uma luta vêm do `MoveSpec` concreto guardado no estado de ataque.

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

Hoje `Rust` usa `RustBorrowJab`, `RustLifetimeAntiAir` e `RustOwnershipThrow` para reforçar leitura técnica: golpes mais rápidos, menores e menos danosos. `Duke` usa `DukeBoilerplatePoke`, `DukeGarbageCollectorSweep`, `DukeAbstractFactoryOverhead` e `DukeEnterpriseThrow` para reforçar midrange pressure: mais alcance/dano, startup maior e whiff mais punível. `Go` usa `GoGoroutineJab`, `GoDeferKick`, `GoChannelOverhead` e `GoHopkick` para validar rushdown com atlas placeholder: menos vida, ações mais rápidas e alcance menor. `C` ja e selecionavel e possui atlas/projectile/entrada proprios, mas ainda usa loadout generico para validar tamanho, leitura e pipeline antes de receber identidade de frame data.

Os especiais de projectile ficam em [`src/combat/projectile.rs`](../src/combat/projectile.rs) como `RUST_PROJECTILE_SPEC`, `DUKE_PROJECTILE_SPEC`, `GO_PROJECTILE_SPEC` e `C_PROJECTILE_SPEC`. `Fighter::projectile_spec` alimenta `Projectile::from_fighter`, o Combat Lab e o overlay técnico, então alterar um spec muda luta real e lab no mesmo caminho.

`World::new_with_characters` e `World::new_greybox_with_intro_for_characters` aceitam qualquer `CharacterId`; a luta padrão ainda instancia Rust x Duke, mas o submenu `Versus Setup` e `--p1`/`--p2` permitem testar Go e C em match real.

A intenção de gameplay por golpe vive em [`docs/15-character-combat-matrix.md`](15-character-combat-matrix.md). Atualize essa matriz quando alterar frame data, alcance, dano, guard rule, projectile ou loadout de personagem.

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
cargo run -- --lab combat --character c --move projectile
cargo run -- --lab combat --character rust --pose block
cargo run -- --lab combat --character duke --pose victory
```

O mesmo laboratório também pode ser aberto pelo menu principal em `Training -> Combat Lab`. Nesse fluxo, `Esc` volta ao menu sem fechar a janela.

Valores aceitos:

| Flag | Valores |
|---|---|
| `--fight`, `--skip-menu` | sem valor; inicia direto na luta normal |
| `--p1`, `--player-one` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang` |
| `--p2`, `--player-two` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang` |
| `--character` | `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang` |
| `--move` | `light_punch`, `heavy_punch`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick`, `throw`, `projectile` |
| `--pose` | `move`, `idle`, `crouch`, `jump`, `block`, `hit`, `victory` |
| `--tool` | `sprite-viewer` |
| `--manifest` | caminho para um JSON `borrow-fighters.sprite.v1` |
| `--clip` | nome de clip presente no manifesto |

`--pose move` é o modo padrão e reproduz o golpe selecionado por `--move`. As outras poses são inspeções estáticas para alinhar sprite, pivot e hurtbox sem depender de uma luta real.

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
- `victory`: força clip visual `taunt`.

### Sprite Combat Viewer

O Sprite Combat Viewer Raylib e ferramenta temporaria. A direcao aprovada em [`docs/adr/0008-external-sprite-studio-tooling.md`](adr/0008-external-sprite-studio-tooling.md) e mover a edicao rica para [`tools/sprite-studio`](../tools/sprite-studio). A paridade operacional ja existe; o viewer embutido deve ser removido em uma mudanca propria.

Abrir a ferramenta isolada de sprites:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
cargo run -- --tool sprite-viewer --manifest assets/placeholder/duke-fighter.sprite.json --clip special --character duke --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/c-fighter.sprite.json --clip special --character c --move projectile
```

O viewer roda fora do loop normal de luta. Pela CLI, [`src/app.rs`](../src/app.rs) desvia para esse modo antes de carregar `GameAssets` e áudio. Pelo menu, `Training -> Sprite Viewer` abre o mesmo loop com o atlas de C em `special/projectile` como ponto de partida, e `Esc` volta ao menu. O estado testável fica em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), e o desenho Raylib fica em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs).

`--character` e `--move` ativam a camada runtime de combate no viewer. Sem `--character`, o viewer tenta inferir Rust, Duke, Go ou C pelo nome do manifesto. Essa camada usa `CharacterSpec`, `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`, então ela reflete os dados de combate atuais.

O viewer tambem entende metadata opcional `frames[].combat` no manifesto. Essa metadata e projetada para tela em [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs), desenhada em [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs), e validada em [`tests/sprite_manifest.rs`](../tests/sprite_manifest.rs). As coordenadas ficam em pixels locais do frame do atlas:

```json
"combat": {
  "hurtboxes": [{ "x": 10, "y": 8, "w": 48, "h": 96, "label": "body" }],
  "hitboxes": [{ "x": 62, "y": 38, "w": 28, "h": 22, "label": "strike" }],
  "projectile_origin": { "x": 84, "y": 44 }
}
```

`frames[].combat` ja pode alimentar a luta real de forma incremental. Quando um frame possui `hitboxes`, elas substituem a hitbox ofensiva calculada por `MoveSpec` naquele frame. Quando um frame possui `hurtboxes`, elas substituem as hurtboxes compostas de `Fighter`. Quando o clip `special` possui `projectile_origin`, o projectile nasce desse ponto projetado para o mundo. Campos ausentes mantem fallback para `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`, entao personagens sem metadata continuam jogaveis.

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

O personagem e o golpe podem ser trocados em runtime com `C`/`Shift+C` e `[`/`]`, sem reabrir o comando. `Enter` tenta selecionar o clip mais provável para o golpe atual. `F5` recarrega manifesto e atlas para iteração com ferramenta externa aberta; `F12` salva screenshot em `target/sprite-viewer-capture.png` para anexar em PR/issue. A evolução restante está rastreada em [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) e na issue [#15](https://github.com/osdeving/borrow-fighters/issues/15).

## Captura de Gameplay

Atalhos globais:

- `F9`: inicia gravação local da janela atual;
- `F10`: para a gravação e salva o MP4 em `captures/`.
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

Esse hook inicia a gravação automaticamente, para depois do número de segundos informado e usa o mesmo pipeline de `F9`/`F10`: render texture do Raylib, áudio PulseAudio e saída em `captures/`.

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
