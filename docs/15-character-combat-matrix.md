# 15 — Matriz de Combate dos Personagens

## Objetivo

Este documento registra a intenção mecânica de cada personagem antes de buscar balanceamento fino. A matriz deve ajudar devs, designers, artistas e agentes de IA a entenderem por que um golpe existe, qual risco ele assume e qual resposta o adversário deveria ter.

O foco atual é **Prototype 0.1**. Não estamos criando combo tree, meter, throw tech, sistema competitivo ou animação final.

## Regras de Leitura

- Números são frame data de protótipo, não promessa de balanceamento final.
- Alcance é a largura base da hitbox, documentada na escala antiga de tuning. No runtime `1280x720`, o `MoveSpec` multiplica esses valores por `RESOLUTION_SCALE = 4 / 3`.
- `whiff` é o lockout extra quando o golpe erra.
- Um golpe forte precisa ter resposta clara: bloquear, pular, espaçar, interromper startup ou punir whiff.
- Quando mudar um golpe próximo, atualize [`src/combat/move_data.rs`](../src/combat/move_data.rs), testes e esta matriz.
- Quando mudar um especial de projectile, atualize [`src/combat/projectile.rs`](../src/combat/projectile.rs), testes e esta matriz.

## Identidade Atual

| Personagem | Arquétipo | Plano de jogo | Fraqueza desejada |
|---|---|---|---|
| Rust | all-rounder técnico | respostas limpas, anti-air confiável, throw rápido, dano moderado | precisa acertar decisões; não deve ganhar por alcance bruto |
| Duke / Java | midrange pressure | presença longa, golpes pesados, pressão lenta e corporativa | startup/recovery maiores; whiff precisa ser punível |
| Go | rushdown de concorrência | ações rápidas, pressão curta, ritmo alto | vida menor, alcance menor, sofre contra anti-air e espaço bem controlado |
| C | low-level fundamentals | alcance honesto, dano sólido, pressão simples e punível | startup/whiff maiores; precisa confirmar espaço antes de bater |
| Python | agile punisher | startup leve, recovery curto, leitura de whiff e ritmo rápido | vida/dano menores; não deve ganhar troca bruta |

## Rust

Rust deve parecer preciso e seguro. Ele não deve ter o maior dano nem o maior alcance, mas deve responder bem quando o jogador lê corretamente a situação.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `RustBorrowJab` | checar avanço e interromper golpe lento | 7 | 4f | 48 | 4f | ficar fora do alcance, whiff punish |
| `H` | `HeavyPunch` | ferramenta média genérica, ainda sem assinatura | 16 | 11f | 96 | 10f | bloquear, pular, punir whiff |
| `V` | `Kick` | golpe baixo/médio de controle | 12 | 9f | 100 | 8f | bloquear, recuar, punir whiff |
| `S+V` | `SweepKick` | low universal de teste | 11 | 10f | 112 | 12f | defender abaixado, pular |
| frente + `H` | `OverheadPunch` | overhead universal de teste | 14 | 12f | 82 | 12f | defender em pé, interromper startup |
| `S+H` | `RustLifetimeAntiAir` | anti-air rápido e menor | 12 | 6f | 62 | 10f | baitar e punir, atacar por baixo |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `AirKick` | ataque aéreo de alcance médio | 12 | 7f | 88 | 6f | anti-air, defender em pé |
| `Q+F` | `RustOwnershipThrow` | throw rápido, curto e pouco danoso | 9 | 5f | 42 | 12f | sair do alcance, pular |

## Duke / Java

Duke deve parecer resistente e inconveniente em média distância. Ele pode ganhar mais alcance e dano, mas o custo precisa aparecer em startup e whiff recovery.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `LightPunch` | jab genérico de emergência | 8 | 5f | 58 | 4f | ficar fora do alcance, whiff punish |
| `H` | `DukeBoilerplatePoke` | normal longo e pesado de midrange | 18 | 13f | 112 | 12f | interromper startup, punir whiff |
| `V` | `Kick` | chute genérico de controle | 12 | 9f | 100 | 8f | bloquear, recuar, punir whiff |
| `S+V` | `DukeGarbageCollectorSweep` | sweep mais longo e pesado | 13 | 13f | 128 | 16f | pular, defender baixo, punir whiff |
| frente + `H` | `DukeAbstractFactoryOverhead` | overhead lento e mais ameaçador | 16 | 15f | 96 | 16f | defender em pé, jab no startup |
| `S+H` | `RisingAntiAir` | anti-air genérico, menos especializado que Rust | 13 | 7f | 70 | 14f | baitar e punir |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `AirKick` | ataque aéreo de alcance médio | 12 | 7f | 88 | 6f | anti-air, defender em pé |
| `U+O` / `U+Enter` | `DukeEnterpriseThrow` | throw mais longo e forte, bem punível | 12 | 9f | 56 | 20f | sair do alcance, pular, jab |

## Go

Go deve parecer rápido e impaciente. Ele existe neste corte para testar se o jogo suporta um personagem que vence por aproximação e sequência curta, sem depender de alcance grande. No momento, Go usa atlas placeholder próprio de luta, entrada e projectile no Combat Lab e em match real iniciado por CLI; ele fica fora do ciclo público do menu da demo enquanto a arte do Gopher é reavaliada.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `GoGoroutineJab` | jab muito rápido para iniciar pressão curta | 6 | 3f | 42 | 3f | ficar fora do alcance, usar poke mais longo |
| `H` | `HeavyPunch` | golpe pesado genérico enquanto não há assinatura | 16 | 11f | 96 | 10f | bloquear, pular, punir whiff |
| `V` | `GoDeferKick` | chute rápido de aproximação curta | 10 | 7f | 86 | 5f | bloquear, anti-air se usado em salto, punir se espaçado |
| `S+V` | `SweepKick` | low universal de teste | 11 | 10f | 112 | 12f | defender abaixado, pular |
| frente + `H` | `GoChannelOverhead` | overhead mais rápido e menos danoso | 12 | 10f | 70 | 8f | defender em pé, desafiar com jab se previsível |
| `S+H` | `RisingAntiAir` | anti-air genérico | 13 | 7f | 70 | 14f | baitar e punir |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `GoHopkick` | pressão aérea rápida e curta | 10 | 5f | 78 | 4f | anti-air, defender em pé, recuar |
| `Q+F` | `CloseThrow` | throw universal enquanto não há assinatura | 10 | 7f | 46 | 16f | sair do alcance, pular |

## C

C deve parecer direto, perigoso e um pouco arriscado. Ele tem alcance e dano acima do genérico em várias ferramentas, mas paga em whiff recovery e não deve conseguir pressionar sem acertar espaço. A leitura desejada é low-level fundamentals: ponteiro para checar, poke grande porém inseguro, sweep que ameaça chão e throw com pushback forte.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `CPointerJab` | jab de ponteiro, alcance um pouco maior e recovery maior | 8 | 5f | 64 | 5f | ficar fora do alcance, whiff punish |
| `H` | `CUnsafePoke` | poke longo e forte, explicitamente inseguro se lido | 17 | 12f | 108 | 13f | interromper startup, bloquear e recuperar espaço, punir whiff |
| `V` | `CNullStepKick` | chute estável de controle médio | 12 | 9f | 102 | 9f | bloquear, recuar, punir whiff |
| `S+V` | `CSegfaultSweep` | low forte e longo que precisa ser espaçado | 14 | 12f | 122 | 15f | defender abaixado, pular, punir whiff |
| frente + `H` | `CStackOverflow` | overhead sólido, menos lento que Duke e mais punível que Rust | 15 | 13f | 86 | 14f | defender em pé, interromper startup |
| `S+H` | `CInterruptVector` | anti-air honesto para cobrir salto previsível | 13 | 7f | 74 | 12f | baitar e punir |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `AirKick` | ataque aéreo de alcance médio | 12 | 7f | 88 | 6f | anti-air, defender em pé |
| `Q+F` | `CUndefinedThrow` | throw de pushback forte, mas mais arriscado no whiff | 11 | 7f | 50 | 18f | sair do alcance, pular, jab |

## Python

Python deve parecer ágil, precisa e oportunista. Ela não vence por dano bruto: vence por chegar antes, punir whiff e voltar a agir mais cedo. O bote da cobra cobre o jab, `Data Strike` dá confirm rápido de soco forte, e os golpes baixos/overhead devem abrir defesa por timing, não por força.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `PythonSnakeBite` | bote rápido da cobra para checar avanço | 7 | 4f | 66 | 5f | ficar fora do alcance, whiff punish |
| `H` | `PythonDataStrike` | soco forte rápido, dano menor que heavy genérico | 15 | 10f | 92 | 9f | bloquear, pular, desafiar se previsível |
| `V` | `PythonHeelKick` | chute ágil para whiff punish curto | 11 | 8f | 86 | 7f | bloquear, recuar, punir se espaçado |
| `S+V` | `PythonIndentSweep` | low rápido e menos danoso | 10 | 9f | 98 | 10f | defender abaixado, pular |
| frente + `H` | `PythonTracebackOverhead` | overhead rápido/moderado para abrir crouch | 13 | 11f | 78 | 11f | defender em pé, jab no startup |
| `S+H` | `PythonVisionAntiAir` | anti-air rápido de leitura, alcance menor | 11 | 6f | 68 | 9f | baitar e punir, atacar por baixo |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `AirKick` | ataque aéreo de alcance médio | 12 | 7f | 88 | 6f | anti-air, defender em pé |
| `Q+F` | `PythonConstrictThrow` | throw moderado com recovery menor que o universal | 10 | 7f | 52 | 14f | sair do alcance, pular, jab |

## Especiais de Projectile

Os projectiles são `ProjectileSpec` por personagem, não `MoveSpec`. O input ainda é o mesmo botão de especial do protótipo, mas dano, tamanho, velocidade, cooldown, reação e limite de alcance já vêm do `CharacterSpec`. Velocidade e alcance abaixo seguem os valores base de tuning; no runtime `1280x720`, medidas espaciais usam `RESOLUTION_SCALE = 4 / 3`.

| Personagem | Spec | Intenção | Dano | Velocidade | Cooldown | Alcance | Contra-jogo |
|---|---|---|---:|---:|---:|---|---|
| Rust | `RUST_PROJECTILE_SPEC` | gear honesto de controle médio, sem ganhar por spam | 8 | 340 px/s | 57f | tela inteira | bloquear chip, pular, aproximar no cooldown |
| Duke / Java | `DUKE_PROJECTILE_SPEC` | bean pesado para ocupar espaço e forçar resposta lenta | 10 | 270 px/s | 72f | tela inteira | pular mais cedo, ganhar terreno durante recovery, punir se Duke erra leitura |
| Go | `GO_PROJECTILE_SPEC` | burst rápido para cobrir entrada sem virar zoner | 6 | 430 px/s | 44f | 320 px | ficar fora do curto alcance, bloquear pouco dano, desafiar depois do burst |
| C | `C_PROJECTILE_SPEC` | bitstream médio/rápido para validar asset separado e origem do especial | 8 | 360 px/s | 56f | tela inteira | bloquear chip, pular, aproximar no cooldown |
| Python | `PYTHON_PROJECTILE_SPEC` | fluxo de dados rápido/médio para validar atlas novo sem virar zoner completo | 7 | 390 px/s | 50f | tela inteira | bloquear pouco dano, pular, desafiar antes do próximo fluxo |

## Matchups de Intenção

| Matchup | O que deve acontecer | Risco a observar |
|---|---|---|
| Rust x Duke | Rust tenta responder e punir; Duke tenta dominar média distância. | Duke não pode ganhar neutral só por alcance; Rust não pode anular tudo com anti-air/throw rápido. |
| Rust x Go | Rust tenta controlar aproximação com respostas limpas; Go tenta entrar antes do Rust estabilizar. | Go não pode virar pressão sem resposta; Rust não pode impedir toda aproximação. |
| Duke x Go | Duke tenta manter Go fora com alcance; Go tenta punir whiffs e ocupar o corpo-a-corpo. | Se Duke erra e não é punido, Go perde identidade; se Go entra sem risco, Duke perde função. |
| C x Rust | C tenta ganhar alcance e dano sólido; Rust tenta whiff punish e anti-air mais limpo. | C não pode ficar seguro demais no poke; Rust não pode anular todo alcance com jab/throw. |
| C x Duke | C joga fundamentos contra presença pesada; Duke tem golpes maiores, mas C deve punir melhor whiffs longos. | Se Duke sempre ganha alcance e dano, C vira redundante; se C é mais rápido e mais forte, Duke perde função. |
| C x Python | C ganha trocas e espaço; Python tenta entrar e sair antes do whiff punish. | Python não pode vencer troca bruta; C não pode prender Python sem risco. |
| Python x Rust | Python tenta acelerar o ritmo e punir decisão errada; Rust tenta estabilizar com respostas honestas. | Python não pode virar Rust melhor e mais rápido; Rust não pode impedir todo whiff punish. |
| Python x Duke | Python tenta passar por startup longo; Duke tenta manter presença com dano e pushback. | Se Python entra sem risco, Duke perde arquétipo; se Duke controla tudo, Python não joga. |

## Critérios de Playtest

1. Rust deve conseguir responder salto com `RustLifetimeAntiAir` sem parecer invencível.
2. Duke deve controlar mais espaço com sweep/overhead/poke, mas deve sofrer quando erra.
3. Go deve ser percebido como mais rápido, mas não como mais seguro.
4. C deve parecer mais sólido e comprido, mas punível quando erra.
5. O jogador deve conseguir explicar por que tomou dano: low, overhead, throw, anti-air ou projectile.
6. CPU x CPU deve mostrar diferença de ritmo, sem parecer dois personagens espelhados.
7. Nenhum golpe deve resolver neutral, defesa e pressão ao mesmo tempo.
8. Python deve parecer ágil e clara no feedback de hit, sem ganhar por dano bruto.

## Próximos Cortes

- Decidir se Rust precisa de uma ferramenta defensiva futura como `ownership_counter`.
- Playtestar se o `DUKE_PROJECTILE_SPEC` pesado abre espaço sem virar spam lento sem resposta.
- Playtestar se o `GO_PROJECTILE_SPEC` curto ajuda aproximação sem transformar Go em zoner.
- Playtestar C e Python contra Rust/Duke antes de mexer em vida ou dano.
- Avaliar hitbox/hurtbox por frame quando os sprites finais começarem a limitar o tuning.
