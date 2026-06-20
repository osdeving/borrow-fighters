# 10 — Greybox Playtest

## Status

Branch: `feature/greybox-vertical-slice`.

Este é o primeiro código jogável do projeto. O objetivo não é parecer bonito; é provar que o loop básico de luta existe e pode ser discutido por gameplay, arte, produção e engenharia.

## O que já existe

- Janela Raylib.
- Loop de jogo com fixed timestep.
- Dois lutadores greybox: Rust e Java.
- Movimento horizontal local.
- Pulo simples.
- Arena com chão e limites.
- Colisão física corpo-corpo com gap mínimo.
- Soco básico.
- Fireball horizontal simples.
- Movimento com aceleração/desaceleração e pulo diagonal.
- Hurtbox visível.
- Hitbox/alcance do soco visível.
- Dano fixo.
- Barra de vida com número.
- Hit spark e dano flutuante.
- Condição de vitória.
- Reinício da partida.
- Testes de regras de combate sem abrir janela.

## Como rodar

Requisitos:

- Rust estável.
- Dependências nativas exigidas por Raylib/raylib-rs no seu sistema operacional.

Comando:

```bash
cargo run
```

Checks úteis:

```bash
cargo fmt
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

O GitHub também roda `Rust Check` no PR para validar formatação, testes e clippy em Linux.

## Controles

| Ação | Rust / Player 1 | Java / Player 2 |
|---|---|---|
| Mover esquerda | `A` | `←` |
| Mover direita | `D` | `→` |
| Pular | `W` | `↑` |
| Soco | `F` | `Enter` |
| Fireball | `G` | `Right Shift` |
| Reiniciar | `R` | `R` |

## Como ler a tela

| Elemento | Significado |
|---|---|
| Corpo azul | Rust / Player 1 |
| Corpo laranja | Java / Player 2 |
| Outline branco | Corpo físico do personagem |
| Caixa verde | Hurtbox, área vulnerável |
| Caixa vermelha | Alcance do soco |
| Caixa/círculo ciano | Fireball |
| Corpo amarelo | Ataque em fase ativa |
| Hit spark amarelo | Soco acertou |
| `-12` | Dano aplicado |
| Linha magenta | Colisão corpo-corpo bloqueando passagem |

## O que testar agora

1. Um jogador não deve atravessar o outro.
2. O soco deve tirar vida apenas quando a hitbox ativa encosta na hurtbox.
3. Cada soco deve aplicar dano uma vez.
4. Fireball deve andar horizontalmente e causar dano ao acertar.
5. Pulo com direção pressionada deve sair em diagonal.
6. A vida deve chegar a zero e encerrar a luta.
7. `R` deve reiniciar a partida.
8. O feedback visual deve deixar claro quando houve contato físico e quando houve golpe.

## Limitações conhecidas

- Só existe um soco básico e uma fireball simples por personagem.
- Não há bloqueio, combo, agarrão, especial, hitstun real ou knockback.
- Não há animação final, sprites, áudio, menu, pausa ou IA.
- O balanceamento ainda não importa.
- A colisão é propositalmente simples e axis-aligned.
- O visual é debug/greybox, não direção de arte final.

## Caminho sugerido

### Próximo passo técnico

- Refinar sensação de movimento.
- Adicionar knockback simples.
- Separar melhor estados de ataque/hitstun se o soco atual for aceito.
- Criar testes para vitória/restart e bordas da arena.

### Próximo passo de gameplay

- Decidir se o soco básico comunica bem contato e dano.
- Definir o primeiro diferencial mecânico mínimo de Rust e Java.
- Escolher se Prototype 0.1 precisa de segundo ataque ou se isso fica para 0.2.

### Próximo passo de arte

- Usar este greybox para testar silhueta.
- Propor placeholders melhores sem perder legibilidade das caixas.
- Começar mood candidato usando `docs/templates/mood-proposal.md`.
