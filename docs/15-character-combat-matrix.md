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
| C | low-level fundamentals placeholder | validar atlas fluido, escala humanoide e projectile de bitstream | ainda sem identidade própria; não deve ser tratado como balanceado |

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

Go deve parecer rápido e impaciente. Ele existe neste corte para testar se o jogo suporta um personagem que vence por aproximação e sequência curta, sem depender de alcance grande. No momento, Go usa atlas placeholder próprio de luta, entrada e projectile no Combat Lab e em match real iniciado pelo menu ou CLI.

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

C entrou primeiro como teste de pipeline: dois atlas de referência com chroma key, mais frames por animação, entrada cinematográfica curta e projectile separado. A intenção futura pode ir para fundamentos de baixo nível, ponteiros, bitstream e risco de `segfault`, mas este corte ainda usa golpes genéricos para validar escala, pivots, leitura de animação e integração no menu/lab/luta.

| Input | MoveId | Intenção | Dano | Startup | Alcance | Whiff | Contra-jogo |
|---|---|---|---:|---:|---:|---:|---|
| `F` | `LightPunch` | jab genérico enquanto a assinatura não existe | 8 | 5f | 58 | 4f | ficar fora do alcance, whiff punish |
| `H` | `HeavyPunch` | golpe pesado genérico | 16 | 11f | 96 | 10f | bloquear, pular, punir whiff |
| `V` | `Kick` | chute genérico de controle | 12 | 9f | 100 | 8f | bloquear, recuar, punir whiff |
| `S+V` | `SweepKick` | low universal de teste | 11 | 10f | 112 | 12f | defender abaixado, pular |
| frente + `H` | `OverheadPunch` | overhead universal de teste | 14 | 12f | 82 | 12f | defender em pé, interromper startup |
| `S+H` | `RisingAntiAir` | anti-air genérico | 13 | 7f | 70 | 14f | baitar e punir |
| no ar + `F/H` | `AirPunch` | ataque aéreo leve universal | 9 | 5f | 72 | 6f | anti-air, andar fora |
| no ar + `V` | `AirKick` | ataque aéreo de alcance médio | 12 | 7f | 88 | 6f | anti-air, defender em pé |
| `Q+F` | `CloseThrow` | throw universal enquanto não há assinatura | 10 | 7f | 46 | 16f | sair do alcance, pular |

## Especiais de Projectile

Os projectiles são `ProjectileSpec` por personagem, não `MoveSpec`. O input ainda é o mesmo botão de especial do protótipo, mas dano, tamanho, velocidade, cooldown, reação e limite de alcance já vêm do `CharacterSpec`. Velocidade e alcance abaixo seguem os valores base de tuning; no runtime `1280x720`, medidas espaciais usam `RESOLUTION_SCALE = 4 / 3`.

| Personagem | Spec | Intenção | Dano | Velocidade | Cooldown | Alcance | Contra-jogo |
|---|---|---|---:|---:|---:|---|---|
| Rust | `RUST_PROJECTILE_SPEC` | gear honesto de controle médio, sem ganhar por spam | 8 | 340 px/s | 57f | tela inteira | bloquear chip, pular, aproximar no cooldown |
| Duke / Java | `DUKE_PROJECTILE_SPEC` | bean pesado para ocupar espaço e forçar resposta lenta | 10 | 270 px/s | 72f | tela inteira | pular mais cedo, ganhar terreno durante recovery, punir se Duke erra leitura |
| Go | `GO_PROJECTILE_SPEC` | burst rápido para cobrir entrada sem virar zoner | 6 | 430 px/s | 44f | 320 px | ficar fora do curto alcance, bloquear pouco dano, desafiar depois do burst |
| C | `C_PROJECTILE_SPEC` | bitstream médio/rápido para validar asset separado e origem do especial | 8 | 360 px/s | 56f | tela inteira | bloquear chip, pular, aproximar no cooldown |

## Matchups de Intenção

| Matchup | O que deve acontecer | Risco a observar |
|---|---|---|
| Rust x Duke | Rust tenta responder e punir; Duke tenta dominar média distância. | Duke não pode ganhar neutral só por alcance; Rust não pode anular tudo com anti-air/throw rápido. |
| Rust x Go | Rust tenta controlar aproximação com respostas limpas; Go tenta entrar antes do Rust estabilizar. | Go não pode virar pressão sem resposta; Rust não pode impedir toda aproximação. |
| Duke x Go | Duke tenta manter Go fora com alcance; Go tenta punir whiffs e ocupar o corpo-a-corpo. | Se Duke erra e não é punido, Go perde identidade; se Go entra sem risco, Duke perde função. |
| C x qualquer | C valida leitura visual e integração do atlas sem ainda prometer matchup. | Não tirar conclusão de balanceamento até C receber golpes próprios. |

## Critérios de Playtest

1. Rust deve conseguir responder salto com `RustLifetimeAntiAir` sem parecer invencível.
2. Duke deve controlar mais espaço com sweep/overhead/poke, mas deve sofrer quando erra.
3. Go deve ser percebido como mais rápido, mas não como mais seguro.
4. C deve aparecer em escala coerente com Rust/Duke e com projectile saindo de ponto visualmente defensável.
5. O jogador deve conseguir explicar por que tomou dano: low, overhead, throw, anti-air ou projectile.
6. CPU x CPU deve mostrar diferença de ritmo, sem parecer dois personagens espelhados.
7. Nenhum golpe deve resolver neutral, defesa e pressão ao mesmo tempo.

## Próximos Cortes

- Decidir se Rust precisa de uma ferramenta defensiva futura como `ownership_counter`.
- Playtestar se o `DUKE_PROJECTILE_SPEC` pesado abre espaço sem virar spam lento sem resposta.
- Playtestar se o `GO_PROJECTILE_SPEC` curto ajuda aproximação sem transformar Go em zoner.
- Criar identidade mecânica real para C antes de balancear matchups.
- Avaliar hitbox/hurtbox por frame quando os sprites finais começarem a limitar o tuning.
