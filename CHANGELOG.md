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

### Em aberto

- Definir organização GitHub e times reais.
- Definir primeira milestone oficial.
