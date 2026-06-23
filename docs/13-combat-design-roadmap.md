# 13 — Roadmap de Combate e Gameplay

## Status

Em implementação.

Fases 1 a 4 concluídas em corte mínimo. A Fase 5 tem o primeiro corte de identidade por dados: Rust ganhou anti-air/throw mais rápidos e menores; Duke ganhou sweep/overhead/throw mais longos, pesados e puníveis; Go entrou como rushdown mantido para CLI/ferramentas enquanto fica fora do menu público da demo; C ganhou fundamentos de alcance/risco; Python ganhou kit ágil de dano moderado. Golpes atuais e projectile já possuem frame data inteira, o Combat Lab abre por CLI ou `Training -> Combat Lab` com playback de golpes e poses estáticas, golpes próximos usam `MoveSpec`, especiais usam `ProjectileSpec` por personagem, personagens possuem `CharacterSpec` consumido pelo runtime para nome, vida máxima, loadout e projectile, e o overlay de debug do laboratório foi separado em `src/ui/combat_debug.rs`.

Este documento define como evoluir o combate de **Borrow Fighters** de greybox funcional para um sistema mensurável, modular e testável de jogo de luta 2D.

## Problema

O protótipo atual já prova movimento, dano, hitbox/hurtbox, defesa, projectile, IA de playtest e sprites placeholder. O próximo risco é crescer o combate sem instrumentos de precisão.

Antes de criar muitos personagens ou golpes, precisamos conseguir responder com dados:

- em qual frame o golpe começa;
- em quais frames ele acerta;
- quando o personagem volta a agir;
- onde está o pivot do sprite;
- onde estão hitbox e hurtbox por fase;
- se o golpe é punível, seguro, interrompível ou sem resposta;
- se o personagem tem identidade real ou só troca visual.

## Pesquisa resumida

Referências usadas em 2026-06-21:

- [CAPCOM SF Seminar — The Basics of Boxes](https://game.capcom.com/cfn/sfv/column/131422?lang=en): hitbox/hurtbox explicam por que um golpe conecta ou erra.
- [CAPCOM SF Seminar — The Basics of Attack Composition](https://game.capcom.com/cfn/sfv/column/131432?lang=en): ataques têm startup, active frames e recovery.
- [The Fighting Game Glossary by Infil](https://glossary.infil.net/): vocabulário de comunidade para zoner, rushdown, grappler, safe, punish etc.
- [The Fighting Game Glossary — Frame Data](https://glossary.infil.net/?t=Frame%20Data): frame data é a linguagem prática para entender vantagem, punição e timing.
- [The Fighting Game Glossary — Frame Advantage](https://glossary.infil.net/?t=Frame%20Advantage): vantagem ou desvantagem em frames ajuda a definir se um golpe é punível.
- [Sirlin — Balancing Multiplayer Games, Part 1](https://www.sirlin.net/articles/balancing-multiplayer-games-part-1-definitions): balanceamento é ter muitas opções viáveis, especialmente em alto nível.
- [Sirlin — Balancing Multiplayer Games, Part 2](https://www.sirlin.net/articles/balancing-multiplayer-games-part-2-viable-options): movimentos precisam de respostas; dominância sem contra-jogo reduz profundidade.
- [Sirlin — Balancing Multiplayer Games, Part 3](https://www.sirlin.net/articles/balancing-multiplayer-games-part-3-fairness): em jogos assimétricos, cada personagem deve ter chance razoável de vencer em mãos certas.
- [Sirlin — What Should Be Banned?](https://www.sirlin.net/ptw-book/what-should-be-banned): banir deve ser raro; se algo parece "apelão", primeiro é preciso perguntar se existe resposta real.
- [Fantasy Strike — About](https://www.fantasystrike.com/about): jogos assimétricos usam personagens com habilidades e personalidades diferentes, não só skins sobre os mesmos golpes.

## Conclusões de design

### Personagens não devem ser skins

Não queremos todos os personagens com os mesmos golpes e só VFX diferentes.

Cada personagem deve ter:

- plano de jogo;
- alcance dominante;
- fraqueza clara;
- ferramenta de aproximação ou contenção;
- pelo menos um golpe assinatura;
- custo real para sua ferramenta mais forte.

### Balanceamento não é igualdade exata

O alvo não é todo personagem ter o mesmo dano, velocidade e alcance. O alvo é viabilidade: cada personagem precisa ter plano de vitória plausível contra o elenco em mãos de um jogador forte.

Para o protótipo, vamos medir:

- se cada personagem consegue iniciar dano;
- se cada personagem consegue defender ou escapar de pressão básica;
- se cada personagem consegue lidar com projectile;
- se existe alguma situação repetível sem contra-jogo;
- se algum golpe resolve neutral, pressão e defesa ao mesmo tempo.

### Todo golpe forte precisa de resposta

Nem todo golpe precisa ser defendido da mesma forma.

Exemplos de respostas aceitáveis:

- bloquear e punir no recovery;
- andar para fora do alcance;
- pular;
- interromper no startup;
- agachar ou defender baixo;
- usar invulnerabilidade com custo;
- bater no projectile;
- esperar cooldown;
- gastar recurso para atravessar;
- aceitar chip pequeno, mas recuperar espaço.

O que evitar no Prototype 0.1:

- unblockable rápido sem aviso;
- projectile sem cooldown e sem aproximação possível;
- golpe seguro no block, forte no hit, rápido no startup e com alcance grande;
- combo infinito;
- touch of death sem recurso e sem erro prévio relevante;
- loops de knockdown sem decisão defensiva;
- ataque que ignora hurtbox sem contrapartida.

### Reclamações comuns que devemos antecipar

1. "Isso me acertou de onde?"
   Mitigação: debug de hitbox/hurtbox, hurtbox por pose e sprites alinhados ao pivot.

2. "Não dá para punir."
   Mitigação: frame data explícita, recovery visível no laboratório e testes de vantagem.

3. "Zoner não deixa jogar."
   Mitigação: cooldown, recuperação em projectile, opções de aproximação e zonas mortas.

4. "Grappler é injusto porque grab passa block."
   Mitigação: throw precisa de curta distância, startup, whiff recovery e resposta por pulo/backdash/jab.

5. "Todo mundo parece igual."
   Mitigação: arquétipos, alcance, tempo, hurtbox e riscos diferentes por personagem.

6. "O jogo exige decorar coisa invisível."
   Mitigação: laboratório de combate, overlay de frame data e documentação de golpes.

7. "A IA parece aleatória ou injusta."
   Mitigação: IA de playtest usa probabilidades e perfis, mas não lê input perfeito.

## Arquétipos iniciais

### Rust — all-rounder técnico

Função: protagonista, "Ryu" do jogo, fácil de entender e difícil de dominar.

Plano de jogo:

- médio alcance;
- golpes limpos;
- projectile moderado;
- defesa confiável;
- poucas ferramentas, mas todas honestas.

Fraquezas:

- dano não deve ser o maior;
- não deve ter pressão infinita;
- precisa tomar decisões corretas, não ganhar por stats.

Golpes candidatos:

- `borrow_jab`: jab rápido para checar avanço. **Implementado em corte inicial** como soco fraco mais rápido, curto e de menor dano.
- `lifetime_step`: avanço curto com recovery punível se mal usado.
- `gear_projectile`: projectile médio, bom para controlar espaço, mas com cooldown.
- `ownership_counter`: contra-ataque futuro, só depois de defesa básica estar sólida.

### Duke / Java — pressão corporativa e midrange pesado

Função: veterano resistente, bom em controle de ritmo e pressão por presença.

Plano de jogo:

- midrange;
- normals mais longos;
- pressure mais lenta, mas difícil de ignorar;
- projectile/bean como ferramenta de ocupação de espaço;
- mais vida ou mais estabilidade que Rust.

Fraquezas:

- startup e recovery maiores;
- movimento menos explosivo;
- se errar golpe pesado, deve ser punido.

Golpes candidatos:

- `boilerplate_poke`: normal longo, dano médio, recovery claro. **Implementado em corte inicial** como soco forte mais longo, mais danoso e mais lento.
- `gc_sweep`: chute/varredura lenta que ganha espaço.
- `bean_projectile`: projectile lento que força movimento, não spam.
- `enterprise_lock`: pressão futura com custo ou setup.

### Go — rushdown de concorrência

Função: personagem de velocidade e pressão, mantido como personagem testável por CLI/ferramentas enquanto a arte do Gopher fica fora da demo pública.

Plano de jogo:

- aproximação rápida;
- strings curtas;
- troca alcance por mobilidade;
- golpes com recovery curto, mas dano menor.

Fraquezas:

- vida ou stun menor;
- alcance curto;
- sofre contra bons anti-airs e controle de espaço.

Golpes candidatos:

- `goroutine_jab`: checagem muito rápida e curta. **Implementado em corte inicial** como soco fraco de baixo dano.
- `defer_kick`: chute mais rápido que o genérico, com alcance reduzido. **Implementado em corte inicial**.
- `channel_overhead`: overhead menos pesado que o de Duke, feito para manter ritmo. **Implementado em corte inicial**.
- `hopkick`: ataque aéreo de pressão curta. **Implementado em corte inicial**.
- `goroutine_dash`: avanço rápido com risco se bloqueado, futuro.
- `channel_cross`: troca de lado futura, só depois do laboratório existir.

### C — fundamentos de baixo nível

Função: personagem direto, sólido e um pouco perigoso no whiff.

Plano de jogo:

- alcance honesto;
- dano levemente acima do genérico;
- lows/overheads simples;
- projectile de bitstream para controle médio;
- riscos claros quando erra poke, sweep ou throw.

Fraquezas:

- whiff recovery maior;
- pressão simples;
- precisa de espaçamento para não virar alvo de rushdown.

Golpes implementados:

- `c_pointer_jab`: jab de alcance levemente maior.
- `c_unsafe_poke`: soco forte longo e punível.
- `c_null_step_kick`: chute de controle médio.
- `c_segfault_sweep`: low forte e longo.
- `c_stack_overflow`: overhead moderado.
- `c_interrupt_vector`: anti-air honesto.
- `c_undefined_throw`: throw com pushback forte e whiff maior.

### Python — punisher ágil

Função: personagem rápida que pune erro e troca dano bruto por tempo.

Plano de jogo:

- startup e recovery mais leves;
- dano moderado;
- leitura de whiff;
- projectile rápido/médio;
- cobra como leitura visual do jab.

Fraquezas:

- vida menor que C/Duke;
- alcance menor em ferramentas de abertura;
- não deve ganhar troca direta de dano.

Golpes implementados:

- `python_snake_bite`: jab rápido da cobra.
- `python_data_strike`: soco forte rápido de dano menor.
- `python_heel_kick`: chute ágil.
- `python_indent_sweep`: low rápido e menos danoso.
- `python_traceback_overhead`: overhead rápido/moderado.
- `python_vision_anti_air`: anti-air rápido e menor.
- `python_constrict_throw`: throw moderado com whiff menor.

### Assembly — boss não-jogável

Função: boss técnico, ritualístico e injusto o bastante para parecer mítico, mas não aleatório.

Plano de jogo:

- golpes simples e letais;
- leitura espacial;
- timings estranhos;
- corpo fora de fase como mecânica visual.

Limite:

- não entra no balanceamento competitivo inicial;
- precisa ser testado como encontro PvE, não como personagem de torneio.

## Combat Lab

Criar uma cena de teste focada em precisão, sem fundo artístico, sem menu completo e sem ruído visual.

Nome proposto: **Combat Lab**.

Objetivo:

- abrir uma janela direta;
- mostrar um personagem por vez;
- exibir pivot, eixo, floor line, hurtboxes, hitboxes e guard boxes;
- tocar um golpe isolado;
- pausar, avançar frame a frame e repetir;
- alternar move atual;
- alternar estado: idle, crouch, jump, block, hit, victory;
- opcionalmente ligar um dummy fixo depois.

Entrada proposta:

```bash
cargo run -- --fight --p1 go --p2 duke
cargo run -- --lab combat --character rust
cargo run -- --lab combat --character duke --move heavy_punch
cargo run -- --lab combat --character go --move kick
cargo run -- --lab combat --character c --move heavy_punch
cargo run -- --lab combat --character python --move light_punch
```

Atalhos propostos:

| Ação | Tecla |
|---|---|
| Próximo golpe | `Tab` |
| Golpe anterior | `Shift+Tab` |
| Repetir golpe | `Enter` |
| Pausar/continuar | `Espaço` |
| Avançar 1 frame | `.` |
| Voltar ao frame 0 | `Home` |
| Alternar hurtbox | `H` |
| Alternar hitbox | `B` |
| Alternar pivot/eixos | `P` |
| Alternar dummy | `D` |
| Exportar screenshot | `F12` |

Overlay mínimo:

- personagem;
- golpe atual;
- frame atual;
- fase: startup/active/recovery;
- dano;
- advantage estimado;
- distância estimada após pushback;
- whiff recovery;
- hitbox ativa ou inativa;
- pivot local e pivot em tela;
- posição da mão/pé usada para projectile ou golpe.

## Arquitetura proposta

### Princípio

Os módulos devem se comunicar por dados públicos pequenos. Um módulo não deve depender dos detalhes internos do outro.

### Novos módulos candidatos

```text
src/combat/
├── frame.rs          # FrameCount, conversão entre frame e fixed timestep
├── boxes.rs          # BoxSpec, BoxTimeline, Pivot, AnchorPoint
├── move_data.rs      # MoveId, MoveSpec, MovePhase, GuardRule, HitReaction
├── resolver.rs       # Aplica hit/block/throw/projectile e produz CombatEvent
└── tuning.rs         # Helpers e validações de balanceamento

src/characters/
├── mod.rs            # CharacterSpec mínimo e registro público de personagens
├── rust.rs           # Dados iniciais do Rust
├── duke.rs           # Dados iniciais do Duke
└── go.rs             # Dados do Go quando o registro for dividido

src/scenes/
└── combat_lab.rs     # Cena direta de laboratório de golpes

src/ui/
└── combat_debug.rs   # Overlay de frame data, box labels e medição
```

### Tipos principais

```text
CharacterSpec
  id
  display_name
  archetype
  stats
  hurtbox_profiles
  moves

MoveSpec
  id
  input_kind
  startup_frames
  active_frames
  recovery_frames
  damage
  guard_rule
  hit_reaction
  hitboxes
  hurtbox_profile_override
  projectile_spawn
  cancel_rules

BoxTimeline
  frame_range
  boxes
  anchor
```

### Fronteiras

- `characters/*` define dados, não resolve colisão.
- `combat/*` resolve regras puras, não desenha.
- `game/world.rs` orquestra lutadores, projéteis e resultado, mas não conhece detalhe de sprite.
- `engine/render.rs` e `engine/render/combat_lab.rs` desenham snapshots e sprites próximos da borda Raylib.
- `ui/combat_debug.rs` desenha overlay de frame data, box labels, pivot e medição do Combat Lab.
- `scenes/combat_lab.rs` usa o mesmo runtime de combate, mas com estado isolado e sem match flow.

## Fases de execução

### Fase 1 — Medição antes de balancear

Status: **concluída**.

Entregáveis:

- [x] converter timings de golpes para frames inteiros;
- [x] criar `FrameCount`;
- [x] manter conversão clara com fixed timestep;
- [x] testes para startup/active/recovery dos golpes atuais;
- [x] overlay mostrando frame atual e fase;
- [x] incluir projectile na mesma tabela de frame data.

Critério de aceite:

- LP, HP, chute e projectile têm frame data visível e testada.
- Projectile registra spawn no frame 0, animação visual de 21 frames e cooldown de 57 frames como decisão provisória do Prototype 0.1.

### Fase 2 — Combat Lab

Status: **concluída em corte mínimo**.

Entregáveis:

- [x] CLI flag `--lab combat`;
- [x] cena limpa sem fundo artístico;
- [x] um personagem isolado;
- [x] seleção de golpe;
- [x] pause, replay e frame step;
- [x] pivot, hurtbox e hitbox desenhados.

Critério de aceite:

- conseguimos alinhar mão/pé/projectile sem iniciar uma luta real.
- Comando atual: `cargo run -- --lab combat --character rust --move light_punch`.
- Poses atuais: `cargo run -- --lab combat --character rust --pose block`.

### Fase 3 — MoveSpec e CharacterSpec

Status: **concluída em corte mínimo, com primeiro tuning específico por personagem**.

Entregáveis:

- [x] mover dados hard-coded de `AttackKind::spec` para `MoveSpec`;
- [x] criar `CharacterSpec` para Rust, Duke, Go, C e Python;
- [x] fazer `World`, `Combat Lab` e `Fighter` consumirem nome, vida máxima e loadout vindos de `CharacterSpec`;
- [x] manter comportamento atual com dados novos;
- [x] testes garantindo que dados antigos continuam equivalentes.
- [x] permitir que o mesmo botão resolva para `MoveSpec` diferente conforme o loadout do personagem.

Critério de aceite:

- adicionar um golpe novo não exige alterar `Fighter` profundamente.
- `AttackKind` permanece como camada de compatibilidade runtime para sprites, debug e seleção de ataque.
- Rust possui `RustBorrowJab`, `RustLifetimeAntiAir` e `RustOwnershipThrow` como ferramentas rápidas/curtas; Duke possui `DukeBoilerplatePoke`, `DukeGarbageCollectorSweep`, `DukeAbstractFactoryOverhead` e `DukeEnterpriseThrow` como ferramentas longas/pesadas e mais puníveis; C possui kit terrestre próprio de fundamentos; Python possui kit terrestre próprio de punição ágil.
- A parte mínima está aceita; a próxima evolução é refinar defesa, hitstun/blockstun e contra-jogo.

### Fase 4 — Defesa e contra-jogo

Status: **concluída em corte mínimo**.

Entregáveis:

- [x] separar high/low/mid/throw/projectile em `GuardRule`;
- [x] bloquear strike/projétil conforme `GuardRule`, com `Throw` explicitamente não bloqueável;
- [x] whiff recovery explícito além da duração/recovery atual dos golpes;
- [x] blockstun/hitstun inicial;
- [x] pushback simples para hit/block e projétil.
- [x] leitura de vantagem estimada no Combat Lab.

Critério de aceite:

- cada golpe forte tem pelo menos uma resposta documentada.
- o corte atual impede ação durante hitstun/blockstun, aplica pushback, mostra vantagem estimada no Combat Lab e aplica whiff recovery quando golpe próximo erra.
- `Low`, `High`/overhead, `Throw`, anti-air e ataques aéreos já têm primeiro corte jogável. Rust, Duke/Java, Go, C e Python possuem identidade própria nos golpes terrestres principais; ataques aéreos universais continuam aceitos no Prototype 0.1.

Respostas mínimas documentadas:

| Ferramenta | Risco atual | Respostas de contra-jogo |
|---|---|---|
| `HeavyPunch` genérico | startup 11f, recovery após contato, whiff recovery 10f | andar fora do alcance, bloquear e observar vantagem, pular antes do contato, punir whiff |
| `Kick` | alcance baixo/médio, active limitado, whiff recovery 8f | defender mid no corte atual, recuar, pular, punir se errar |
| `SweepKick` | acerta baixo, recovery claro, perde para salto/spacing | defender abaixado, pular, ficar fora do alcance, punir whiff |
| `OverheadPunch` | vence defesa abaixada, startup/recovery maiores | defender em pé, interromper startup, andar fora, punir se errar |
| `RisingAntiAir` | cobre alto/frente, pode errar no chão se mal espaçado | baitar e punir recovery, bloquear, atacar por baixo no timing certo |
| `AirPunch` / `AirKick` | só inicia no ar, trajetória compromete posição | anti-air, andar para fora, defender em pé |
| `CloseThrow` | ignora defesa, alcance muito curto, whiff recovery alto | sair do alcance, pular, jab antes do active |
| `DukeBoilerplatePoke` | alcance e dano altos, startup 13f, whiff recovery 12f | ficar fora do alcance, interromper startup com jab rápido, bloquear e recuperar espaço com pushback, punir whiff |
| Projectile | cooldown 57f e trajetória horizontal | bloquear chip reduzido, pular, aproximar durante cooldown, usar spacing para forçar erro |

### Fase 5 — Identidade dos personagens

Status: **concluída em corte mínimo de dados; aguardando playtest e arte própria**.

Entregáveis:

- [x] Rust all-rounder técnico em dados: `RustBorrowJab`, `RustLifetimeAntiAir`, `RustOwnershipThrow`;
- [x] Duke midrange/pressure em dados: `DukeBoilerplatePoke`, `DukeGarbageCollectorSweep`, `DukeAbstractFactoryOverhead`, `DukeEnterpriseThrow`;
- [x] Go rushdown em dados: `GoGoroutineJab`, `GoDeferKick`, `GoChannelOverhead`, `GoHopkick`;
- [x] C fundamentals em dados: `CPointerJab`, `CUnsafePoke`, `CNullStepKick`, `CSegfaultSweep`, `CStackOverflow`, `CInterruptVector`, `CUndefinedThrow`;
- [x] Python agile punisher em dados: `PythonSnakeBite`, `PythonDataStrike`, `PythonHeelKick`, `PythonIndentSweep`, `PythonTracebackOverhead`, `PythonVisionAntiAir`, `PythonConstrictThrow`;
- [x] especiais de projectile por personagem via `ProjectileSpec`, com Rust balanceado, Duke mais pesado/lento, Go em burst curto, C em bitstream medio/rapido e Python em fluxo rapido/medio;
- [x] demo publica ciclando Rust, Duke/Java, C e Python, mantendo Go disponivel por CLI/ferramentas;
- [x] matriz de intenção mecânica em [`docs/15-character-combat-matrix.md`](15-character-combat-matrix.md);
- [x] matriz de matchups de intenção, sem buscar balanceamento final.

Critério de aceite:

- cada personagem vence de uma forma diferente no playtest.
- Rust deve ganhar por resposta limpa e decisão correta, não por alcance bruto.
- Duke deve controlar espaço com risco real quando erra.
- Go deve pressionar com velocidade, mas sofrer por alcance e vida menores.
- C deve ganhar por alcance e dano sólido sem ficar seguro no erro.
- Python deve ganhar por punir whiff e agir cedo, não por troca bruta de dano.

### Fase 6 — Proteções contra degeneração

Status: **iniciada em corte mínimo**.

Entregáveis:

- testes para impedir loop infinito simples;
- limite de hitstun/cancel se necessário;
- cooldown e recovery para projectiles;
- [x] log de eventos de combate para reproduzir bugs.

O primeiro corte adicionou [`src/game/combat_log.rs`](../src/game/combat_log.rs), exposto por `World::combat_log`, registrando round, countdown, ataques, whiffs, hits/blocks, projectiles e fim de luta.

Critério de aceite:

- não existe sequência automática repetível que mate sem nova decisão do atacante ou defensor.

## Backlog técnico imediato

1. Usar a leitura de vantagem do Combat Lab para ajustar golpes seguros, puníveis e spacing.
2. Playtestar a matriz da demo Rust x Duke x C x Python e manter Go em validação separada por CLI/ferramentas.
3. Playtestar se a seleção mínima no menu basta ou se precisa de tela dedicada de personagem.
4. Adicionar leitura de hitbox/hurtbox por pose ou frame quando os sprites exigirem mais precisão.
5. Só depois ampliar para novos golpes especiais.

## Decisões pendentes

- Sweep, overhead, anti-air, ataques aéreos e throw continuam universais ou serão diferenciados por personagem?
- Throws terão throw tech, prioridade própria ou outra resposta defensiva?
- Projectile deve colidir com projectile?
- Haverá recurso/meter ou cooldown continua sendo o único custo?
- Combat Lab entra por CLI flag, tela de preferência ou ambos?
- Os manifests de sprite devem carregar hitbox/hurtbox por frame no futuro?
