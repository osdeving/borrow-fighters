# 08 — Arquitetura de Código

## Status

Proposto. Ainda não há código Rust de produção.

Este documento define o esboço inicial para quando o projeto sair da fase somente documental e criar o primeiro scaffold Rust + Raylib.

## Objetivo

Criar uma base simples, testável e extensível para o protótipo 0.1 sem transformar o projeto em uma engine antes de provar o combate.

## Referências de base

- [Cargo Book — Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html): `Cargo.toml` e `Cargo.lock` na raiz, código em `src/`, binário padrão em `src/main.rs`, biblioteca em `src/lib.rs`, exemplos em `examples/` e testes de integração em `tests/`.
- [The rustdoc book — How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html): documentação de crate/módulo com `//!`, documentação pública com `///`, e primeira frase curta e clara.
- [raylib-rs](https://github.com/deltaphc/raylib-rs): binding Rust para Raylib que mantém proximidade com a API C, mas com ajustes idiomáticos de Rust.
- [Game Programming Patterns — Game Loop](https://gameprogrammingpatterns.com/game-loop.html): separação entre input, update e render; fixed timestep como candidato para previsibilidade de combate.
- [Game Programming Patterns — Update Method](https://gameprogrammingpatterns.com/update-method.html): objetos/sistemas atualizados por frame quando houver simulação concorrente simples.

## Estrutura proposta

```text
borrow-fighters/
├── Cargo.toml                  # Futuro manifesto do pacote Rust
├── Cargo.lock                  # Deve subir: jogo/aplicação precisa de build reproduzível
├── src/
│   ├── main.rs                 # Binário fino: inicializa janela, cria App e roda loop
│   ├── lib.rs                  # Módulos testáveis e API interna do jogo
│   ├── app.rs                  # Orquestra estado global, loop e transições de alto nível
│   ├── config.rs               # Constantes de janela, timestep e debug flags
│   ├── game/
│   │   ├── mod.rs              # Estado de partida e regras de fluxo
│   │   ├── feature_flags.rs    # Flags runtime para experimentos e preferências
│   │   ├── match_state.rs      # Round, timer, vitória, reinício
│   │   └── world.rs            # Estado jogável mínimo do protótipo
│   ├── engine/
│   │   ├── mod.rs              # Adaptadores finos em volta de Raylib
│   │   ├── clock.rs            # Delta time, fixed timestep e frame pacing
│   │   ├── input.rs            # Raylib keyboard/gamepad -> comandos do jogo
│   │   ├── render.rs           # Desenho de primitives/placeholders
│   │   ├── sprites.rs          # Spritesheets placeholder e seleção de frames
│   │   ├── assets.rs           # Carregamento futuro de sprites/som
│   │   └── debug_draw.rs       # Hitbox/hurtbox, eixos e overlays
│   ├── combat/
│   │   ├── mod.rs              # Contratos do sistema de combate
│   │   ├── fighter.rs          # Estado comum de lutador
│   │   ├── hitbox.rs           # Áreas ofensivas temporárias
│   │   ├── hurtbox.rs          # Áreas vulneráveis
│   │   ├── collision.rs        # Resolução hitbox x hurtbox
│   │   └── damage.rs           # Dano, hitstun, knockback futuro
│   ├── characters/
│   │   ├── mod.rs              # Registro dos personagens disponíveis
│   │   ├── rustacean.rs        # Futuro personagem Rust
│   │   └── duke.rs             # Futuro personagem Java
│   ├── scenes/
│   │   ├── mod.rs              # Estados de tela
│   │   ├── boot.rs             # Inicialização e carregamento mínimo
│   │   ├── preferences.rs      # Tela de ajustes e feature flags de playtest
│   │   ├── fight.rs            # Cena jogável principal
│   │   └── victory.rs          # Resultado e reinício
│   ├── ui/
│   │   ├── mod.rs              # Elementos de UI
│   │   └── hud.rs              # Barra de vida, timer e debug labels
│   └── math/
│       ├── mod.rs              # Tipos geométricos pequenos do jogo
│       ├── rect.rs             # Retângulos de colisão/hitbox
│       └── vec2.rs             # Vetores 2D se Raylib Vector2 não bastar
├── examples/
│   └── sandbox.rs              # Futuro exemplo isolado para testar sistemas
└── tests/
    └── combat_rules.rs         # Futuro teste de regras puras de combate
```

Os arquivos acima são intenção arquitetural, não obrigação imediata. No protótipo 0.1, criar apenas os módulos necessários para fazer dois placeholders lutarem.

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

### `combat/*`

Deve ser o núcleo mais estável do protótipo. Prioridade:

- dados simples;
- estados explícitos;
- colisão legível;
- efeitos previsíveis.

Evitar callback/event bus cedo demais.

### `characters/*`

No começo, personagens podem ser dados e pequenas funções. Não criar sistema de plugins, scripting ou data-driven avançado antes de existir gameplay divertido.

### `scenes/*`

Usar cenas simples para separar fluxo de tela sem criar framework pesado.

### Feature flags runtime

Opções experimentais de gameplay, UI e input devem entrar por `src/game/feature_flags.rs`.

Regras:

- criar um novo `FeatureFlag`;
- definir label, descrição e default no mesmo módulo;
- consumir com `FeatureFlags::enabled`, `set` ou `toggle`;
- evitar booleans soltos em `App`, `World`, render ou IA;
- registrar ADR quando a flag virar decisão estrutural.

## Loop de jogo proposto

Fluxo conceitual:

```text
read platform input
translate input to game commands
run fixed update step while needed
render current state
```

Para o protótipo 0.1:

- começar com fixed update simples;
- limitar número máximo de updates por frame para evitar spiral of death;
- renderizar placeholders sem interpolação se isso reduzir risco;
- só adicionar interpolação quando movimento visual exigir.

## Convenções por arquivo

Todo arquivo Rust novo deve começar com uma descrição curta:

```rust
//! Responsabilidade do módulo em uma frase.
//!
//! Explicar aqui apenas o contexto que ajuda alguém a decidir se este é o
//! arquivo certo para editar.
```

Regras:

- primeira frase deve dizer o que o arquivo faz;
- evitar comentário óbvio em cada função;
- documentar itens públicos com `///`;
- se o arquivo tiver regra de domínio importante, registrar o porquê perto da regra;
- se uma decisão afetar vários arquivos, abrir ou atualizar ADR.

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
