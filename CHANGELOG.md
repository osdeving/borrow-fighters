# Changelog

Todas as mudanças relevantes do projeto devem ser registradas aqui.

O formato é inspirado em Keep a Changelog, mas adaptado para o estágio de pré-produção.

## [Unreleased]

### Adicionado

- Base inicial de documentação docs-first.
- Governança proposta para PRs, branches, labels, squads e releases.
- Templates de contribuição para GitHub, arte, personagens, ADR e release.
- ADR de fluxo de versionamento com Trunk-Based Development adaptado e Conventional Commits.
- Workflows iniciais para validar documentação, YAML do GitHub e título de PR.
- Esboço inicial de arquitetura Rust + Raylib.
- Instruções repo-local para Codex e Claude Code.
- Skills iniciais de IA para atlas do repo, Rust/Raylib, gameplay e direção de arte.
- Primeiro protótipo greybox jogável com Rust + Raylib.
- Guia de playtest do greybox.
- Workflow Rust inicial com fmt, testes e clippy.
- HUD greybox reorganizado para evitar sobreposição de texto.
- Fireball simples e movimento/pulo mais suave no protótipo.
- Kit greybox tradicional com soco fraco, soco forte, chute, defesa, abaixar e corpo composto por partes.
- CPU simples para o Player 2, ligada por padrão e alternável com `C`.
- Tuning inicial de ritmo: golpes e fireball mais lentos, e CPU menos agressiva.
- Arena bitmap placeholder `Terminal Compiler Lab` e carregamento inicial de texture asset.
- Suporte inicial a gamepad estilo Xbox para Player 1 e Player 2 manual.
- Tela inicial de preferências com feature flags para IA, dano do Player 1, HUD, ajuda, debug de combate e gamepad.
- Spritesheet placeholder de lutador com poses de idle, andar, abaixar, pular, defender, socos e chute.
- Runtime inicial de sprites com atlas + manifesto JSON.
- Atlas placeholder de Rust e Duke, incluindo clips de luta, especial e vitória.
- Animações cinematográficas de entrada para Rust e Duke.
- Projéteis separados para Rust e Duke.
- IA opcional para ambos os jogadores com perfis diferentes e ações variadas.
- Flag para Player 2 ignorar dano durante playtest.
- Arena Java Street como cenário atual do protótipo.
- Ferramentas locais em `tools/art/` para extrair atlas e gerar manifests.
- Documentação atualizada para refletir o estado jogável da `main`.
- Amostra em vídeo sem áudio no README, com capa clicável e disclaimer de arte placeholder.
- Roadmap técnico de combate com pesquisa, arquétipos, Combat Lab e plano de modularização.
- Frame data inteira para golpes próximos atuais, com testes de startup/active/recovery e overlay de debug.
- Frame data de projectile/special com spawn, duração visual e cooldown em frames.
- Combat Lab mínimo por CLI para inspecionar golpes isolados, frame step, pivot, hurtbox, hitbox e projectile.
- Tabela inicial `MoveSpec` para golpes próximos, mantendo `AttackKind` como camada runtime de compatibilidade.
- Registro inicial `CharacterSpec` para Rust e Duke.
- Guia técnico de combate com rastreio de código, técnica de hitbox/hurtbox, comandos e hotkeys do Combat Lab.
- Runtime de luta consumindo `CharacterSpec` para nome, vida máxima e loadout de golpes; Duke/Java começa com vida máxima maior.
- Combat Lab com `--pose` e hotkeys `PageDown`/`PageUp` para inspecionar idle, crouch, jump, block, hit e victory.
- Overlay/debug do Combat Lab separado em `src/ui/combat_debug.rs` para reduzir responsabilidade do renderer.
- Primeiros golpes próximos específicos por personagem: `RustBorrowJab` e `DukeBoilerplatePoke`, resolvidos por loadout sem mudar os controles.
- Início da Fase 4 de combate com `GuardRule`, `HitReaction`, hitstun/blockstun inicial e debug visual de stun.
- Pushback simples para hit, block e projétil, configurado por `HitReaction` e aplicado pelo runtime de luta.
- Combat Lab com leitura de vantagem estimada, pushback, distância após pushback e dummy de contato por golpe.
- Fechamento da Fase 4 com whiff recovery explícito por golpe, debug visual de `WHIFF` e respostas mínimas de contra-jogo documentadas.
- Motor inicial de áudio por eventos com manifesto JSON, bindings por cue/personagem/golpe e integração Raylib para clips opcionais.
- Documentação técnica do pipeline de áudio e ADR para eventos de áudio data-driven.
- Assets CC0 iniciais de áudio para impactos, defesa, whiff, UI, anúncio de luta/vitória e música de menu/combate.
- Suporte a música de fundo via `Music` streaming do Raylib, com troca automática entre menu e luta.
- Arenas placeholder Sirius e Fortaleza Tech Coast, com rotação de cenário ao iniciar a próxima luta após uma vitória começando pelo Sirius.
- Combat Lab com fundo de arena ligado por padrão e atalho `A` para alternar entre cenário e grid limpo.
- Contagem pré-luta central `11`, `10`, `01`, `Fight!`, bloqueando gameplay até a liberação.
- Vozes CC0 de contagem pré-luta registradas no manifesto de áudio.
- Direção narrativa inicial com O Linker como força cósmica, Ada Lovelace, Rust, Duke, Assembly, frontenzos e arenas brasileiras de ciência/tecnologia.
- Primeiro corte de golpes tradicionais: varredura baixa, overhead, anti-air, ataques aéreos e agarrão curto, com testes dedicados em `tests/traditional_moves.rs` e cobertura da CPU em `tests/cpu_traditional_moves.rs`.
- Primeiro corte de identidade mecânica por dados: Rust com anti-air/throw mais rápidos e menores; Duke com sweep/overhead/throw mais longos, pesados e puníveis; Go como rushdown greybox no Combat Lab e em match via CLI, registrado em `docs/15-character-combat-matrix.md`.
- Seleção de matchup por CLI para luta normal com `--p1`/`--player-one` e `--p2`/`--player-two`, além de `--fight`/`--skip-menu` para iniciar direto em match real.
- Seleção mínima de personagens na tela de preferências, ciclando Player 1 e Player 2 entre Rust, Duke e Go.
- `ProjectileSpec` por personagem: Rust mantém projectile médio, Duke ganha projectile mais pesado/lento e Go ganha burst rápido de curto alcance.
- `CombatLog` diagnóstico no `World`, registrando round, countdown, ataques, whiffs, hits, projectiles e fim de luta para reproduzir bugs.
- Issue de follow-up de combate para playtestar a branch de identidade mecânica e decisões restantes.
- Roadmap do Sprite Combat Viewer para artistas conferirem atlas, pivot, grid, hitbox/hurtbox futura e origem de projectile.
- Primeiro corte do Sprite Combat Viewer por CLI, carregando manifesto/atlas em runtime, com grid, pivot, bounds, navegação de clips/frames e drag com mouse.
- Sprite Combat Viewer com dummy espelhado arrastável, distância entre anchors, zoom por mouse wheel, hot reload de manifesto/atlas com `F5` e screenshot com `F12`.
- Sprite Combat Viewer com overlay inicial de combate via `--character` e `--move`, mostrando hurtboxes atuais, hitbox do golpe e origem/caixa de projectile.
- Schema opcional `frames[].combat` em manifests de sprite para hurtboxes, hitboxes e origem de projectile por frame, com validação e testes.
- Sprite Combat Viewer com overlay data-driven de `frames[].combat` e timeline inferior de startup/active/recovery para o golpe selecionado.
- Sprite Combat Viewer com troca runtime de personagem/golpe e preview simples de trajetória de projectile.

### Em aberto

- Definir organização GitHub e times reais.
- Definir primeira milestone oficial.
