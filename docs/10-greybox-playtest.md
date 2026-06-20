# 10 — Greybox Playtest

## Status

Branch de trabalho atual: `feature/projectile-and-smoother-movement`.

Este é o primeiro código jogável do projeto. O objetivo não é parecer bonito; é provar que o loop básico de luta existe e pode ser discutido por gameplay, arte, produção e engenharia.

## O que já existe

- Janela Raylib.
- Loop de jogo com fixed timestep.
- Dois lutadores greybox: Rust e Java.
- Corpo composto por cabeça, tronco e pernas placeholder.
- Movimento horizontal local.
- Pulo simples e pulo diagonal com momentum.
- Abaixar com hurtbox menor.
- Defesa com redução de dano.
- Arena com chão e limites.
- Colisão física corpo-corpo com gap mínimo.
- Soco fraco/curto.
- Soco forte/longo.
- Chute.
- Fireball horizontal simples em velocidade legível.
- CPU simples e pouco agressiva para o Player 2.
- Movimento com aceleração/desaceleração.
- Hurtbox visível.
- Hitbox/alcance dos golpes visível.
- Dano fixo.
- Barra de vida com número.
- Hit spark, block spark e dano flutuante.
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
| Mover esquerda | `A` | `←` ou `J` |
| Mover direita | `D` | `→` ou `L` |
| Pular | `W` | `↑` ou `I` |
| Abaixar | `S` | `↓` ou `K` |
| Defender | `Q` | `U` |
| Soco fraco / curto | `F` | `O` ou `Enter` |
| Soco forte / longo | `H` | `P` ou `Right Shift` |
| Chute | `V` | `;` ou `/` |
| Fireball | `G` | `Right Ctrl` ou `KP0` |
| Alternar P2 CPU/manual | `C` | `C` |
| Reiniciar | `R` | `R` |

O Player 2 começa em modo CPU. Quando CPU está ligada, os comandos manuais do Player 2 são ignorados.

## Como ler a tela

| Elemento | Significado |
|---|---|
| Partes azuis | Rust / Player 1 |
| Partes laranja | Java / Player 2 |
| Outline branco | Corpo físico do personagem |
| Caixas verdes | Hurtboxes de cabeça, tronco e pernas |
| Caixa vermelha | Alcance do golpe corpo-a-corpo |
| Caixa/círculo ciano | Fireball |
| Corpo amarelo | Ataque em fase ativa |
| Hit spark amarelo | Golpe acertou |
| Escudo/spark azul | Defesa reduziu dano |
| `-8`, `-12`, `-16` | Dano aplicado |
| Linha magenta | Colisão corpo-corpo bloqueando passagem |

## O que testar agora

1. Um jogador não deve atravessar o outro.
2. Soco fraco deve ser mais curto e mais rápido.
3. Soco forte deve alcançar mais longe e causar mais dano.
4. Chute deve acertar em uma altura mais baixa.
5. Defesa deve reduzir dano e mostrar feedback azul.
6. Abaixar deve reduzir a hurtbox visualmente.
7. Fireball deve andar horizontalmente em velocidade legível e causar dano ao acertar.
8. A CPU do Player 2 deve se aproximar, hesitar antes de atacar e defender fireballs próximas.
9. `C` deve alternar entre CPU e controle manual do Player 2.
10. Pulo com direção pressionada deve sair em diagonal.
11. A vida deve chegar a zero e encerrar a luta.
12. `R` deve reiniciar a partida.
13. O feedback visual deve deixar claro quando houve contato físico, golpe, bloqueio e projétil.

## Limitações conhecidas

- Os dois personagens ainda compartilham o mesmo kit de golpes.
- Defesa é um experimento mínimo: reduz dano, mas ainda não tem direção, high/low guard ou pushback.
- A CPU é um sparring dummy determinístico: aproxima com cautela, ataca depois de delay, solta fireball em média distância e bloqueia projétil próximo.
- Não há combo, agarrão, especial avançado, hitstun real, knockback ou IA adaptativa.
- Não há animação final, sprites, áudio, menu, pausa ou IA avançada.
- O balanceamento ainda não importa.
- A colisão é propositalmente simples e axis-aligned.
- O visual é debug/greybox, não direção de arte final.

## Caminho sugerido

### Próximo passo técnico

- Refinar sensação de movimento.
- Adicionar knockback simples.
- Separar melhor estados de ataque/hitstun se o kit atual for aceito.
- Criar testes para vitória/restart e bordas da arena.
- Ajustar heurísticas da CPU depois de playtest manual.

### Próximo passo de gameplay

- Decidir se soco fraco, soco forte e chute comunicam alcance/dano diferentes.
- Decidir se defesa e abaixar entram no Prototype 0.1 final ou ficam como experimento de 0.2.
- Definir o primeiro diferencial mecânico mínimo de Rust e Java.

### Próximo passo de arte

- Usar este greybox para testar silhueta.
- Propor placeholders melhores sem perder legibilidade das caixas.
- Começar mood candidato usando `docs/templates/mood-proposal.md`.
