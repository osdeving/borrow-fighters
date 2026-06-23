# 08 — Arquitetura de Código

## Status

Implementado em corte inicial.

Este documento descreve a arquitetura atual do protótipo Rust + Raylib e mantém algumas intenções futuras. A regra segue sendo evitar transformar o projeto em uma engine antes de provar o combate.

## Objetivo

Criar uma base simples, testável e extensível para o protótipo 0.1 sem transformar o projeto em uma engine antes de provar o combate.

## Referências de base

- [Cargo Book — Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html): `Cargo.toml` e `Cargo.lock` na raiz, código em `src/`, binário padrão em `src/main.rs`, biblioteca em `src/lib.rs`, exemplos em `examples/` e testes de integração em `tests/`.
- [The rustdoc book — How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html): documentação de crate/módulo com `//!`, documentação pública com `///`, e primeira frase curta e clara.
- [raylib-rs](https://github.com/deltaphc/raylib-rs): binding Rust para Raylib que mantém proximidade com a API C, mas com ajustes idiomáticos de Rust.
- [Game Programming Patterns — Game Loop](https://gameprogrammingpatterns.com/game-loop.html): separação entre input, update e render; fixed timestep como candidato para previsibilidade de combate.
- [Game Programming Patterns — Update Method](https://gameprogrammingpatterns.com/update-method.html): objetos/sistemas atualizados por frame quando houver simulação concorrente simples.

## Estrutura atual

```text
borrow-fighters/
├── Cargo.toml                  # Manifesto do pacote Rust
├── Cargo.lock                  # Deve subir: jogo/aplicação precisa de build reproduzível
├── src/
│   ├── main.rs                 # Binário fino: inicializa janela, cria App e roda loop
│   ├── lib.rs                  # Módulos testáveis e API interna do jogo
│   ├── app.rs                  # Orquestra estado global, loop e transições de alto nível
│   ├── cli.rs                  # Parser pequeno de argumentos de inicialização
│   ├── config.rs               # Constantes de janela, arena, escala e timestep
│   ├── audio/
│   │   └── mod.rs              # Eventos, manifesto e roteamento data-driven de áudio
│   ├── game/
│   │   ├── mod.rs              # Estado de partida e regras de fluxo
│   │   ├── arena.rs            # Identidade e rotação das arenas do protótipo
│   │   ├── ai.rs               # CPU simples para playtest
│   │   ├── feature_flags.rs    # Flags runtime para experimentos e menu Options
│   │   └── world.rs            # Estado jogável, intro/contagem e regras de partida
│   ├── engine/
│   │   ├── mod.rs              # Adaptadores finos em volta de Raylib
│   │   ├── audio.rs            # Boundary Raylib para carregar e tocar clips de áudio
│   │   ├── assets.rs           # Caminhos e carregamento de texturas
│   │   ├── gamepad.rs          # Mapeamento básico de gamepad
│   │   ├── input.rs            # Raylib keyboard/gamepad -> comandos do jogo
│   │   ├── render.rs           # Desenho de arena, HUD, debug, menu e lutadores
│   │   ├── video_capture.rs    # Captura local do framebuffer via ffmpeg
│   │   ├── render/
│   │   │   ├── combat_lab.rs   # Desenho da cena isolada de Combat Lab
│   │   │   └── sprite_viewer.rs # Desenho da ferramenta isolada de sprites
│   │   └── sprites/
│   │       ├── animation.rs    # Seleção de frame por duração
│   │       ├── combat.rs       # Projeção de frames[].combat para coordenadas de mundo
│   │       ├── draw.rs         # Desenho de atlas com pivot
│   │       ├── manifest.rs     # Leitura/validação de JSON de sprite
│   │       ├── mod.rs          # API do módulo de sprites
│   │       └── selection.rs    # Estado de lutador -> clip
│   ├── combat/
│   │   ├── mod.rs              # Contratos do sistema de combate
│   │   ├── fighter.rs          # Estado comum de lutador
│   │   ├── frame.rs            # Timing de combate em frames inteiros
│   │   ├── collision.rs        # Resolução hitbox x hurtbox
│   │   ├── move_data.rs        # Tabela MoveSpec dos golpes atuais
│   │   ├── move_set.rs         # Tipos runtime e compatibilidade com AttackKind
│   │   └── projectile.rs       # Estado de projéteis
│   ├── characters/
│   │   ├── mod.rs              # CharacterSpec e registro inicial de personagens
│   │   └── body_metrics.rs     # Manifesto de corpo fisico por personagem
│   ├── scenes/
│   │   ├── mod.rs              # Estados de tela
│   │   ├── combat_lab.rs       # Laboratório isolado para timing e boxes
│   │   ├── preferences.rs      # Cursor e navegação do menu principal/submenus
│   │   ├── sprite_viewer.rs    # Viewer testável de atlas, pivot e frame bounds
│   │   └── sprite_viewer/
│   │       └── combat_edit.rs  # Helpers puros para editar boxes do viewer
│   ├── ui/
│   │   ├── mod.rs              # API dos overlays de UI/debug
│   │   └── combat_debug.rs     # Overlay de boxes, pivot e timing do Combat Lab
│   └── math/
│       ├── mod.rs              # Tipos geométricos pequenos do jogo
│       ├── rect.rs             # Retângulos de colisão/hitbox
│       └── vec2.rs             # Vetores 2D se Raylib Vector2 não bastar
└── tests/
    ├── cli.rs                  # Contrato de argumentos de inicialização
    ├── characters.rs           # Contrato do registro de personagens
    ├── combat_lab.rs           # Estado testável do Combat Lab
    ├── attack_frame_data.rs    # Timing de golpes em frames
    ├── move_data.rs            # Contrato da tabela MoveSpec
    ├── character_identity_tuning.rs # Intenção mecânica de Rust/Duke/Go/C por dados
    ├── combat_rules.rs         # Regras puras de combate e IA
    ├── traditional_moves.rs    # High/low/throw e ataques aéreos tradicionais
    ├── cpu_traditional_moves.rs # Cobertura da CPU para golpes tradicionais
    ├── feature_flags.rs        # Contrato de flags runtime
    ├── audio_manifest.rs       # Contrato do manifesto e roteamento de áudio
    ├── sprite_manifest.rs      # Validação do formato JSON de sprites
    └── sprite_selection.rs     # Clip escolhido a partir do estado do lutador
```

O diretório `scenes/` ainda deve permanecer simples, sem framework de telas. `ui/` já abriga o overlay de debug do Combat Lab, mas ainda não deve virar um sistema genérico antes de haver HUD e menus suficientes para justificar isso. `characters/` já possui o registro mínimo de personagens, mas ainda deve permanecer simples e orientado a dados. Novos módulos só devem entrar quando reduzirem responsabilidade real dos arquivos atuais.

## Regras de fronteira

### `main.rs`

Deve ser fino. Responsabilidades:

- inicializar Raylib;
- carregar config inicial;
- criar `App`;
- rodar o loop;
- encaminhar encerramento.

Não deve concentrar regras de combate, input detalhado, colisão ou UI.

### `lib.rs`

Deve expor os módulos internos para testes e exemplos. Regras puras de jogo devem viver sob `lib.rs` sempre que possível para permitir `cargo test` sem abrir janela.

### `engine/*`

É a camada de adaptação com Raylib. Ela pode conhecer Raylib. O core de combate deve depender pouco ou nada de Raylib para ficar testável.

`src/engine/audio.rs` segue essa regra: ele carrega `Sound` e chama Raylib, mas recebe eventos e bindings já modelados pelo domínio de áudio.

`src/engine/video_capture.rs` tambem segue essa fronteira: ele pode conhecer ferramentas do host (`ffmpeg` e PulseAudio) e Raylib para leitura de framebuffer, mas gameplay e cenas só enxergam start/stop/status e envio do frame renderizado.

`src/engine/sprites/combat.rs` tambem fica em `engine` porque depende do formato de sprite, clip, pivot e escala visual. Ele nao depende de Raylib; apenas projeta metadata local do atlas para `Rect`/`Vec2` em coordenadas de mundo para que `game::World` possa usar com fallback.

### `combat/*`

Deve ser o núcleo mais estável do protótipo. Prioridade:

- dados simples;
- estados explícitos;
- colisão legível;
- efeitos previsíveis.

Evitar callback/event bus cedo demais.

Quando combate precisar de feedback sonoro, deve emitir ou encaminhar `AudioEvent` pelo match runtime em vez de tocar arquivo diretamente.

### `characters/*`

No começo, personagens devem ser dados e pequenas funções. `CharacterSpec` já alimenta `World`, `Combat Lab` e `Fighter` com nome, vida máxima e loadout de golpes próximos. Não criar sistema de plugins, scripting ou data-driven avançado antes de existir gameplay divertido.

Chaves estáveis para assets, como `CharacterId::audio_key`, devem ficar próximas do registro de personagem para evitar strings soltas.

`src/characters/body_metrics.rs` carrega [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json). Esse manifesto e o primeiro corte data-driven de corpo fisico por personagem, separado de arte e de hitbox de golpe.

### `audio/*`

É o domínio testável de áudio. Pode conhecer personagens e golpes por chave estável, mas não Raylib.

Responsabilidades:

- nomear cues de gameplay;
- carregar schema do manifesto;
- escolher o binding mais específico para um evento;
- manter clips ausentes como opção de pipeline, não como erro fatal de runtime.

Detalhes ficam em [`docs/14-audio-pipeline.md`](14-audio-pipeline.md) e [`docs/adr/0005-data-driven-audio-events.md`](adr/0005-data-driven-audio-events.md).

### `scenes/*`

Usar cenas simples para separar fluxo de tela sem criar framework pesado.

Ferramentas temporárias e plugáveis também podem entrar em `scenes/*` quando tiverem estado testável sem Raylib. O corte atual é `src/scenes/sprite_viewer.rs`, acionado por `--tool sprite-viewer`, enquanto o desenho fica em `src/engine/render/sprite_viewer.rs`. Esse modo não deve carregar `World`, áudio ou loop de luta normal.

### Feature flags runtime

Opções experimentais de gameplay, UI e input devem entrar por `src/game/feature_flags.rs`.

Regras:

- criar um novo `FeatureFlag`;
- definir label, descrição e default no mesmo módulo;
- consumir com `FeatureFlags::enabled`, `set` ou `toggle`;
- evitar booleans soltos em `App`, `World`, render ou IA;
- registrar ADR quando a flag virar decisão estrutural.

## Loop de jogo atual

Fluxo conceitual:

```text
read platform input
translate input to game commands
run fixed update step while needed
render current state
```

No protótipo 0.1:

- usar fixed update simples;
- limitar número máximo de updates por frame para evitar spiral of death;
- renderizar placeholders sem interpolação se isso reduzir risco;
- só adicionar interpolação quando movimento visual exigir.

## Convenções por arquivo

Todo arquivo Rust novo deve começar com uma descrição curta e declarar a qual sistema pertence:

```rust
//! Responsabilidade do módulo em uma frase.
//!
//! System: Nome do sistema maior. Explica qual motor/módulo possui este arquivo
//! e o que não pertence aqui.
```

Regras:

- primeira frase deve dizer o que o arquivo faz;
- a linha `System:` deve ajudar devs e IAs a localizar o módulo maior;
- evitar comentário óbvio em cada função;
- documentar itens públicos com `///`;
- se o arquivo tiver regra de domínio importante, registrar o porquê perto da regra;
- se uma decisão afetar vários arquivos, abrir ou atualizar ADR.
- se uma mudança alterar comandos, hotkeys, hitbox/hurtbox, frame data, personagens ou Combat Lab, atualizar [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md).

## O que evitar no começo

- ECS antes de haver necessidade real.
- Sistema de eventos genérico antes de uma dor concreta.
- Asset pipeline complexo.
- Scripting de personagens.
- Editor de fases.
- Múltiplos crates/workspace.
- Abstração própria de render que esconda Raylib cedo demais.

## Quando revisar esta arquitetura

Revisar se:

- o protótipo 0.1 provar o combate básico;
- hitbox/hurtbox exigirem ferramenta visual;
- personagens passarem a precisar de dados externos;
- testes de combate ficarem difíceis de escrever;
- Raylib começar a vazar para todo o domínio de gameplay.
