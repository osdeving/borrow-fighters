# Borrow Fighters

Jogo 2D de luta com humor de programação, iniciado como um projeto **docs-first** e agora com um protótipo greybox jogável em Rust + Raylib.

Status: **Prototype 0.1 / Greybox jogável / Vertical slice em evolução**

## Baixar e jogar (sem instalar Rust)

A versão do playtest é **v0.1.0-prototype.3**, com seleção visual Linker,
novas reações dos lutadores, pausa, revanche e energia para os cinematográficos.
[Abra os downloads e as instruções da release](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.3):
instalador/ZIP para Windows 10 (1903+) ou 11, DEB para Debian/Ubuntu, RPM para Fedora e
arquivo portátil para Linux, todos em x86_64. Extraia a pasta inteira se escolher
a versão portátil. Os arquivos necessários do jogo acompanham os pacotes.
Todos precisam de driver com OpenGL 3.3. No Linux, a base é glibc 2.35+
e desktop com X11 ou XWayland.

Na primeira abertura, **Como jogar** apresenta os controles e permite escolher
CPU, duelo local ou assistir à demo. O guia pode ser reaberto pelo menu.
A entrada normal deixa P1 manual contra CPU; `Quick Fight` abre a seleção com
a configuração atual. Durante a luta, `Esc`/`Start` abre a pausa; o resultado
oferece revanche na mesma arena. O código e as ferramentas de desenvolvimento
continuam descritos abaixo.

Jogue por dez minutos e [conte o que funcionou e o que ficou confuso](https://github.com/osdeving/borrow-fighters/issues/new/choose).
[Notas do playtest](docs/releases/v0.1.0-prototype.3.md) ·
[Como gerar os pacotes](docs/06-release-process.md) ·
[Decisão de distribuição](docs/adr/0019-playtest-distribution.md).

## Objetivo

Este repositório centraliza documentação, governança, assets placeholder e código do primeiro protótipo jogável.

A ideia continua sendo evoluir com decisões explícitas, escopo controlado e colaboração aberta entre programação, game design e arte.

## Índice central

### Visão e produto

- [`docs/27-rust-story-adventure.md`](docs/27-rust-story-adventure.md): proposta de aventura 2D com Rust, prólogo de Ada/Assembly e primeiro capítulo explorável.

- [`docs/26-playtest-visual-completion.md`](docs/26-playtest-visual-completion.md): seleção visual, pausa/revanche, energia e conclusão das reações do elenco.

- [`docs/25-python-cpp-contact-reactions.md`](docs/25-python-cpp-contact-reactions.md): piloto de reações próprias e sincronizadas entre Python e C++.

- [`docs/24-reactions-and-transformations.md`](docs/24-reactions-and-transformations.md): reações dos seis lutadores, Python gigante, notebook/BIOS de C++ e mutação persistente de arena.
- [`docs/23-authored-super-sequences.md`](docs/23-authored-super-sequences.md): quatro supers com roteiro, clones, mutação de arena, BIOS, tiro no pé e vozes distintas.
- [`docs/22-presentation-and-brazilian-stage-life.md`](docs/22-presentation-and-brazilian-stage-life.md): polimento de interface, cenários vivos e seis novos especiais cinematográficos.
- [`docs/00-vision.md`](docs/00-vision.md): visão do jogo.
- [`docs/01-mini-gdd.md`](docs/01-mini-gdd.md): Mini-GDD inicial.
- [`docs/02-prototype-scope.md`](docs/02-prototype-scope.md): escopo do primeiro protótipo.
- [`docs/03-backlog.md`](docs/03-backlog.md): prioridades, backlog de to-do e backlog de bugfix, com histórico do protótipo.
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

- [`docs/adr/0020-match-flow-selection-and-energy.md`](docs/adr/0020-match-flow-selection-and-energy.md): limites entre seleção, fluxo de partida, apresentação e energia.

- [`docs/adr/0018-contact-reaction-profiles.md`](docs/adr/0018-contact-reaction-profiles.md): perfil e janela visual de cada contato, separados do stun físico.

- [`docs/adr/0017-reaction-clocks-and-arena-mutation.md`](docs/adr/0017-reaction-clocks-and-arena-mutation.md): relógios de reação e arena efetiva no World.
- [`docs/adr/0016-authored-super-sequences.md`](docs/adr/0016-authored-super-sequences.md): captura, fases e contatos autorais sob o relógio do combate.
- [`docs/adr/0015-cinematic-presentation-and-stage-life.md`](docs/adr/0015-cinematic-presentation-and-stage-life.md): fontes consistentes, atores de cenário e efeitos cinematográficos com contato local.
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

[![Seleção Linker com Rust e Java](docs/evidence/playtest-visual-flow/roster-rust-java.png)](docs/evidence/playtest-visual-flow/roster-motion.mp4)

_Seleção com retratos, personagens animados, random e vagas futuras.
[Vídeo e revisão da interface, energia e pausa](docs/evidence/playtest-visual-flow/README.md).
[Reações de Rust, Java, Old C e Go](docs/evidence/roster-contact-reactions/README.md)._

[![Python reage ao chute da rajada de C++](docs/evidence/python-cpp-reactions/cpp-barrage-impact.png)](assets/showcase/python-cpp-reactions-2026-09-09.mp4)

_Clique para ver Python e C++ atacando e reagindo: 12 golpes e quatro situações
de defesa para cada personagem, em 86,3 segundos com áudio.
[Padrão de reações](docs/25-python-cpp-contact-reactions.md) e
[evidências por contato](docs/evidence/python-cpp-reactions/README.md)._

A [amostra anterior de reações e transformações](assets/showcase/reactions-transformations-2026-09-09.mp4)
preserva os cinco supers, incluindo Rust/Sirius. O piloto de Python/C++ agora
se estende a Rust, Java, Old C e Go, com 128 poses novas nos oito perfis de contato.
[Escopo e verificação desta rodada](docs/26-playtest-visual-completion.md).

A [amostra dos quatro supers da rodada anterior](assets/showcase/authored-supers-2026-09-09.mp4)
preserva a primeira versão de Duke, Rust, Old C e C++, junto ao
[registro daquela entrega](docs/23-authored-super-sequences.md).

A [amostra das assinaturas e arremessos](assets/showcase/signature-spectacle-2026-09-08.mp4)
permanece como registro da rodada anterior, junto à sua
[produção e verificação](docs/21-signature-spectacle-and-throws.md).

O [clipe original do greybox](assets/showcase/prototype-0.1-greybox.mp4) permanece como histórico.

O acabamento atual inclui fontes Barlow/Lora incorporadas com acentos e filtragem
suave, HUD renovado, menus com descrições, caramelo de pelo curto correndo entre
intervalos e uma participação de “Já acabou, Jéssica?” ao fundo de São Paulo.
`Options > Vida nos cenarios` alterna os detalhes brasileiros dos cenários.
As fontes e a arte têm procedência em [assets/fonts](assets/fonts/README.md) e
[assets/production/stage-life](assets/production/stage-life/README.md).

Cada um dos seis personagens também tem um especial cinematográfico adicional:
`Y` para P1, `]` para P2, ou `LB` segurado + `RT` no controle. Rust, Duke, Old C, Python e C++
agora executam sequências de 5–10,5 segundos: a música pausa, os atores são capturados
e o roteiro assume a luta até a conclusão. Na luta, exigem 100 de energia;
Combat Lab e Move Showcase mantêm o uso livre. A captura funciona inclusive à
distância. Segurar defesa na entrada reduz o
dano para chip e não deixa morrer por chip. Go mantém contato local.
Para inspecionar:

```sh
cargo run -- --showcase --character rust --move cinematic_special --repeat
```

Troque `rust` por `duke`, `go`, `c`, `python` ou `cpp`. Os especiais anteriores
continuam em `T` / `\\` / `RT`. [Roteiros, duração, dano e entrega](docs/24-reactions-and-transformations.md).

Java faz chover lixo, multiplica Duke para coletar/comer e termina com uma queda
gigante. Rust troca a arena por Sirius em quadrados, incluindo a música; Old C
provoca #GP, tela azul e BIOS. C++ digita código com ponteiro nulo no notebook,
atira no pé, aplica a rajada e termina herdando o crash de C. Python se transforma
em cobra gigante, devora o adversário, volta à forma humana, pula e faz sinal de paz.
Todos os defensores têm reações animadas com relógio próprio, incluindo queda e
recuperação ampliadas nos supers. Old C compartilha temporariamente a voz de Rust,
como autorizado para o playtest; as vozes dos demais personagens foram preservadas.
[Créditos de áudio](assets/audio/ATTRIBUTION.md).

O [vídeo do acabamento de 8 de setembro](assets/showcase/presentation-polish-2026-09-08.mp4)
e suas [capturas/verificação](docs/evidence/presentation-polish/README.md)
registram a etapa anterior de tipografia, interface e cenários vivos.

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

O código jogável atual implementa um greybox local para validar o básico: menu principal, seleção visual Linker, treino, lore/roster e opções, arenas brasileiras selecionáveis, pausa e revanche na mesma arena, livro de história carregado de JSON, intro cinematográfica com contagem `11` / `10` / `01` / `Fight!`, personagens com atlas de ações revisadas e placeholders preservados, movimento, pulo diagonal, abaixar, defesa, soco fraco, soco forte, chute, varredura, overhead, anti-air com lançamento, arremesso com troca de lados, ataques aéreos, fireball, cinco especiais de assinatura, seis especiais cinematográficos adicionais, queda com recuperação protegida, primeira identidade mecânica de Rust, Duke/Java, Go, C, Python e C++ por frame data, elenco público com Rust, Duke/Java, C, Python e C++, sem Go no menu, CPU de playtest para um ou dois jogadores, colisão corpo-corpo, hitbox/hurtbox opcional, dano, stun, pushback, whiff recovery, hitspark, block pulse, trail de projétil, luz de chão em hitstun/blockstun, scanline/glow e animações leves de fundo por arena, vida, vitória e restart.

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

Na primeira abertura, o jogo mostra `Como jogar`; depois abre no menu principal. Use `Setas` ou `W/S` para navegar e `Enter` ou `Espaço` para confirmar. O mouse também navega: passe sobre uma opção e clique para confirmar. `Esc` volta das páginas e ferramentas; durante a luta, `Esc` ou `Start` abre a pausa com Continuar, Reiniciar, Trocar personagens e Menu. Para sair do jogo, use `Exit` ou feche a janela. O cursor permanece livre para sair da janela; em WSL, o cursor Linker acompanha o ponteiro apenas enquanto a janela está em foco.

A seleção Linker mostra retratos, os personagens animados em pé e confirmações P1/P2. No modo contra CPU, escolha os dois lados com `WASD`/setas e `Enter`/`A`. No duelo local, P1 usa `WASD` + `F`, P2 usa setas + `Enter`, ou cada jogador usa seu controle; `Espaço` confirma o lado ativo. O mouse permite clicar nos retratos e alternar o lado clicando no painel do jogador. `Tab` troca modo e `Q/E` troca arena; no controle de P1, `Select`/`Back` troca modo e `LB/RB` troca arena. `Random` sorteia uma escolha ao confirmar e as vagas com interrogação ficam reservadas para o futuro. Com ambos confirmados, `Enter`, `Start` ou o botão Lutar inicia a luta. `Esc`/`B` desfaz a confirmação antes de voltar ao menu.

O menu principal mantém a primeira tela simples:

- `Quick Fight`: abre a seleção com a configuração atual.
- `Como jogar`: reabre controles e escolha de modo.
- `Versus Setup`: abre a seleção visual de personagens, arena e modo.
- `Training`: abre `Move Showcase`, `Combat Lab` ou `Sprite Viewer`.
- `Lore / Roster`: abre um livro de programação com capítulos da história e fichas dos personagens.
- `Options`: liga/desliga gravação local e feature flags de protótipo.

Ao iniciar uma luta, o jogo roda a entrada dos personagens e depois bloqueia input durante a contagem central `11`, `10`, `01`, `Fight!`. A revanche preserva personagens e arena. O resultado oferece Revanche, Trocar personagens e Menu; `R` também reinicia uma luta em andamento.

Cada jogador começa com 50 de energia, até o máximo de 100. Acertar ou defender carrega a barra; o cinematográfico consome 100 ao iniciar. Ataques no vazio e o próprio cinematográfico não carregam energia. A assinatura continua independente dessa barra. Combat Lab e Move Showcase mantêm energia livre para revisão; os valores da luta são provisórios, com balanceamento fino em uma rodada posterior.

Por padrão, a luta normal inicia `rust.rs` contra `duke.java` no `Sirius Light Ring` em Campinas, SP. A seleção Linker reúne rust.rs, duke.java, old.c, python.py e cpp.cpp, com retratos derivados da arte atual e escolha de arena pelo nome/local. Go/Gopher continua no repositório, no CLI, no Combat Lab e no Sprite Viewer, mas saiu da seleção pública da demo por enquanto. Para testar matchups direto por CLI, use `--p1`/`--player-one` e `--p2`/`--player-two` com `rust`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx` ou `cpp.cpp`. Adicione `--fight` ou `--skip-menu` para entrar direto na luta sem passar pelo menu. Rust, Duke/Java, Go, C, Python e C++ já possuem vida, loadout, frame data e projectile próprios; Old C compartilha temporariamente a voz de Rust. C joga como fundamentos de alcance/risco, Python como punisher ágil de dano moderado e C++ como herdeira técnica entre alcance de C e ritmo de Python.

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

No Combat Lab, use `Tab` / `Shift+Tab` para alternar golpe, `PageDown` / `PageUp` para alternar pose, `Enter` para repetir, `Espaço` para pausar, `.` para avançar 1 frame quando pausado, `Home` para voltar ao frame 0, `H` para hurtbox, `B` para hitbox, `P` para pivot/eixos, `D` para dummy de contato, `A` para mostrar/esconder o fundo de arena e `Esc` para voltar ao menu quando aberto pelo submenu `Training`. O overlay mostra frame data, vantagem estimada, pushback, whiff recovery e distância após pushback. Valores aceitos em `--character`: `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx` ou `cpp.cpp`. Valores aceitos em `--move`: `light_punch`, `heavy_punch`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick`, `throw`, `projectile`, `signature_special` e `cinematic_special`. Valores aceitos em `--pose`: `move`, `idle`, `crouch`, `jump`, `block`, `hit`, `victory`, `spawn`, `defeat` e `crouch_block`. As poses mantêm o corpo parado para inspeção e reproduzem o clip com pause, avanço por frame e reinício.

O `Move Showcase`, em `Training`, usa o personagem escolhido como Player 1 contra um adversário real. Cada um dos cinco personagens da demo tem 16 situações: os dez golpes anteriores, um especial de assinatura, um cinematográfico e quatro exemplos de defesa. O adversário se aproxima, salta para receber anti-air ou mantém a guarda apropriada para demonstrar rasteira, overhead e agarrão. O resultado mostra o dano ou bloqueio calculado pelo combate. As cenas incluem voo, queda e recuperação, respeitando a duração própria dos supers; o painel inferior deixa o espaço aéreo visível. Agarrões capturam e arremessam para o outro lado, e ganchos lançam a vítima. Os especiais de assinatura são Borrow Fortress, System.out.println!, Segmentation Fault, import antigravity e Undefined Bazooka, sempre disponíveis com `T`/Backslash/`RT`, sem medidor.

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

No Sprite Combat Viewer, use o mouse para inspecionar coordenadas locais do frame, arrastar personagem/dummy e ajustar alças de `frames[].combat`. `N` gera um rascunho de metadata a partir do overlay runtime do golpe selecionado, `Tab` / `Shift+Tab` alterna clip, `Enter` sincroniza clip com golpe, `C` / `Shift+C` alterna personagem de combate, `[` / `]` alterna golpe, `.` / `,` avança ou volta frame, `Espaço` pausa, mouse wheel controla zoom, `0` reseta zoom, `=` / `-` ajusta `scale`, `Setas` ou `Shift+Setas` move o `pivot`, `Ctrl+Setas` ajusta largura/altura do corpo físico, `Ctrl+Shift+Setas` ajusta altura abaixada, `Ctrl+S` salva manifestos de tuning, `O` mostra/esconde dummy, `M` mostra/esconde boxes de combate, `T` mostra/esconde trajetória prevista do projectile, `F5` recarrega manifesto/atlas, `F12` salva screenshot em `captures/sprite-viewer-capture.png` dentro dos dados do usuário, `F9`/`F10` gravam um MP4 local, `G` alterna grade, `P` alterna pivot, `B` alterna bounds, `R` reseta posição e `Esc` volta ao menu quando aberto por `Training`. O padrão de escala fica em [`docs/17-visual-scale-and-stage-metrics.md`](docs/17-visual-scale-and-stage-metrics.md), e o roadmap completo fica em [`docs/16-sprite-combat-viewer-roadmap.md`](docs/16-sprite-combat-viewer-roadmap.md).

O novo Sprite Studio externo vive em `tools/sprite-studio` e deve substituir o viewer Raylib em uma limpeza propria:

```bash
cd tools/sprite-studio
pnpm install
pnpm build
pnpm tauri dev
```

Ele usa Tauri 1.8 + React, edita `*.sprite.json` por UI propria, possui file picker nativo, menu desktop, timeline, paineis colapsaveis, tutorial visual, autosave/backup, snap, guias de escala, presets de combat boxes, validacao do runtime e export de review. Detalhes e pre-requisitos ficam em [`docs/18-sprite-studio.md`](docs/18-sprite-studio.md).

Configurações disponíveis na seleção Linker e em `Options`:

| Preferência | Padrão | Efeito |
|---|---|---|
| Personagem Player 1 | rust.rs | Define o personagem do Player 1 na próxima luta. |
| Personagem Player 2 | duke.java | Define o personagem do Player 2 na próxima luta. |
| Arena | Sirius Light Ring / Campinas, SP | Define o cenário da próxima luta sem esperar a rotação automática. |
| Volume da música | 50% | Ajusta apenas a música de fundo em passos de 10%. |
| Player 1 usa IA | Desligado | P1 começa manual; ligue para assistir à CPU. |
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

O primeiro gamepad conectado controla o Player 1 quando a IA do Player 1 estiver desligada. O segundo gamepad controla o Player 2 quando a IA do Player 2 estiver desligada. P1 começa manual contra P2 CPU. `Como jogar` oferece duelo local e demo; use `Options` para alternar a IA do Player 1 e `C` ou `View` para alternar CPU/manual do Player 2 durante a luta.

Quando ambos os jogadores usam IA, Rust e Java usam perfis diferentes para evitar movimentos espelhados: um tende a jogar mais em média distância e o outro pressiona mais de perto. A IA anda, pula, bloqueia, soca, chuta e tenta o kit completo, incluindo o especial de assinatura. Reage de forma falível à altura do golpe e tenta saltar contra agarrões; permanece determinística e serve para playtest.

Captura local: `F9` inicia uma gravação MP4 do framebuffer do jogo com áudio e `F10` para/salva em `captures/` dentro dos dados do usuário. O submenu `Options` também tem a linha `Local Recording`, útil quando o ambiente captura mal teclas de função. O corte atual envia frames brutos do Raylib para `ffmpeg` e usa PulseAudio para áudio; no WSLg o áudio padrão é `RDPSink.monitor`. Se a fonte de áudio local tiver outro nome, rode com `BORROW_FIGHTERS_CAPTURE_AUDIO_SOURCE=<fonte> cargo run`.

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
