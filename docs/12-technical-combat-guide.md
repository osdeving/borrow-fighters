# 12 — Guia Técnico de Combate

## Objetivo

Este documento ajuda devs e agentes de IA a encontrar rapidamente onde o combate vive no código, como testar golpes isolados e quais docs atualizar quando o sistema mudar.

Sempre que um código novo alterar combate, personagens, input de combate, Combat Lab, hitbox/hurtbox, projectile, frame data ou sprites ligados a golpes, atualize este guia ou explique no PR por que não foi necessário.

## Sistemas

| Sistema | Responsabilidade | Código principal | Testes |
|---|---|---|---|
| Combat runtime | Estado de lutador, movimento, defesa, ataque ativo, stun, dano e hurtbox | [`src/combat/fighter.rs`](../src/combat/fighter.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs) |
| Combat data | Frame data, dano, guard rule, hit reaction e hitbox dos golpes próximos | [`src/combat/move_data.rs`](../src/combat/move_data.rs) | [`tests/move_data.rs`](../tests/move_data.rs) |
| Move runtime | Enum runtime `AttackKind` e compatibilidade com `MoveSpec` | [`src/combat/move_set.rs`](../src/combat/move_set.rs) | [`tests/move_data.rs`](../tests/move_data.rs) |
| Projectile | Projétil horizontal, dano, guard rule, hit reaction, velocidade, spawn e timing do especial | [`src/combat/projectile.rs`](../src/combat/projectile.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs), [`tests/attack_frame_data.rs`](../tests/attack_frame_data.rs) |
| Collision | Interseção simples de retângulos | [`src/combat/collision.rs`](../src/combat/collision.rs), [`src/math/rect.rs`](../src/math/rect.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Character data | Registro de personagens e listas de golpes | [`src/characters/mod.rs`](../src/characters/mod.rs) | [`tests/characters.rs`](../tests/characters.rs) |
| Match runtime | Instancia lutadores a partir de personagens, bloqueia intro/contagem, resolve hits, projéteis e vitória | [`src/game/world.rs`](../src/game/world.rs) | [`tests/combat_rules.rs`](../tests/combat_rules.rs) |
| Arena runtime | Identidade e rotação de arenas do protótipo | [`src/game/arena.rs`](../src/game/arena.rs) | [`tests/arena_rotation.rs`](../tests/arena_rotation.rs) |
| Audio domain | Cues, eventos de gameplay, manifesto JSON e matching de bindings | [`src/audio/mod.rs`](../src/audio/mod.rs) | [`tests/audio_manifest.rs`](../tests/audio_manifest.rs) |
| Audio Raylib boundary | Carrega clips existentes e toca eventos resolvidos por manifesto | [`src/engine/audio.rs`](../src/engine/audio.rs) | Teste manual via jogo |
| Combat Lab state | Cena isolada para playback de golpes, pause, frame step e leitura de vantagem | [`src/scenes/combat_lab.rs`](../src/scenes/combat_lab.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab analysis | Cálculo de vantagem estimada, pushback e dummy de contato | [`src/scenes/combat_lab_analysis.rs`](../src/scenes/combat_lab_analysis.rs) | [`tests/combat_lab.rs`](../tests/combat_lab.rs) |
| Combat Lab render | Orquestra Raylib da cena isolada, sprites, grid e projéteis | [`src/engine/render/combat_lab.rs`](../src/engine/render/combat_lab.rs) | Teste manual via Combat Lab |
| Combat debug UI | Boxes, pivot, dummy, overlay e texto de timing do laboratório | [`src/ui/combat_debug.rs`](../src/ui/combat_debug.rs) | Teste manual via Combat Lab |
| Input | Teclado/gamepad para luta, preferências e Combat Lab | [`src/engine/input.rs`](../src/engine/input.rs), [`src/engine/gamepad.rs`](../src/engine/gamepad.rs) | [`tests/cli.rs`](../tests/cli.rs), [`tests/feature_flags.rs`](../tests/feature_flags.rs) |
| Sprite runtime | Manifest JSON, clip selection e desenho por pivot | [`src/engine/sprites/`](../src/engine/sprites) | [`tests/sprite_manifest.rs`](../tests/sprite_manifest.rs), [`tests/sprite_selection.rs`](../tests/sprite_selection.rs) |

## Técnica Atual

### Fluxo de Início de Luta

O início de luta fica em [`src/game/world.rs`](../src/game/world.rs), não no renderer. `World::new_greybox_with_intro` liga primeiro `spawn_intro_timer` para a entrada cinematográfica e também prepara `countdown_timer`.

Enquanto `spawn_intro_active` ou `countdown_active` estiverem ativos, `World::update_with_flags` atualiza apenas timers e feedback transitório; movimento, ataques, projéteis e IA não avançam gameplay. A contagem visual usa os labels `11`, `10`, `01` e `Fight!`, expostos por `World::countdown_label`. Os eventos de áudio correspondentes são `match.countdown.11`, `match.countdown.10`, `match.countdown.01` e `match.countdown.fight`.

O desenho da contagem fica em [`src/engine/render.rs`](../src/engine/render.rs), que só consulta `World::countdown_label`. A troca de arena é decisão de [`src/app.rs`](../src/app.rs): depois que `World::outcome` aparece, a arena atual permanece na pose de vitória e só avança quando uma nova luta é iniciada por restart ou pela tela de preferências.

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

Essa técnica foi escolhida porque é legível, testável sem Raylib e suficiente para o Prototype 0.1. Quando sprites finais exigirem precisão maior, a evolução provável é `BoxTimeline` por frame no manifesto ou em dados de personagem.

### Frame Data

O jogo usa fixed timestep de 60 FPS em [`src/config.rs`](../src/config.rs). A linguagem de tuning do combate deve ser frame, não segundo.

- `FrameCount` fica em [`src/combat/frame.rs`](../src/combat/frame.rs).
- Golpes próximos usam `AttackFrameData` e `whiff_recovery` dentro de `MoveSpec`.
- Projectile/special usa `ProjectileFrameData` em [`src/combat/projectile.rs`](../src/combat/projectile.rs).
- O Combat Lab mostra frame atual, fase e janela ativa.

### Defesa, Guard Rule e Stun

`GuardRule` e `HitReaction` ficam em [`src/combat/move_data.rs`](../src/combat/move_data.rs). O corte atual define a linguagem de defesa antes de implementar todos os tipos de golpe:

- `GuardRule::High`, `Mid`, `Low`, `Throw` e `Projectile` já existem como dados.
- Golpes próximos atuais usam `GuardRule::Mid`.
- Projéteis usam `PROJECTILE_GUARD_RULE`, hoje `GuardRule::Projectile`.
- `GuardRule::Throw` já é explicitamente não bloqueável, mas ainda não existe golpe de throw jogável.
- `Low` exige defesa com block + crouch, mas nenhum golpe baixo real foi ligado ainda.

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
- `RustBorrowJab`
- `DukeBoilerplatePoke`

`DEFAULT_CLOSE_RANGE_MOVE_IDS` define a lista padrão genérica usada por construtores e testes que não selecionam personagem. `CharacterSpec.move_ids` define o loadout real de cada personagem.

`Fighter` carrega `move_ids` próprios. Quando um botão de golpe é pressionado, `move_spec_for_input` procura no loadout o primeiro `MoveSpec` com o `MoveInputKind` correspondente. Se não houver `MoveId` compatível, o input daquele golpe não inicia ataque. Isso permite que o mesmo botão resolva para golpes diferentes por personagem sem alterar profundamente `Fighter`.

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
- `move_ids`: golpes próximos disponíveis no loadout.

Hoje `Rust` usa `RustBorrowJab` no soco fraco: startup menor, alcance menor e dano menor para funcionar como checagem rápida. `Duke` usa `DukeBoilerplatePoke` no soco forte: alcance e dano maiores, com startup/recovery mais longos para reforçar midrange pesado e punição em whiff. O `World` cria lutadores via `World::new_with_characters`, consumindo `CharacterSpec` para nome, vida e loadout. O Combat Lab usa o mesmo caminho para testar personagem isolado.

### Combat Lab

Abrir o laboratório:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --pose block
cargo run -- --lab combat --character duke --pose victory
```

Valores aceitos:

| Flag | Valores |
|---|---|
| `--character` | `rust`, `rustacean`, `duke`, `java` |
| `--move` | `light_punch`, `heavy_punch`, `kick`, `projectile` |
| `--pose` | `move`, `idle`, `crouch`, `jump`, `block`, `hit`, `victory` |

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
