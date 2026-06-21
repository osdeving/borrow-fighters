# 14 — Pipeline Técnico de Áudio

## Status

Implementado em corte inicial.

O projeto já possui um motor leve de áudio por eventos, manifesto JSON, música via stream e integração com Raylib. Os arquivos sonoros atuais são assets CC0 de protótipo, ainda sem direção final de mixagem.

## Objetivo

Preparar o jogo para vozes, dor, golpes, defesa, whiff, projéteis, vitória, UI e ambientes sem espalhar chamadas diretas de áudio pelo código.

O fluxo desejado é:

```text
gameplay/UI gera AudioEvent
AudioBank encontra binding mais específico
AudioPlayer toca clip carregado via Raylib
App troca faixa MusicTrack conforme cena
```

## Pesquisa e Decisão

Referências usadas:

- [raylib cheatsheet](https://www.raylib.com/cheatsheet/cheatsheet.html): a camada atual usa `InitAudioDevice`, `LoadSound`, `PlaySound`, `LoadMusicStream`, `PlayMusicStream` e `UpdateMusicStream` via raylib-rs.
- [FMOD Studio — Authoring Events](https://www.fmod.com/docs/2.03/studio/authoring-events.html): middlewares modernos organizam áudio como eventos acionáveis, não como arquivos tocados aleatoriamente pelo jogo.
- [FMOD Studio — Concepts / Parameters](https://www.fmod.com/docs/2.03/studio/fmod-studio-concepts.html): parâmetros permitem que o jogo module eventos conforme contexto.
- [Wwise — Understanding Events](https://www.audiokinetic.com/en/public-library/2025.1.8_9170/?id=understanding_events&source=WwiseFundamentalApproach): eventos são a unidade que o jogo dispara para dirigir áudio.
- [Wwise — Understanding SoundBanks](https://www.audiokinetic.com/fr/public-library/2025.1.7_9143/?id=understanding_soundbanks&source=WwiseFundamentalApproach): bancos agrupam dados e eventos de áudio para carregamento.

Decisão para o Prototype 0.1:

- não integrar FMOD/Wwise ainda;
- usar Raylib para tocar `Sound` curto;
- usar `Music`/stream para faixa de menu e combate;
- modelar o jogo com `AudioEvent`, `AudioCue` e manifesto JSON;
- manter clips ausentes como opcionais quando forem planejamento, mas marcar música de showcase como obrigatória;
- associar áudio por cue, personagem, golpe e futuramente ambiente;
- deixar volume, pitch, pan e bus no manifesto para evitar ajustes hardcoded.

## Onde Fica

| Parte | Arquivo | Responsabilidade |
|---|---|---|
| Domínio de áudio | [`src/audio/mod.rs`](../src/audio/mod.rs) | Define `AudioCue`, `AudioEvent`, schema do manifesto e matching de bindings. |
| Boundary Raylib | [`src/engine/audio.rs`](../src/engine/audio.rs) | Inicializa banco, carrega clips existentes e chama Raylib para tocar. |
| Emissão de eventos | [`src/game/world.rs`](../src/game/world.rs) | Emite eventos de contagem pré-luta, ataque, hit, block, dor, projectile e vitória. |
| Loop do app | [`src/app.rs`](../src/app.rs) | Inicializa dispositivo de áudio, troca música por cena, toca feedback de menu e drena eventos do `World`. |
| Manifesto | [`assets/audio/audio_manifest.json`](../assets/audio/audio_manifest.json) | Roteia eventos de gameplay para clips. |
| Convenções de assets | [`assets/audio/README.md`](../assets/audio/README.md) | Explica pastas, buses e placeholders. |
| Fontes/licenças | [`assets/audio/ATTRIBUTION.md`](../assets/audio/ATTRIBUTION.md) | Registra fontes CC0 dos assets baixados. |
| Testes | [`tests/audio_manifest.rs`](../tests/audio_manifest.rs), [`tests/combat_rules.rs`](../tests/combat_rules.rs) | Validam manifesto e eventos emitidos pelo combate. |

## Eventos Atuais

| Cue | Contexto esperado |
|---|---|
| `ui.navigate` | movimento de cursor no menu |
| `ui.confirm` | confirmar, alternar opção ou iniciar luta |
| `ui.back` | voltar da luta para preferências |
| `match.start` | cue genérico reservado para início de luta |
| `match.countdown.11` | primeira etapa da contagem visual binária |
| `match.countdown.10` | segunda etapa da contagem visual binária |
| `match.countdown.01` | terceira etapa da contagem visual binária |
| `match.countdown.fight` | liberação da luta após a contagem |
| `match.victory` | primeiro frame em que uma vitória é resolvida |
| `fighter.attack.start` | início de golpe próximo |
| `fighter.attack.whiff` | golpe próximo terminou sem contato |
| `fighter.projectile.cast` | especial/projétil foi disparado |
| `fighter.hurt` | personagem recebeu dano real |
| `fighter.block` | personagem bloqueou hit ou projectile |
| `combat.hit` | impacto físico de golpe próximo |
| `combat.block` | impacto físico em defesa |
| `projectile.impact` | impacto de projétil |

## Música Atual

| Track | Quando toca | Arquivo |
|---|---|---|
| `menu` | tela de preferências/menu | `assets/audio/music/menu-loop.ogg` |
| `combat` | luta e Combat Lab | `assets/audio/music/combat-loop.ogg` |

Música usa `Music` streaming do Raylib, não `Sound`. Por isso [`App`](../src/app.rs) chama `AudioPlayer::update_streams` a cada frame.

Cada evento pode carregar:

- `slot`: Player 1 ou Player 2;
- `character`: `rust`, `duke`, `go` ou aliases aceitos;
- `move`: `light_punch`, `heavy_punch`, `kick`, `rust_borrow_jab`, `duke_boilerplate_poke`, `go_goroutine_jab`;
- `environment`: reservado para arena, ainda não emitido pelo runtime.

A contagem pré-luta é emitida pelo `World`, não pelo menu. A tela mostra `11`, `10`, `01`, `Fight!`, enquanto os clips atuais usam voz CC0 de "three", "two", "one" e "fight" para manter leitura auditiva imediata.

## Manifesto

Exemplo reduzido:

```json
{
  "version": 1,
  "clips": [
    {
      "id": "voice.rust.attack.borrow_jab.01",
      "file": "assets/audio/characters/rust/voice/attack-borrow-jab-01.wav",
      "bus": "voice",
      "volume": 0.9
    }
  ],
  "music": [
    {
      "id": "combat",
      "file": "assets/audio/music/combat-loop.ogg",
      "volume": 0.42,
      "looping": true
    }
  ],
  "bindings": [
    {
      "cue": "fighter.attack.start",
      "character": "rust",
      "move": "rust_borrow_jab",
      "clips": ["voice.rust.attack.borrow_jab.01"]
    }
  ]
}
```

Campos de `clips`:

- `id`: chave única usada por bindings;
- `file`: caminho relativo ao repositório;
- `bus`: `sfx`, `voice`, `music`, `ui` ou outro grupo acordado;
- `volume`: 0.0 a 1.0;
- `pitch`: 1.0 é normal;
- `pan`: 0.5 é centro;
- `required`: quando `true`, clip ausente gera warning explícito.

Campos de `music`:

- `id`: `menu` ou `combat`;
- `file`: caminho relativo ao repositório;
- `volume`: 0.0 a 1.0;
- `pitch`: 1.0 é normal;
- `looping`: `true` para faixa de fundo;
- `required`: quando `true`, faixa ausente gera warning explícito.

Campos de `bindings`:

- `cue`: evento de gameplay;
- `character`: opcional;
- `move`: opcional;
- `environment`: opcional;
- `clips`: lista de clips candidatos.

O binding mais específico vence. Por exemplo, `fighter.attack.start + rust + rust_borrow_jab` vence um binding genérico só com `fighter.attack.start`.

Quando houver vários clips no mesmo binding, o player alterna entre os clips carregados para evitar repetição idêntica.

## Como Adicionar um Som

1. Coloque o arquivo no diretório sugerido em [`assets/audio/README.md`](../assets/audio/README.md).
2. Adicione uma entrada em `clips`.
3. Adicione ou ajuste um `binding`.
4. Rode:

```bash
cargo test --all-targets
```

5. Teste manualmente:

```bash
cargo run
```

Clips opcionais não quebram o jogo se o arquivo não existir. Os assets atuais já existem no repositório e são validados por teste.

## Regras Para Código Novo

- Gameplay deve emitir `AudioEvent`, não chamar Raylib.
- UI pode tocar eventos de feedback pelo `App`, sem conhecer Raylib.
- `src/audio/mod.rs` não deve depender de Raylib.
- `src/engine/audio.rs` pode conhecer Raylib, mas não deve conhecer regras internas de combate.
- Se adicionar novo golpe ou personagem, registre a chave estável em `MoveId::audio_key` ou `CharacterId::audio_key`.
- Se adicionar nova cue, atualize `AudioCue`, `assets/audio/audio_manifest.json`, este documento e testes.
- Se adicionar música nova, registre `MusicTrack`, o manifesto e a transição de cena correspondente.
- Se baixar asset externo, atualize [`assets/audio/ATTRIBUTION.md`](../assets/audio/ATTRIBUTION.md).
- Se uma arena precisar áudio próprio, use `environment` no binding em vez de checar nome de arena no código de combate.

## Próximos Cortes

- Adicionar controle de volume por bus na tela de preferências.
- Adicionar `environment` real quando houver seleção de arena.
- Adicionar cooldown de voz se clips repetirem demais em multi-hit.
- Criar teste de lint para garantir que todo binding referencia clip existente no manifesto.
- Trocar placeholders CC0 por direção sonora própria quando houver áudio original.
- Avaliar middleware dedicado somente quando Raylib deixar de cobrir mistura, estados, bancos ou authoring.
