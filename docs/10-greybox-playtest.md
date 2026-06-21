# 10 — Greybox Playtest

## Status

Branch de trabalho atual: `feature/projectile-and-smoother-movement`.

Este é o primeiro código jogável do projeto. O objetivo não é parecer bonito; é provar que o loop básico de luta existe e pode ser discutido por gameplay, arte, produção e engenharia.

## O que já existe

- Janela Raylib.
- Loop de jogo com fixed timestep.
- Arena bitmap placeholder `Terminal Arcade + Compiler Lab`.
- Tela inicial de preferências com feature flags runtime.
- Dois lutadores greybox: Rust e Java.
- Corpo composto por cabeça, tronco e pernas placeholder.
- Spritesheet placeholder com poses de idle, andar, abaixar, pular, defender, socos e chute.
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
- CPU de playtest para um ou dois jogadores, com perfis diferentes e acoes variadas.
- Opção para IA mover/defender sem dar golpes.
- Opção para Player 1 não receber dano.
- Movimento com aceleração/desaceleração.
- Hurtbox visível quando debug de combate está ligado.
- Hitbox/alcance dos golpes visível quando debug de combate está ligado.
- Dano fixo.
- Barra de vida com número.
- Hit spark, block spark e dano flutuante.
- Condição de vitória.
- Reinício da partida.
- HUD, ajuda de controles e debug visual configuráveis.
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

## Preferências

O jogo abre primeiro uma tela de preferências. Use `Setas` ou `W/S` para navegar, `Espaço` para ligar/desligar uma opção e `Enter` para começar ou voltar para a luta. Durante a luta, `Esc` volta para essa tela.

| Preferência | Padrão | O que testar |
|---|---|---|
| Player 1 usa IA | Desligado | Rust deve ser controlado automaticamente quando ligado. |
| Player 2 usa IA | Ligado | Java deve ser controlado automaticamente. |
| IA pode dar golpes | Ligado | Quando desligado, lutadores controlados por IA devem andar, pular, afastar, aproximar e defender, mas não atacar. |
| Player 1 recebe dano | Ligado | Quando desligado, Rust não deve perder vida ao ser acertado. |
| Mostrar HUD | Ligado | Barras de vida e status no topo aparecem/desaparecem. |
| Mostrar ajuda de controles | Desligado | Texto de controles no rodapé aparece/desaparece. |
| Mostrar debug de combate | Desligado | Hitboxes, hurtboxes, labels e colisão corpo-corpo aparecem/desaparecem. |
| Entrada por gamepad | Ligado | Gamepads detectados pelo Raylib podem controlar o jogo. |

## Controles

| Ação | Rust / Player 1 | Java / Player 2 | Gamepad Xbox |
|---|---|---|---|
| Mover esquerda | `A` | `←` ou `J` | Left stick ou D-pad |
| Mover direita | `D` | `→` ou `L` | Left stick ou D-pad |
| Pular | `W` | `↑` ou `I` | `A` |
| Abaixar | `S` | `↓` ou `K` | Left stick para baixo ou D-pad baixo |
| Defender | `Q` | `U` | `LB` ou `LT` |
| Soco fraco / curto | `F` | `O` ou `Enter` | `X` |
| Soco forte / longo | `H` | `P` ou `Right Shift` | `Y` |
| Chute | `V` | `;` ou `/` | `B` |
| Fireball | `G` | `Right Ctrl` ou `KP0` | `RB` |
| Alternar P2 CPU/manual | `C` | `C` | `View` |
| Reiniciar | `R` | `R` | `Menu` |

O primeiro gamepad conectado controla o Player 1 quando a IA do Player 1 está desligada. O segundo gamepad controla o Player 2 quando a IA do Player 2 está desligada. O Player 2 começa em modo CPU; quando a CPU de um jogador está ligada, os comandos manuais daquele jogador são ignorados.

Quando ambos os jogadores usam IA, Rust e Java usam perfis diferentes para evitar movimentos espelhados. Rust tende a preservar mais média distância e usar especial com mais frequência; Java tende a pressionar mais de perto. A IA decide em pequenos blocos de tempo e pode andar, afastar, pular, abaixar, bloquear, socar, chutar e soltar especial.

O HUD mostra `Pad P1` e `P2` como `ON` quando Raylib detecta o controle. Se um controle Bluetooth estiver pareado mas aparecer `OFF`, confirme se o sistema que executa `cargo run` expõe joystick/gamepad para o Raylib. Em WSL ou ambiente remoto, pode ser necessário testar no host nativo ou encaminhar o dispositivo.

## Como ler a tela

| Elemento | Significado |
|---|---|
| Partes azuis | Rust / Player 1 |
| Partes laranja | Java / Player 2 |
| Braços e pernas do sprite | Pose/ação atual sem depender do debug |
| Outline branco | Corpo físico do personagem |
| Caixas verdes | Hurtboxes de cabeça, tronco e pernas |
| Caixa vermelha | Alcance do golpe corpo-a-corpo |
| Caixa/círculo ciano | Fireball |
| Corpo amarelo | Ataque em fase ativa |
| Hit spark amarelo | Golpe acertou |
| Escudo/spark azul | Defesa reduziu dano |
| `-8`, `-12`, `-16` | Dano aplicado |
| Linha magenta | Colisão corpo-corpo bloqueando passagem |
| Fundo Terminal Compiler Lab | Arena placeholder, não arte final |

Hitboxes, hurtboxes, labels de golpe e linha de colisão aparecem somente com `Mostrar debug de combate` ligado. A ajuda de comandos no rodapé aparece somente com `Mostrar ajuda de controles` ligado.

## O que testar agora

1. Um jogador não deve atravessar o outro.
2. A arena bitmap deve ajudar o mood sem esconder lutadores, hitboxes, hurtboxes ou HUD.
3. Soco fraco deve ser mais curto e mais rápido.
4. Soco forte deve alcançar mais longe e causar mais dano.
5. Chute deve acertar em uma altura mais baixa.
6. Defesa deve reduzir dano e mostrar feedback azul.
7. Abaixar deve reduzir a hurtbox visualmente.
8. Fireball deve andar horizontalmente em velocidade legível e causar dano ao acertar.
9. A CPU do Player 2 deve variar aproximação, afastamento, pulo, ataques, defesa e fireballs.
10. A tela de preferências deve ligar/desligar HUD, ajuda e debug sem reiniciar o jogo.
11. A opção `Player 1 usa IA` ligada deve permitir CPU x CPU quando `Player 2 usa IA` tambem estiver ligada.
12. A opção `IA pode dar golpes` desligada deve impedir soco, chute e fireball da CPU, mantendo movimento/defesa.
13. A opção `Player 1 recebe dano` desligada deve impedir perda de vida do Rust.
14. Gamepad Xbox deve controlar o Player 1 com left stick/D-pad, `A`, `X`, `Y`, `B`, `LB/LT` e `RB` quando o ambiente expõe controle ao Raylib.
15. `C` ou `View` deve alternar entre CPU e controle manual do Player 2.
16. `R` ou `Menu` deve reiniciar a partida.
17. `Esc` durante a luta deve voltar para a tela de preferências.
18. Pulo com direção pressionada deve sair em diagonal.
19. A vida deve chegar a zero e encerrar a luta.
20. O feedback visual deve deixar claro quando houve contato físico, golpe, bloqueio e projétil.

## Limitações conhecidas

- Os dois personagens ainda compartilham o mesmo kit de golpes.
- A arena bitmap é placeholder gerado por IA e não deve ser tratada como arte final.
- O spritesheet de lutador é placeholder gerado localmente com formas simples e não deve ser tratado como arte final.
- Fireball no gamepad usa `RB` por enquanto; `RT` pode entrar depois quando tivermos leitura de gatilho com borda de pressionamento.
- Defesa é um experimento mínimo: reduz dano, mas ainda não tem direção, high/low guard ou pushback.
- A CPU é um sparring dummy determinístico: decide em pequenos blocos de tempo, usa perfis diferentes por slot, varia movimento/ataque/especial/defesa e reage a projéteis sem ser perfeita.
- Não há combo, agarrão, especial avançado, hitstun real, knockback ou IA adaptativa.
- Não há animação final, sprites, áudio, pausa ou IA avançada.
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

- Usar este greybox para testar silhueta, proporção de braços/pernas e leitura de ataque sem overlays.
- Propor placeholders melhores sem perder legibilidade das caixas.
- Começar mood candidato usando `docs/templates/mood-proposal.md`.
