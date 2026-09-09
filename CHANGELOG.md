# Changelog

Todas as mudanças relevantes do projeto devem ser registradas aqui.

O formato é inspirado em Keep a Changelog, mas adaptado para o estágio de pré-produção.

## [Unreleased]

### Adicionado

- Seis especiais cinematográficos adicionais: Ownership Eclipse, JVM Overdrive,
  Million Goroutines, Kernel Panic, Event Horizon e Template Singularity, com
  efeitos de tela inteira e um único contato local bloqueável. Entrada `Y` / `]`
  / `LB+RT`, CLI `--move cinematic_special`, Combat Lab e showcase.
- Caramelo brasileiro de pelo curto em corrida animada, cameo gestual de “Já
  acabou, Jéssica?” em São Paulo e detalhes de memes discretos nas seis arenas;
  opção `Vida nos cenarios` e Sirius revisado sem o cachorro estático anterior.
- Fontes Barlow/Lora incorporadas com licença OFL, glifos portugueses, atlas de
  alta resolução e filtragem por mipmaps; logo, retratos, contagem e fundos
  pintados das arenas suavizados na escala de apresentação.
- HUD renovado com barras espelhadas, banner de vitória, dicas de controles e
  menus com descrições e geometria ajustada para navegação por teclado e mouse.

- Showcase contextual com dois lutadores no combate real, 11 ataques e quatro exemplos de defesa por personagem da demo, resultado de contato, áudio real, repetição, espelhamento, pausa e avanço de frame.
- Entrada direta `--showcase --character ... --move ...`, com `--repeat` e `--reverse`; `L` repete, `X` espelha e `PageUp/PageDown` troca personagem.
- Cinco especiais de assinatura com oito poses e efeitos animados separados: Borrow Fortress, System.out.println!, Segmentation Fault, import antigravity e Undefined Bazooka. Sempre acessíveis com `T`/Backslash/`RT`, sem medidor; o projétil comum mantém seus controles.
- Arremesso real que pune guarda, captura, levanta e lança por cima para trocar os lados; pouso seguro nos cantos e KO que aguarda aterrissagem. Novas poses de atacante e vítima nos cinco.
- Reações específicas `heavy_hit`, `launched`, `thrown` e recuperação protegida `knockdown`; ganchos lançam de verdade e a vítima permanece sem controle até pousar.
- Showcase de 260 frames por situação, soma dos três pulsos Java e painel inferior que deixa o espaço aéreo livre.
- Auditoria determinística de 60 lutas espelhadas, evidência em CSV e testes de contrajogo, guarda, postura baixa e contatos reais do showcase.

- Navegação dos menus com hover, clique esquerdo para ativar e clique direito para voltar valores ajustáveis.
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
- Vozes CC0 de ataque, dano e defesa para Go, C e Python, com fallback de ataque por personagem para cobrir os golpes da demo.
- Otimização do player de áudio para reduzir clones/alocações por evento e evitar chamadas repetidas de ducking de música sem mudança de estado.
- Suporte a música de fundo via `Music` streaming do Raylib, com troca automática entre menu e luta.
- Arenas placeholder Sirius e Fortaleza Tech Coast, com rotação de cenário ao iniciar a próxima luta após uma vitória começando pelo Sirius.
- Combat Lab com fundo de arena ligado por padrão e atalho `A` para alternar entre cenário e grid limpo.
- Contagem pré-luta central `11`, `10`, `01`, `Fight!`, bloqueando gameplay até a liberação.
- Vozes CC0 de contagem pré-luta registradas no manifesto de áudio.
- Direção narrativa inicial com O Linker como força cósmica, Ada Lovelace, Rust, Duke, Assembly, frontenzos e arenas brasileiras de ciência/tecnologia.
- Primeiro corte de golpes tradicionais: varredura baixa, overhead, anti-air, ataques aéreos e agarrão curto, com testes dedicados em `tests/traditional_moves.rs` e cobertura da CPU em `tests/cpu_traditional_moves.rs`.
- Primeiro corte de identidade mecânica por dados: Rust com anti-air/throw mais rápidos e menores; Duke com sweep/overhead/throw mais longos, pesados e puníveis; Go como rushdown, agora com atlas placeholder no Combat Lab e em match via CLI, registrado em `docs/15-character-combat-matrix.md`.
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
- Sprite Combat Viewer com inspetor de coordenada local/atlas do cursor e sincronização manual entre golpe e clip visual.
- Backlog central reforçado com tabela **Agora / Proximo / Depois** para manter proximas frentes, issues e PRs rastreaveis.
- Go/Gopher com atlas placeholder próprio de luta, entrada cinematográfica e projectile de canais.
- Demo pública ajustada para ciclar Rust, Duke/Java, C e Python no menu, mantendo Go/Gopher disponível por CLI, Combat Lab e Sprite Viewer.
- Kits terrestres próprios para C e Python, com `MoveId`, frame data, dano, whiff recovery, reações e testes de balanceamento inicial.
- Feedback visual de hit/block reforçado no renderer com tint nos sprites e flash no corpo durante hitstun/blockstun.
- Atlases de entrada de Rust e Duke regenerados pelo pipeline para remover componentes isolados que pareciam sobras de frames anteriores.
- VFX de demo para hitspark, block pulse, rastro de projectile, luz de chão em hitstun/blockstun e scanline/glow sutil de arena.
- Novas arenas placeholder `BioTIC`, `Porto Digital` e `Vale do Pinhao`, com rotação ampliada para seis cenários.
- Seis novas músicas CC0/public domain no manifesto, com troca por tela e por arena.
- Seleção manual de arena no `Versus Setup`, com nomes, locais e conceito de cada cenário.
- Controle de volume da música em `Options`, aplicado sem reduzir vozes e SFX.
- VFX animados de fundo por arena, incluindo feixes, pacotes de dados, chuva, shimmer, pulsos e scanlines sutis.
- Teste de manifesto garantindo voz de início de golpe e cast de projectile para todos os personagens jogáveis e de ferramenta.
- Submenu `Lore / Roster`, com livro do Linker, capítulos, fichas de personagem e retratos placeholder de Rust, Duke/Java, C e Python.
- JSON runtime `assets/lore/story.json` para editar história e roster sem recompilar o jogo.
- Clips CC0 adicionais e bindings específicos para deixar golpes de Rust e Duke/Java audíveis por move, sem depender só do fallback curto.

### Corrigido

- Prioridade indevida de Player 1 em especiais simultâneos e direção frontal da proteção de Rust em curta distância.
- Vítima flutuando na aterrissagem, reset em pé no KO aéreo e cortes laterais das poses de reação.
- Levantamento com mãos afastadas da vítima: poses de apoio regeneradas e conferidas nas duas direções.

- Sparks e dano flutuante posicionados no contato real entre hitbox e hurtbox antes do pushback, incluindo rasteira, anti-air e projétil.

- Retângulos de reação/guarda e limites da arena exibidos com debug desligado; a luta limpa mantém sprites, tintas, luzes e impactos sem essas caixas.
- Rasteiras que voltavam à postura alta durante o ataque; hitboxes revisadas e hurtboxes baixas agora coincidem entre combate e debug.
- Guarda abaixada perdida durante blockstun, blockstun residual após quebra por overhead, chip encerrando luta e agarrões atingindo saltos ou recuperação protegida.
- Conjuração de projétil sobreposta a ataques, pulo ou guarda; um impacto também cancela sua pose.
- Ataques acompanhando automaticamente o adversário que saltava por cima e comandos perdidos entre renderização e tick de combate; presses da luta são consumidos uma vez, mesmo quando um frame de renderização exige vários ticks.

- Cursor reposicionado no centro a cada quadro, que deixava o `Linker` parado e impedia alcançar o botão de fechar da janela.
- Overlay `Linker` residual ao tirar o foco ou mover o mouse para fora da janela.

### Em aberto

- Definir organização GitHub e times reais.
- Definir primeira milestone oficial.
