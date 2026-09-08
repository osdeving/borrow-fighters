# Borrow Fighters

Jogo 2D de luta com humor de programação, iniciado como um projeto **docs-first** e agora com um protótipo greybox jogável em Rust + Raylib.

Status: **Prototype 0.1 / Greybox jogável / Vertical slice em evolução**

## Objetivo

Este repositório centraliza documentação, governança, assets placeholder e código do primeiro protótipo jogável.

A ideia continua sendo evoluir com decisões explícitas, escopo controlado e colaboração aberta entre programação, game design e arte.

## Índice central

### Visão e produto

- [`docs/00-vision.md`](docs/00-vision.md): visão do jogo.
- [`docs/01-mini-gdd.md`](docs/01-mini-gdd.md): Mini-GDD inicial.
- [`docs/02-prototype-scope.md`](docs/02-prototype-scope.md): escopo do primeiro protótipo.
- [`docs/03-backlog.md`](docs/03-backlog.md): backlog inicial e t-shirt sizing.
- [`docs/04-team-briefing.md`](docs/04-team-briefing.md): briefing para reunir colaboradores.
- [`docs/10-greybox-playtest.md`](docs/10-greybox-playtest.md): como testar o primeiro protótipo greybox.
- [`docs/21-signature-spectacle-and-throws.md`](docs/21-signature-spectacle-and-throws.md): arremessos, reações aéreas e os cinco especiais de assinatura.
- [`docs/20-mvp-combat-showcase.md`](docs/20-mvp-combat-showcase.md): execução e critérios da rodada de MVP, showcase contextual e especiais.
- [`docs/12-worldbuilding.md`](docs/12-worldbuilding.md): história, personagens e arenas brasileiras.

### Governança, contribuição e release

- [`CONTRIBUTING.md`](CONTRIBUTING.md): guia prático para contribuir agora.
- [`docs/05-governance.md`](docs/05-governance.md): regras de PR, branches, labels, papéis, squads e decisões.
- [`docs/06-release-process.md`](docs/06-release-process.md): sistema de release, tags, milestones e checklist.
- [`CHANGELOG.md`](CHANGELOG.md): histórico de mudanças relevantes.

### Arte, mood e moldes

- [`docs/07-art-direction.md`](docs/07-art-direction.md): direção de arte inicial, moods e critérios visuais.
- [`docs/11-sprite-pipeline.md`](docs/11-sprite-pipeline.md): formato candidato para atlas, animações, pivots e metadata de sprites.
- [`docs/16-sprite-combat-viewer-roadmap.md`](docs/16-sprite-combat-viewer-roadmap.md): roadmap do viewer para artistas conferirem atlas, pivot, grade e boxes.
- [`docs/17-visual-scale-and-stage-metrics.md`](docs/17-visual-scale-and-stage-metrics.md): escala visual alvo de personagens, arena e workflow de calibracao.
- [`docs/18-sprite-studio.md`](docs/18-sprite-studio.md): ferramenta Tauri + React para editar manifestos e atlas fora do loop do jogo.
- [`docs/19-sprite-production-coverage.md`](docs/19-sprite-production-coverage.md): matriz de ações, referências, candidatos e verificações da produção de sprites.
- [`docs/templates/mood-proposal.md`](docs/templates/mood-proposal.md): molde para proposta de moodboard.
- [`docs/templates/character-concept.md`](docs/templates/character-concept.md): molde para personagem e mecânica.
- [`docs/templates/adr-template.md`](docs/templates/adr-template.md): molde para novas decisões.
- [`docs/templates/release-checklist.md`](docs/templates/release-checklist.md): checklist de release.

### Código e IA

- [`docs/08-code-architecture.md`](docs/08-code-architecture.md): esboço da arquitetura Rust + Raylib.
- [`docs/11-sprite-pipeline.md`](docs/11-sprite-pipeline.md): ponte entre assets de artistas e futuro motor de sprites.
- [`docs/12-technical-combat-guide.md`](docs/12-technical-combat-guide.md): guia técnico de combate, hitbox/hurtbox, Combat Lab e rastreio de código.
- [`docs/13-combat-design-roadmap.md`](docs/13-combat-design-roadmap.md): plano técnico para golpes, balanceamento e Combat Lab.
- [`docs/14-audio-pipeline.md`](docs/14-audio-pipeline.md): motor de áudio por eventos, manifesto JSON e convenções de clips.
- [`docs/15-character-combat-matrix.md`](docs/15-character-combat-matrix.md): matriz de identidade mecânica e tuning inicial de Rust, Duke, Go, C, Python e C++.
- [`docs/16-sprite-combat-viewer-roadmap.md`](docs/16-sprite-combat-viewer-roadmap.md): ferramenta isolada para inspecionar sprites e preparar hitbox/hurtbox data-driven.
- [`docs/17-visual-scale-and-stage-metrics.md`](docs/17-visual-scale-and-stage-metrics.md): padrao tecnico de tamanho em tela, escala de sprite e largura de arena.
- [`docs/18-sprite-studio.md`](docs/18-sprite-studio.md): app desktop externo para artistas editarem `*.sprite.json` com UI propria.
- [`docs/09-ai-collaboration.md`](docs/09-ai-collaboration.md): como Codex, Claude e skills devem navegar o projeto.
- [`AGENTS.md`](AGENTS.md): instruções persistentes para Codex.
- [`CLAUDE.md`](CLAUDE.md): instruções persistentes para Claude Code.
- [`.agents/skills/`](.agents/skills): skills repo-local para Codex.
- [`.claude/skills/`](.claude/skills): skills de projeto para Claude Code.

### Decisões registradas

- [`docs/adr/0001-stack-rust-raylib.md`](docs/adr/0001-stack-rust-raylib.md): decisão inicial de stack.
- [`docs/adr/0002-version-control-workflow.md`](docs/adr/0002-version-control-workflow.md): fluxo de branches, PRs e commits.
- [`docs/adr/0003-code-architecture-rust-raylib.md`](docs/adr/0003-code-architecture-rust-raylib.md): arquitetura inicial de código Rust + Raylib.
- [`docs/adr/0004-runtime-feature-flags-and-preferences.md`](docs/adr/0004-runtime-feature-flags-and-preferences.md): feature flags runtime e tela de preferências.
- [`docs/adr/0005-data-driven-audio-events.md`](docs/adr/0005-data-driven-audio-events.md): eventos de áudio data-driven com manifesto JSON.
- [`docs/adr/0006-runtime-sprite-scale-and-scene-state.md`](docs/adr/0006-runtime-sprite-scale-and-scene-state.md): escala visual por manifesto e maquina de estados de cenas.
- [`docs/adr/0007-sprite-frame-combat-runtime.md`](docs/adr/0007-sprite-frame-combat-runtime.md): metadata de hitbox/hurtbox por frame no runtime.
- [`docs/adr/0008-external-sprite-studio-tooling.md`](docs/adr/0008-external-sprite-studio-tooling.md): Sprite Studio externo em Tauri + React, isolado do codigo do jogo.
- [`docs/adr/0009-multi-image-sprite-manifests.md`](docs/adr/0009-multi-image-sprite-manifests.md): manifests de sprite podem compor um personagem a partir de mais de um atlas.
- [`docs/adr/0010-reviewed-action-sprite-production.md`](docs/adr/0010-reviewed-action-sprite-production.md): produção por ação, exportação explícita e revisão de candidatos sem alterar combate.
- [`docs/adr/0011-reviewed-art-default.md`](docs/adr/0011-reviewed-art-default.md): seleção dos conjuntos revisados por padrão, mantendo comparação e fallback.
- [`docs/adr/0012-shared-menu-pointer-layout.md`](docs/adr/0012-shared-menu-pointer-layout.md): geometria compartilhada entre desenho e navegação por mouse.
- [`docs/adr/0014-throws-launches-and-signature-effects.md`](docs/adr/0014-throws-launches-and-signature-effects.md): captura, lançamento e efeitos físicos de assinatura.
- [`docs/adr/0013-contextual-showcase-and-mvp-combat.md`](docs/adr/0013-contextual-showcase-and-mvp-combat.md): showcase com combate real, especiais e recuperação de queda.

### GitHub

- [`.github/PULL_REQUEST_TEMPLATE.md`](.github/PULL_REQUEST_TEMPLATE.md): template padrão de PR.
- [`.github/ISSUE_TEMPLATE/`](.github/ISSUE_TEMPLATE): templates de issues.
- [`.github/CODEOWNERS`](.github/CODEOWNERS): molde de donos de código/docs/assets.
- [`.github/release.yml`](.github/release.yml): categorias de release notes.
- [`.github/workflows/docs-check.yml`](.github/workflows/docs-check.yml): validação leve de docs e YAML.
- [`.github/workflows/pr-title.yml`](.github/workflows/pr-title.yml): validação de título de PR como Conventional Commit.
- [`.github/workflows/rust-check.yml`](.github/workflows/rust-check.yml): validação Rust com fmt, testes e clippy.

## Nome provisório

**Borrow Fighters** é um working title. O nome pode mudar conforme identidade visual, escopo e tom do jogo evoluírem.

## Amostra atual

[![Cinco especiais, arremesso e gancho no combate real](assets/showcase/signature-spectacle-cover.jpg)](assets/showcase/signature-spectacle-2026-09-08.mp4)

_Clique para ver os cinco especiais, o arremesso com troca de lados e o gancho: 21 segundos, 1280×720 a 60 fps, captura sem áudio do showcase real. [Produção e verificação](docs/21-signature-spectacle-and-throws.md)._

O [clipe original do greybox](assets/showcase/prototype-0.1-greybox.mp4) permanece como histórico.

## Como contribuir

Leia primeiro:

1. [`docs/00-vision.md`](docs/00-vision.md)
2. [`docs/01-mini-gdd.md`](docs/01-mini-gdd.md)
3. [`docs/03-backlog.md`](docs/03-backlog.md)
4. [`CONTRIBUTING.md`](CONTRIBUTING.md)
5. [`docs/05-governance.md`](docs/05-governance.md)

O documento [`docs/03-backlog.md`](docs/03-backlog.md) e a fonte de verdade dos proximos passos. Ele mantem a tabela **Agora / Proximo / Depois** e deve ser atualizado quando uma frente ativa muda, conclui ou entra no fluxo de PR.

Neste estágio, contribuições devem focar em:

1. clareza da visão;
2. redução de escopo;
3. mecânica central de luta;
4. identidade dos personagens;
5. sprites, animações, cenários e feedback visual;
6. decisões técnicas reversíveis.

## Como o GitHub deve ser usado

- Ideias pequenas entram como issue.
- Mudanças de documentação, processo, arte ou decisão entram por PR.
- Decisões estruturais entram como ADR.
- Milestones agrupam escopo de release.
- `main` deve ser protegida no GitHub antes do primeiro trabalho colaborativo real.

As regras propostas estão em [`docs/05-governance.md`](docs/05-governance.md).

## Rodando o protótipo greybox

O código jogável atual implementa um greybox local para validar o básico: menu principal com submenus de versus, treino, lore/roster e opções, arenas brasileiras em rotação começando pelo Sirius e trocando apenas no início da próxima luta, seleção manual de arena, livro de história carregado de JSON, intro cinematográfica com contagem `11` / `10` / `01` / `Fight!`, personagens com atlas de ações revisadas e placeholders preservados, movimento, pulo diagonal, abaixar, defesa, soco fraco, soco forte, chute, varredura, overhead, anti-air com lançamento, arremesso com troca de lados, ataques aéreos, fireball, cinco especiais de assinatura, queda com recuperação protegida, primeira identidade mecânica de Rust, Duke/Java, Go, C, Python e C++ por frame data, demo pública ciclando Rust, Duke/Java, C, Python e C++ sem Go no menu, CPU de playtest para um ou dois jogadores, colisão corpo-corpo, hitbox/hurtbox opcional, dano, stun, pushback, whiff recovery, hitspark, block pulse, trail de projétil, luz de chão em hitstun/blockstun, scanline/glow e animações leves de fundo por arena, vida, vitória e restart.

O runtime também já está preparado para áudio por eventos. O manifesto fica em [`assets/audio/audio_manifest.json`](assets/audio/audio_manifest.json), e o guia técnico fica em [`docs/14-audio-pipeline.md`](docs/14-audio-pipeline.md). O pacote inicial inclui SFX/UI/vozes de anúncio, contagem pré-luta, vozes de golpe por personagem com cobertura específica para Rust e Duke/Java, e músicas de menu, Combat Lab e arenas com fontes CC0 registradas em [`assets/audio/ATTRIBUTION.md`](assets/audio/ATTRIBUTION.md). O volume global da música pode ser ajustado em `Options`.

Requisitos iniciais:

- Rust estável.
- Dependências nativas exigidas por Raylib/raylib-rs no sistema operacional.

Comandos:

```bash
cargo run
cargo run -- --fight --p1 go --p2 duke
cargo run -- --player-one rust --player-two go
cargo run -- --fight --p1 c --p2 rust
cargo run -- --fight --p1 python --p2 duke
cargo run -- --fight --p1 cpp --p2 c
```

O jogo abre primeiro no menu principal. Use `Setas` ou `W/S` para navegar, `Enter` ou `Espaço` para confirmar, `A/D` ou `←`/`→` para trocar personagem, arena, capítulo, ficha de roster e volume em linhas ajustáveis, e `Esc` para voltar de submenus, luta, Move Showcase, Combat Lab ou Sprite Viewer. O mouse também navega pelos menus: passe sobre uma linha para selecioná-la, clique com o botão esquerdo para confirmar, alternar uma opção ou avançar um valor, e use o botão direito para voltar valores de personagem, arena, capítulo e volume. O cursor nativo permanece visível e livre para sair da janela; em WSL, o cursor `Linker` acompanha o mouse apenas enquanto a janela está em foco e o ponteiro está dentro dela. `Esc` não fecha mais a janela; para sair, use `Exit` ou o botão de fechar da janela.

O menu principal mantém a primeira tela simples:

- `Quick Fight`: inicia a luta com a configuração atual.
- `Versus Setup`: escolhe Player 1, Player 2 e arena.
- `Training`: abre `Move Showcase`, `Combat Lab` ou `Sprite Viewer`.
- `Lore / Roster`: abre um livro de programação com capítulos da história e fichas dos personagens.
- `Options`: liga/desliga gravação local e feature flags de protótipo.

Ao iniciar uma luta, o jogo roda a entrada dos personagens e depois bloqueia input durante a contagem central `11`, `10`, `01`, `Fight!`. A arena só avança para a próxima rotação quando uma nova luta é iniciada depois de uma vitória, para preservar a pose final no mesmo cenário.

Por padrão, a luta normal inicia `rust.rs` contra `duke.java` no `Sirius Light Ring` em Campinas, SP. O submenu `Versus Setup` permite ciclar Player 1 e Player 2 entre rust.rs, duke.java, old.c, python.py e cpp.cpp, e escolher a arena pelo nome/contexto/local. Go/Gopher continua no repositório, no CLI, no Combat Lab e no Sprite Viewer, mas saiu da seleção pública da demo por enquanto. Para testar matchups direto por CLI, use `--p1`/`--player-one` e `--p2`/`--player-two` com `rust`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx` ou `cpp.cpp`. Adicione `--fight` ou `--skip-menu` para entrar direto na luta sem passar pelo menu. Rust, Duke/Java, Go, C, Python e C++ já possuem vida, loadout, frame data, voz de ataque e projectile próprios; C joga como fundamentos de alcance/risco, Python como punisher ágil de dano moderado e C++ como herdeira técnica entre alcance de C e ritmo de Python.

O submenu `Lore / Roster` lê [`assets/lore/story.json`](assets/lore/story.json) em runtime. Edite esse arquivo para alterar capítulos, perfis, objetivos ou notas de personagem sem recompilar o jogo; reinicie o processo para recarregar o JSON. Com `CHAPTER` ou `CHARACTER` selecionado, use a roda do mouse ou `PageUp` / `PageDown` para rolar textos longos no capítulo ou na ficha. Os retratos atuais do roster são cards placeholder derivados dos sprites jogáveis.

Para abrir o laboratório de combate direto em uma cena limpa:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move sweep
cargo run -- --lab combat --character duke --move throw
cargo run -- --lab combat --character go --move kick
cargo run -- --lab combat --character c --move projectile
cargo run -- --lab combat --character python --move light_punch
cargo run -- --lab combat --character cpp --move projectile
cargo run -- --lab combat --character rust --pose block
cargo run -- --lab combat --character rust --pose crouch_block
cargo run -- --lab combat --character rust --pose spawn
cargo run -- --lab combat --character rust --pose defeat
```

No Combat Lab, use `Tab` / `Shift+Tab` para alternar golpe, `PageDown` / `PageUp` para alternar pose, `Enter` para repetir, `Espaço` para pausar, `.` para avançar 1 frame quando pausado, `Home` para voltar ao frame 0, `H` para hurtbox, `B` para hitbox, `P` para pivot/eixos, `D` para dummy de contato, `A` para mostrar/esconder o fundo de arena e `Esc` para voltar ao menu quando aberto pelo submenu `Training`. O overlay mostra frame data, vantagem estimada, pushback, whiff recovery e distância após pushback. Valores aceitos em `--character`: `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx` ou `cpp.cpp`. Valores aceitos em `--move`: `light_punch`, `heavy_punch`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick`, `throw`, `projectile` e `signature_special`. Valores aceitos em `--pose`: `move`, `idle`, `crouch`, `jump`, `block`, `hit`, `victory`, `spawn`, `defeat` e `crouch_block`. As poses mantêm o corpo parado para inspeção e reproduzem o clip com pause, avanço por frame e reinício.

O `Move Showcase`, em `Training`, usa o personagem escolhido como Player 1 contra um adversário real. Cada um dos cinco personagens da demo tem 15 situações: os dez golpes anteriores, um especial de assinatura e quatro exemplos de defesa. O adversário se aproxima, salta para receber anti-air ou mantém a guarda apropriada para demonstrar rasteira, overhead e agarrão. O resultado mostra o dano ou bloqueio calculado pelo combate. Cada cena dura 260 frames para incluir voo, queda e recuperação; o painel inferior deixa o espaço aéreo visível. Agarrões capturam e arremessam para o outro lado, e ganchos lançam a vítima. Os especiais são Borrow Fortress, System.out.println!, Segmentation Fault, import antigravity e Undefined Bazooka, sempre disponíveis com `T`/Backslash/`RT`, sem medidor.

```bash
cargo run -- --showcase --character rust --move anti_air --repeat
cargo run -- --showcase --character duke --move throw
cargo run -- --showcase --character python --move signature_special --repeat --reverse
```

`Tab` / `Shift+Tab` troca situação; `Enter` repete; `Espaço` pausa; `.` avança um frame; `Home` reinicia; `L` alterna repetição contínua; `X` troca lados; `PageUp` / `PageDown` troca personagem; `Esc` volta ao menu. [Guia de treino e defesa](docs/10-greybox-playtest.md#move-showcase) e [evidência de balanceamento](docs/evidence/mvp-balance/README.md).

Os atlas revisados completos são a apresentação padrão; os placeholders permanecem em disco como referência e fallback. Para abrir a arte nova ou comparar a anterior:

```bash
cargo run -- --fight --p1 rust --p2 duke
cargo run -- --lab combat --character rust --pose victory
BORROW_FIGHTERS_SPRITE_CANDIDATES=0 cargo run -- --fight --p1 rust --p2 duke
```

O carregador procura `assets/candidates/<personagem>/<personagem>-fighter.sprite.json` e exige os 20 clips anteriores mais `signature_special`, `knockdown`, `heavy_hit`, `launched` e `thrown` para os cinco personagens da demo; Go conserva os 20 clips anteriores; arquivo inválido, incompleto ou ausente mantém o placeholder daquele personagem. Conjuntos parciais são inspecionados diretamente no Sprite Viewer/Studio. A variável com valor `1` continua escolhendo os novos atlas explicitamente; `0` ou um valor inválido seleciona os originais. A arte revisada controla a apresentação; `combat_manifest` preserva as boxes e origens baseline dos golpes anteriores. Rasteiras usam as boxes do `MoveSpec`; especiais usam as entidades físicas de `World.signature_effects`, e ações baixas usam hurtboxes físicas agachadas, conforme a [ADR 0013](docs/adr/0013-contextual-showcase-and-mvp-combat.md). Reações, defesa, agachamento, salto e poses finais têm relógios visuais próprios. Veja [pipeline e comandos](docs/11-sprite-pipeline.md#revisao-de-candidatos-no-runtime) e [cobertura e pendências](docs/19-sprite-production-coverage.md).

Para abrir o viewer de sprites direto em uma ferramenta isolada:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
cargo run -- --tool sprite-viewer --manifest assets/placeholder/duke-fighter.sprite.json --clip special --character duke --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/c-fighter.sprite.json --clip special --character c --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/python-fighter.sprite.json --clip punch_light --character python --move light_punch
```

No Sprite Combat Viewer, use o mouse para inspecionar coordenadas locais do frame, arrastar personagem/dummy e ajustar alças de `frames[].combat`. `N` gera um rascunho de metadata a partir do overlay runtime do golpe selecionado, `Tab` / `Shift+Tab` alterna clip, `Enter` sincroniza clip com golpe, `C` / `Shift+C` alterna personagem de combate, `[` / `]` alterna golpe, `.` / `,` avança ou volta frame, `Espaço` pausa, mouse wheel controla zoom, `0` reseta zoom, `=` / `-` ajusta `scale`, `Setas` ou `Shift+Setas` move o `pivot`, `Ctrl+Setas` ajusta largura/altura do corpo físico, `Ctrl+Shift+Setas` ajusta altura abaixada, `Ctrl+S` salva manifestos de tuning, `O` mostra/esconde dummy, `M` mostra/esconde boxes de combate, `T` mostra/esconde trajetória prevista do projectile, `F5` recarrega manifesto/atlas, `F12` salva screenshot em `target/sprite-viewer-capture.png`, `F9`/`F10` gravam um MP4 local, `G` alterna grade, `P` alterna pivot, `B` alterna bounds, `R` reseta posição e `Esc` volta ao menu quando aberto por `Training`. O padrão de escala fica em [`docs/17-visual-scale-and-stage-metrics.md`](docs/17-visual-scale-and-stage-metrics.md), e o roadmap completo fica em [`docs/16-sprite-combat-viewer-roadmap.md`](docs/16-sprite-combat-viewer-roadmap.md).

O novo Sprite Studio externo vive em `tools/sprite-studio` e deve substituir o viewer Raylib em uma limpeza propria:

```bash
cd tools/sprite-studio
pnpm install
pnpm build
pnpm tauri dev
```

Ele usa Tauri 1.8 + React, edita `*.sprite.json` por UI propria, possui file picker nativo, menu desktop, timeline, paineis colapsaveis, tutorial visual, autosave/backup, snap, guias de escala, presets de combat boxes, validacao do runtime e export de review. Detalhes e pre-requisitos ficam em [`docs/18-sprite-studio.md`](docs/18-sprite-studio.md).

Configurações disponíveis em `Versus Setup` e `Options`:

| Preferência | Padrão | Efeito |
|---|---|---|
| Personagem Player 1 | rust.rs | Define o personagem do Player 1 na próxima luta. |
| Personagem Player 2 | duke.java | Define o personagem do Player 2 na próxima luta. |
| Arena | Sirius Light Ring / Campinas, SP | Define o cenário da próxima luta sem esperar a rotação automática. |
| Volume da música | 50% | Ajusta apenas a música de fundo em passos de 10%. |
| Player 1 usa IA | Ligado | Controla o Player 1 automaticamente. |
| Player 2 usa IA | Ligado | Controla o Player 2 automaticamente. |
| IA pode dar golpes | Ligado | Quando desligado, a IA ainda anda, pula, afasta, aproxima e defende, mas não ataca. |
| Player 1 recebe dano | Ligado | Quando desligado, o Player 1 fica invencível para playtest. |
| Player 2 recebe dano | Ligado | Quando desligado, o Player 2 fica invencível para playtest. |
| Mostrar HUD | Ligado | Exibe vida e título no topo. |
| Mostrar ajuda de controles | Desligado | Exibe comandos no rodapé durante a luta. |
| Mostrar debug de combate | Desligado | Exibe hitboxes, hurtboxes, labels e colisão corpo-corpo. |
| Entrada por gamepad | Ligado | Usa controles detectados pelo Raylib quando disponíveis. |

Defesa: segure `Q`/`U` para bloquear médios, overheads e ataques aéreos em pé; acrescente baixo para bloquear rasteiras. Agarrões vencem guarda, mas erram contra saltos. Rasteiras derrubam no lugar; agarrões arremessam, enquanto ganchos e os especiais de C, Python e C++ lançam a vítima; a recuperação protegida dura 36 frames e permite voltar a agir sem receber golpes enquanto caído. Chip reduz vida até o mínimo de 1.

Controles:

| Ação | Rust / Player 1 | Java / Player 2 | Gamepad Xbox |
|---|---|---|---|
| Mover | `A` / `D` | `←` / `→` ou `J` / `L` | Left stick ou D-pad |
| Pular | `W` | `↑` ou `I` | `A` |
| Abaixar | `S` | `↓` ou `K` | Left stick para baixo ou D-pad baixo |
| Defender | `Q` | `U` | `LB` ou `LT` |
| Soco fraco / curto | `F` | `O` ou `Enter` | `X` |
| Soco forte / longo | `H` | `P` ou `Right Shift` | `Y` |
| Chute | `V` | `;` ou `/` | `B` |
| Varredura baixa | `S` + `V` | `↓`/`K` + `;`/`/` | Baixo + `B` |
| Anti-air | `S` + `H` | `↓`/`K` + `P`/`Right Shift` | Baixo + `Y` |
| Overhead | Frente + `H` | Frente + `P`/`Right Shift` | Frente + `Y` |
| Agarrão curto | `Q` + `F` | `U` + `O`/`Enter` | `LB`/`LT` + `X` |
| Ataque aéreo | No ar: `F` ou `V` | No ar: `O`/`Enter` ou `;`/`/` | No ar: `X` ou `B` |
| Fireball / projétil | `G` | `Right Ctrl` ou `KP0` | `RB` |
| Especial de assinatura | `T` | `\` (Backslash) | `RT` |
| Alternar P2 CPU/manual | `C` | `C` | `View` |
| Reiniciar | `R` | `R` | `Menu` |
| Gravar captura local | `F9` inicia / `F10` para | `F9` / `F10` | - |

O primeiro gamepad conectado controla o Player 1 quando a IA do Player 1 estiver desligada. O segundo gamepad controla o Player 2 quando a IA do Player 2 estiver desligada. Player 1 e Player 2 começam em modo CPU; use `Options` para alternar a IA do Player 1 e `C` ou `View` para alternar CPU/manual do Player 2 durante a luta.

Quando ambos os jogadores usam IA, Rust e Java usam perfis diferentes para evitar movimentos espelhados: um tende a jogar mais em média distância e o outro pressiona mais de perto. A IA anda, pula, bloqueia, soca, chuta e tenta o kit completo, incluindo o especial de assinatura. Reage de forma falível à altura do golpe e tenta saltar contra agarrões; permanece determinística e serve para playtest.

Captura local: `F9` inicia uma gravação MP4 do framebuffer do jogo com áudio e `F10` para/salva em `captures/`. O submenu `Options` também tem a linha `Local Recording`, útil quando o ambiente captura mal teclas de função. O corte atual envia frames brutos do Raylib para `ffmpeg` e usa PulseAudio para áudio; no WSLg o áudio padrão é `RDPSink.monitor`. Se a fonte de áudio local tiver outro nome, rode com `BORROW_FIGHTERS_CAPTURE_AUDIO_SOURCE=<fonte> cargo run`.

Com `Mostrar debug de combate` ligado, o topo da tela mostra `Pad P1` e `P2` como `ON` quando Raylib detecta o controle. Se um controle Bluetooth estiver pareado mas aparecer `OFF`, confirme se o sistema que executa `cargo run` expõe joystick/gamepad para o Raylib. Em WSL ou ambiente remoto, pode ser necessário testar no host nativo ou encaminhar o dispositivo.

Assets placeholder:

- [`assets/placeholder/arena-sirius.png`](assets/placeholder/arena-sirius.png): `Sirius Light Ring`, Campinas, SP.
- [`assets/placeholder/arena-fortaleza.png`](assets/placeholder/arena-fortaleza.png): `Tech Coast Beacon`, Fortaleza, CE.
- [`assets/placeholder/arena-java-street.png`](assets/placeholder/arena-java-street.png): `Java Street Terminal`, Sao Paulo, SP.
- [`assets/placeholder/arena-biotic.png`](assets/placeholder/arena-biotic.png): `BioTIC Garden`, Brasilia, DF.
- [`assets/placeholder/arena-porto-digital.png`](assets/placeholder/arena-porto-digital.png): `Porto Digital Cache`, Recife, PE.
- [`assets/placeholder/arena-vale-pinhao.png`](assets/placeholder/arena-vale-pinhao.png): `Pinhao Smart Grid`, Curitiba, PR.
- [`assets/lore/story.json`](assets/lore/story.json): capítulos de história e fichas de roster carregados pelo menu.
- [`assets/placeholder/roster-rust.png`](assets/placeholder/roster-rust.png), [`assets/placeholder/roster-duke.png`](assets/placeholder/roster-duke.png), [`assets/placeholder/roster-c.png`](assets/placeholder/roster-c.png) e [`assets/placeholder/roster-python.png`](assets/placeholder/roster-python.png): retratos placeholder do roster.
- [`assets/placeholder/fighter-greybox-spritesheet.png`](assets/placeholder/fighter-greybox-spritesheet.png): poses simples de lutador para testar leitura de movimento e golpes sem debug visual.
- [`assets/placeholder/c-fighter-atlas.png`](assets/placeholder/c-fighter-atlas.png): atlas placeholder jogável do C.
- [`assets/placeholder/c-start-atlas.png`](assets/placeholder/c-start-atlas.png): entrada cinematográfica placeholder do C.
- [`assets/placeholder/c-bitstream-projectile.png`](assets/placeholder/c-bitstream-projectile.png): projectile placeholder do C com `0` e `1` legíveis.
- [`assets/placeholder/python-start-atlas.png`](assets/placeholder/python-start-atlas.png): entrada cinematográfica placeholder da Python com cavalete e gráfico de barras.

Guia completo de teste: [`docs/10-greybox-playtest.md`](docs/10-greybox-playtest.md).
