# 15 — Matriz de Combate dos Personagens

## Objetivo

Este documento registra a intenção mecânica de cada personagem antes de buscar balanceamento fino. A matriz deve ajudar devs, designers, artistas e agentes de IA a entenderem por que um golpe existe, qual risco ele assume e qual resposta o adversário deveria ter.

O foco atual é **Prototype 0.1**. Não estamos criando combo tree, meter, throw tech, sistema competitivo ou animação final.

## Regras de Leitura

- Números são frame data de protótipo, não promessa de balanceamento final.
- Alcance é a largura da hitbox em pixels no `MoveSpec`.
- `whiff` é o lockout extra quando o golpe erra.
- Um golpe forte precisa ter resposta clara: bloquear, pular, espaçar, interromper startup ou punir whiff.
- Quando mudar um golpe, atualize [`src/combat/move_data.rs`](../src/combat/move_data.rs), testes e esta matriz.

## Identidade Atual

| Personagem | Arquétipo | Plano de jogo | Fraqueza desejada |
|---|---|---|---|
| Rust | all-rounder técnico | respostas limpas, anti-air confiável, throw rápido, dano moderado | precisa acertar decisões; não deve ganhar por alcance bruto |
| Duke / Java | midrange pressure | presença longa, golpes pesados, pressão lenta e corporativa | startup/recovery maiores; whiff precisa ser punível |
| Go | rushdown de concorrência | ações rápidas, pressão curta, ritmo alto | vida menor, alcance menor, sofre contra anti-air e espaço bem controlado |

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

Go deve parecer rápido e impaciente. Ele existe neste corte para testar se o jogo suporta um personagem que vence por aproximação e sequência curta, sem depender de alcance grande. No momento, Go ainda usa o spritesheet greybox genérico com cor própria no Combat Lab e em match real iniciado por CLI.

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

## Matchups de Intenção

| Matchup | O que deve acontecer | Risco a observar |
|---|---|---|
| Rust x Duke | Rust tenta responder e punir; Duke tenta dominar média distância. | Duke não pode ganhar neutral só por alcance; Rust não pode anular tudo com anti-air/throw rápido. |
| Rust x Go | Rust tenta controlar aproximação com respostas limpas; Go tenta entrar antes do Rust estabilizar. | Go não pode virar pressão sem resposta; Rust não pode impedir toda aproximação. |
| Duke x Go | Duke tenta manter Go fora com alcance; Go tenta punir whiffs e ocupar o corpo-a-corpo. | Se Duke erra e não é punido, Go perde identidade; se Go entra sem risco, Duke perde função. |

## Critérios de Playtest

1. Rust deve conseguir responder salto com `RustLifetimeAntiAir` sem parecer invencível.
2. Duke deve controlar mais espaço com sweep/overhead/poke, mas deve sofrer quando erra.
3. Go deve ser percebido como mais rápido, mas não como mais seguro.
4. O jogador deve conseguir explicar por que tomou dano: low, overhead, throw, anti-air ou projectile.
5. CPU x CPU deve mostrar diferença de ritmo, sem parecer dois personagens espelhados.
6. Nenhum golpe deve resolver neutral, defesa e pressão ao mesmo tempo.

## Próximos Cortes

- Decidir se Rust precisa de uma ferramenta defensiva futura como `ownership_counter`.
- Decidir se Duke precisa de um especial diferente ou se o projectile genérico ainda basta.
- Testar Go em match real via `--p1 go`/`--p2 go` antes de criar tela de seleção.
- Avaliar hitbox/hurtbox por frame quando os sprites finais começarem a limitar o tuning.
