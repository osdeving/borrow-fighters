# 14 — Pipeline Técnico de Áudio

## Status

Implementado em corte inicial.

O projeto já possui um motor leve de áudio por eventos, manifesto JSON, música via stream, controle de volume de música no menu e integração com Raylib. Os arquivos sonoros são assets CC0 para UI, impactos, música, contagem e vozes dos lutadores. Old C reutiliza temporariamente a voz de Rust por autorização do usuário; Duke/Java, Go, Python e C++ conservam suas gravações. As cinco sequências autorais também possuem entradas sonoras e efeitos sincronizados por fase, com pausa real da música.

## Objetivo

Preparar o jogo para vozes, dor, golpes, defesa, whiff, projéteis, vitória, UI e ambientes sem espalhar chamadas diretas de áudio pelo código.

O fluxo desejado é:

```text
gameplay/UI gera AudioEvent
AudioBank encontra binding mais específico
AudioPlayer toca clip carregado via Raylib
App troca faixa MusicTrack conforme cena e volume de música
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
| Loop do app | [`src/app.rs`](../src/app.rs) | Inicializa dispositivo de áudio, troca música por cena, aplica volume de música, toca feedback de menu e drena eventos do `World`. |
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
| `super.start` | entrada com voz do atacante e efeito específico; substitui `fighter.attack.start` nos cinco supers autorais |
| `super.trash_rain` | papel, copos e resíduos caem na sequência de Duke |
| `super.collect` | coleta curta pelos clones de Duke; três variações |
| `super.giant_drop` | queda do Duke gigante |
| `super.mutation` | materialização de Rust ou transformação/crescimento da Python; duas variações próprias de cada personagem |
| `super.error` | cascata de erros de Old C e trecho herdado pela C++ |
| `super.boot` | sinais de POST/BIOS antes de restaurar a arena |
| `super.footshot` | disparo cômico da C++ |
| `super.barrage_hit` | camada rápida de deslocamento/impacto da rajada de C++; três variações |
| `super.typing` / `super.enter` | digitação curta no notebook da C++ (frame 25) e confirmação (130) |
| `super.lunge` / `super.swallow` | bote (270) e deglutição (304) da Python |
| `super.revert` | retorno da serpente para Python (344) |
| `super.celebrate` / `super.peace` | salto feliz (400) e gesto de paz (444) da Python |
| `super.end` | encerramento discreto da sequência autoral |

## Música Atual

| Track | Quando toca | Arquivo |
|---|---|---|
| `menu` | tela de preferências/menu | `assets/audio/music/menu-loop.ogg` |
| `combat` | luta em `Sirius Light Ring` | `assets/audio/music/combat-loop.ogg` |
| `combat-random-encounter` | luta em `Tech Coast Beacon` | `assets/audio/music/combat-random-encounter.ogg` |
| `combat-console-floor` | luta em `Java Street Terminal` | `assets/audio/music/combat-console-floor.ogg` |
| `combat-rins-theme` | luta em `BioTIC Garden` | `assets/audio/music/combat-rins-theme.ogg` |
| `combat-chiptune-battle` | luta em `Porto Digital Cache` | `assets/audio/music/combat-chiptune-battle.ogg` |
| `combat-8bit-battle` | luta em `Pinhao Smart Grid` | `assets/audio/music/combat-8bit-battle.ogg` |
| `combat-determined-pursuit` | `Combat Lab` | `assets/audio/music/combat-determined-pursuit.ogg` |

Música usa `Music` streaming do Raylib, não `Sound`. Por isso [`App`](../src/app.rs) chama `AudioPlayer::update_streams` a cada frame. O app troca a faixa em transições de tela e quando uma nova luta muda de arena, e `Options > Music Volume` aplica um multiplicador global apenas sobre música.

Na entrada de uma sequência autoral, o app chama `AudioPlayer::set_cinematic_paused(true)` antes de tocar `super.start`. O player interrompe uma única vez os sons anteriores e chama `pause_stream` na música. Os próximos cues continuam tocando; `update_streams` e chamadas repetidas de `play_music` não avançam nem reiniciam a faixa pausada. Ao terminar normalmente, `set_cinematic_paused(false)` chama `resume_stream`, preservando a posição da faixa e permitindo terminar o último efeito. Reinício e mudanças de cena/personagem usam `cancel_cinematic()`, que também interrompe os efeitos da sequência abortada antes de retomar a música. `Esc`/`Start` abre a pausa da luta: `set_match_paused(true)` preserva os cursores dos sons e da música; continuar usa `set_match_paused(false)` e mantém a pausa musical do cinematográfico, se ele ainda estiver ativo. Reiniciar, trocar personagens ou ir ao menu cancela a sequência antiga antes de liberar a pausa da luta. Essa pausa é independente do ducking usado em outros contextos. A mutação de Rust troca a faixa para Sirius no frame 125 mantendo a pausa; a nova faixa começa no cursor zero apenas ao terminar a sequência. Reiniciar a luta restaura a música da arena selecionada. O mixer de evidência [`mix_authored_super_review.py`](../tools/audio/mix_authored_super_review.py) aplica os mesmos eventos `arena_transitions` exportados no JSON.

Cada evento pode carregar:

- `slot`: Player 1 ou Player 2;
- `character`: `rust`, `duke`, `go`, `c`, `python`, `cpp` ou aliases aceitos;
- `move`: `light_punch`, `heavy_punch`, `kick`, `rust_borrow_jab`, `duke_boilerplate_poke`, `go_goroutine_jab`, `go_defer_kick`, `c_pointer_jab`, `c_unsafe_poke`, `python_snake_bite`, `python_data_strike`, `python_heel_kick` e demais chaves estáveis de `MoveId::audio_key`;
- `environment`: reservado para arena, ainda não emitido pelo runtime.

A contagem pré-luta é emitida pelo `World`, não pelo menu. A tela mostra `11`, `10`, `01`, `Fight!`, enquanto os clips atuais usam voz CC0 de "three", "two", "one" e "fight" para manter leitura auditiva imediata.

As vozes de ataque possuem bindings específicos para golpes de identidade e fallback do próprio personagem para os demais golpes próximos. Bindings com várias opções alternam gravações para reduzir repetição. C++ agora tem arquivos próprios e não usa vozes de C ou Python.

| Lutador | Direção e gravação |
|---|---|
| Rust | Esforço jovem de aventureiro, por Brandon Song / wolfwoot; voz no pitch original. |
| Duke / Java | Pacote anterior preservado, incluindo arquivos e parâmetros do manifesto. |
| Old C | Fallback temporário autorizado para seis clips de Rust/Brandon Song, copiados sem alteração; esforços de 0,26–0,74 s. A atuação anterior de Volvion saiu do runtime. |
| Go | Vocalizações de criatura de Ogrebane, com articulação encurtada; fonte diferente de todas as vozes humanas. |
| Python | Pacote anterior de cicifyre preservado, incluindo arquivos e parâmetros. |
| C++ | Esforços de SkyRae e reações de AuraVoice; ambos diferentes da intérprete de Python. |

As páginas originais, licenças e créditos estão em [ATTRIBUTION](../assets/audio/ATTRIBUTION.md). O [registro de produção original](../assets/audio/production-2026-09-09.json) documenta hashes SHA256 das gravações, recortes, filtros, mixagens, saídas e preservação de Duke/Python. O [registro atual de Old C](../assets/audio/production-old-c-fallback-2026-09-09.json) substitui somente as seis vozes de C e a camada vocal da entrada do seu super; mantém o manifesto, 87 outros arquivos de áudio e os efeitos Kenney de erro/glitch. O compartilhamento C/Rust é uma exceção explícita de playtest, com a procedência compartilhada registrada.

[`tests/audio_manifest.rs`](../tests/audio_manifest.rs) garante que os loadouts resolvem voz, que os bindings apontam para arquivos existentes no diretório do personagem e que somente C/Rust compartilham gravações. Também confere as seis cópias exatas e todos os cues de sequência. A [comparação de Old C](../assets/audio/review/old-c-fallback-2026-09-09/README.md) oferece um reel antes/depois de 18,25 s e candidatos CC0 não adotados. A busca identificou locuções Kenney e caricaturas de xathien; não houve audição subjetiva pelo agente porque a ferramenta rejeitou entrada de áudio. O fallback usa a alternativa de Rust já autorizada, enquanto uma voz exclusiva melhor depende de revisão humana.

A [página geral de audição](../assets/audio/audition.html) permite comparar os arquivos atuais isolados. Seu reel original permanece identificado como histórico anterior ao fallback de C.

## Manifesto

Exemplo reduzido:

```json
{
  "version": 1,
  "clips": [
    {
      "id": "voice.rust.attack.borrow_jab.01",
      "file": "assets/audio/characters/rust/voice/attack-borrow-jab-01.ogg",
      "bus": "voice",
      "volume": 0.42
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

Nota de calibração: o manifesto e o wrapper Rust documentam `pan=0.5` como centro, mas o `raudio.c` empacotado em raylib 6 usa −1..1, com centro em 0. O player atual repassa o valor do manifesto diretamente. Esta rodada preserva o runtime e os parâmetros aprovados; a mixagem da evidência reproduz essa chamada e a lei de pan nativa. Uma calibração futura deve alinhar essas convenções antes de alterar a imagem estéreo.

Campos de `music`:

- `id`: chave estável conhecida por `MusicTrack`;
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

No runtime, a fila de eventos do `World` é drenada com `drain(..)` para reter capacidade entre fixed steps. O `AudioPlayer` resolve bindings por índice, alterna clips sem clonar listas ou ids no hot path e só reaplica ducking de música quando o estado muda.

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

Para verificar pausa e retomada contra um dispositivo real, sem API de depuração no player de produção:

```bash
BORROW_AUDIO_REVIEW_OUTPUT=docs/evidence/authored-supers/audio-stream-review.json \
  cargo test --lib live_music_pause_resume_and_cancel -- --ignored --nocapture
```

Este teste manual consulta `get_time_played`, verifica cues durante a pausa, cancela a sequência e mede a retomada das faixas usadas por luta, Combat Lab e Move Showcase. Também troca a faixa para Menu enquanto pausado. O [registro da execução](evidence/authored-supers/audio-stream-review.json) confirmou cursor estável durante a pausa e avanço após retomada/cancelamento. Trata-se do player real e das chamadas usadas nas cenas; a navegação do app e a sequência visual são verificadas separadamente pelo [harness de captura](../examples/capture_authored_super_review.rs).


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

- Adicionar controle separado por bus de SFX/voice na tela de preferências.
- Adicionar `environment` real quando houver seleção de arena.
- Adicionar cooldown de voz se clips repetirem demais em multi-hit.
- Trocar placeholders CC0 por direção sonora própria quando houver áudio original.
- Avaliar middleware dedicado somente quando Raylib deixar de cobrir mistura, estados, bancos ou authoring.

## Cinematográficos adicionais

O cinematográfico local de Go mantém os bindings anteriores de ataque e impacto.
Rust, Duke, Old C, C++ e Python usam os cues `super.*` acima, emitidos pelo relógio
puro em [`src/game/world/supers.rs`](../src/game/world/supers.rs). A entrada já
mistura a voz com o efeito do personagem, evitando duas vozes sobrepostas.
`combat.hit`, `fighter.hurt` e os equivalentes de defesa acompanham cada contato
de dano real, incluindo o chip; os efeitos decorativos não aplicam dano.

Ao receber `SuperStart`, o player reinicia os cursores dos bindings `super.*`.
Assim, repetir um super interrompido mantém a ordem de sons definida pelo roteiro,
incluindo transformação antes de crescimento de Python. Os cursores das vozes
comuns permanecem independentes. O teste de regressão em
[`src/engine/audio.rs`](../src/engine/audio.rs) cobre interrupção e replay;
o [teste com stream real](evidence/reactions-transformations/audio-stream-review.json)
verifica também a troca pausada de Java Street para Sirius e a restauração por reset.

Os sons de lixo combinam papel, pequenos metais e impactos úmidos; a coleta alterna três recortes curtos. A queda gigante usa impacto grave; Rust usa metal e lâminas reverberantes; Old C usa erros digitais e POST; C++ usa disparo seco e camadas curtas de rajada. São recortes e mixagens de gravações CC0, sem fala gerada. Na rodada 23 foram produzidos 25 arquivos de voz novos/substituídos e 18 efeitos de sequência, Ogg Vorbis mono 48 kHz, normalizados antes da codificação a −3,48 dBFS e com fades curtos. A validação das saídas decodificadas verificou ausência de clipping, amostras inválidas e arquivos vazios. A seleção subjetiva permanece disponível para audição humana no reel.

A rodada 24 acrescenta dez efeitos: digitação e confirmação do notebook de C++,
entrada de Python e fases de transformação, crescimento, bote, deglutição,
reversão, salto e paz. Os arquivos e bindings das vozes aprovadas de Duke/Python
permanecem preservados; a entrada de Python reutiliza sua voz existente na nova
mixagem. A [procedência adicional](../assets/audio/production-transformations-2026-09-09.json)
identifica os recortes e arquivos produzidos.

Os roteiros e tempos atuais estão em [Reações e transformações](24-reactions-and-transformations.md);
a [rodada anterior](23-authored-super-sequences.md) preserva o histórico dos quatro
primeiros roteiros. Os nomes e comandos ficam no
[guia técnico de combate](12-technical-combat-guide.md#especiais-cinematográficos-adicionais).
