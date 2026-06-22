# 10 — Greybox Playtest

## Status

Este é o primeiro código jogável do projeto. O objetivo não é parecer bonito; é provar que o loop básico de luta existe e pode ser discutido por gameplay, arte, produção e engenharia.

## O que já existe

- Janela Raylib.
- Loop de jogo com fixed timestep.
- Arenas bitmap placeholder `Sirius`, `Fortaleza Tech Coast` e `Java Street`.
- Tela inicial de preferências com feature flags runtime.
- Dois lutadores greybox na luta padrão: Rust e Java.
- Corpo composto por cabeça, tronco e pernas placeholder.
- Spritesheet placeholder com poses de idle, andar, abaixar, pular, defender, socos e chute.
- Movimento horizontal local.
- Pulo simples e pulo diagonal com momentum.
- Abaixar com hurtbox menor.
- Defesa com redução de dano.
- Arena com chão, limites e rotação de cenário ao iniciar a próxima luta após uma vitória.
- Entrada cinematográfica seguida de contagem pré-luta `11`, `10`, `01`, `Fight!`.
- Colisão física corpo-corpo com gap mínimo.
- Soco fraco/curto.
- Soco forte/longo.
- Chute.
- Varredura baixa, overhead, anti-air, agarrão curto e ataques aéreos.
- Primeiro corte de identidade mecânica: Rust com respostas mais rápidas/curtas; Duke com ferramentas mais longas/pesadas e mais puníveis; Go como rushdown greybox testável no Combat Lab e em match via CLI.
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
- Combat Lab com reprodução de golpes e poses estáticas de inspeção.

## Como rodar

Requisitos:

- Rust estável.
- Dependências nativas exigidas por Raylib/raylib-rs no seu sistema operacional.

Comando:

```bash
cargo run
cargo run -- --fight --p1 go --p2 duke
```

Use `--p1`/`--player-one` e `--p2`/`--player-two` para iniciar matchups específicos sem tela de seleção. Valores aceitos: `rust`, `duke`, `java`, `go`, `golang` e `gopher`. Use `--fight` ou `--skip-menu` para abrir diretamente na luta.

Checks úteis:

```bash
cargo fmt
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

O GitHub também roda `Rust Check` no PR para validar formatação, testes e clippy em Linux.

## Combat Lab

Para abrir uma cena limpa de inspeção de golpe:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move sweep
cargo run -- --lab combat --character duke --move anti-air
cargo run -- --lab combat --character go --move kick
```

Para abrir uma pose estática:

```bash
cargo run -- --lab combat --character rust --pose crouch
cargo run -- --lab combat --character duke --pose victory
cargo run -- --lab combat --character go --pose jump
```

No Combat Lab, `Tab` / `Shift+Tab` alterna golpes, `PageDown` / `PageUp` alterna poses, `Enter` reinicia, `Espaço` pausa, `.` avança um frame quando pausado, `Home` volta ao frame 0, `H` alterna hurtbox, `B` alterna hitbox, `P` alterna pivot/eixos, `D` alterna dummy e `A` alterna o fundo de arena.

O Combat Lab abre com o fundo `Sirius` ligado para validar contraste de golpe/sprite contra cenário. Use `A` para remover o fundo e voltar ao grid limpo.

## Preferências

O jogo abre primeiro uma tela de preferências. Use `Setas` ou `W/S` para navegar, `Espaço` para ligar/desligar uma opção e `Enter` para começar ou voltar para a luta. Durante a luta, `Esc` volta para essa tela.

Ao começar uma luta, os personagens entram em cena e depois aparece a contagem central `11`, `10`, `01`, `Fight!`. Enquanto a intro ou a contagem estiver ativa, ataques, movimento e projéteis ficam bloqueados. Depois que alguém vence, o cenário permanece o mesmo durante a pose final; a próxima arena só entra quando a luta seguinte começa com `R`/`Start` ou ao voltar da tela de preferências.

| Preferência | Padrão | O que testar |
|---|---|---|
| Player 1 usa IA | Desligado | Rust deve ser controlado automaticamente quando ligado. |
| Player 2 usa IA | Ligado | Java deve ser controlado automaticamente. |
| IA pode dar golpes | Ligado | Quando desligado, lutadores controlados por IA devem andar, pular, afastar, aproximar e defender, mas não atacar. |
| Player 1 recebe dano | Ligado | Quando desligado, Rust não deve perder vida ao ser acertado. |
| Player 2 recebe dano | Ligado | Quando desligado, Java não deve perder vida ao ser acertado. |
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
| Varredura baixa | `S` + `V` | `↓`/`K` + `;`/`/` | Baixo + `B` |
| Anti-air | `S` + `H` | `↓`/`K` + `P`/`Right Shift` | Baixo + `Y` |
| Overhead | Frente + `H` | Frente + `P`/`Right Shift` | Frente + `Y` |
| Agarrão curto | `Q` + `F` | `U` + `O`/`Enter` | `LB`/`LT` + `X` |
| Ataque aéreo | No ar: `F` ou `V` | No ar: `O`/`Enter` ou `;`/`/` | No ar: `X` ou `B` |
| Fireball | `G` | `Right Ctrl` ou `KP0` | `RB` |
| Alternar P2 CPU/manual | `C` | `C` | `View` |
| Reiniciar | `R` | `R` | `Menu` |

O primeiro gamepad conectado controla o Player 1 quando a IA do Player 1 está desligada. O segundo gamepad controla o Player 2 quando a IA do Player 2 está desligada. O Player 2 começa em modo CPU; quando a CPU de um jogador está ligada, os comandos manuais daquele jogador são ignorados.

Quando ambos os jogadores usam IA, Rust e Java usam perfis diferentes para evitar movimentos espelhados. Rust tende a preservar mais média distância e usar especial com mais frequência; Java tende a pressionar mais de perto. A IA decide em pequenos blocos de tempo e pode andar, afastar, pular, abaixar, bloquear, socar, chutar, tentar varredura, overhead, anti-air, agarrão curto, ataque aéreo e soltar especial.

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
| Fundo Sirius/Fortaleza/Java Street | Arena placeholder, não arte final |

Hitboxes, hurtboxes, labels de golpe e linha de colisão aparecem somente com `Mostrar debug de combate` ligado. A ajuda de comandos no rodapé aparece somente com `Mostrar ajuda de controles` ligado.

## O que testar agora

1. Um jogador não deve atravessar o outro.
2. A arena bitmap deve ajudar o mood sem esconder lutadores, hitboxes, hurtboxes ou HUD.
3. Soco fraco deve ser mais curto e mais rápido.
4. Soco forte deve alcançar mais longe e causar mais dano.
5. Chute deve acertar em uma altura mais baixa.
6. Varredura baixa deve exigir defesa abaixada.
7. Overhead deve exigir defesa em pé.
8. Agarrão curto deve ignorar defesa, mas errar fora de alcance.
9. Anti-air deve cobrir região acima/frente do personagem.
10. Ataques aéreos devem funcionar durante salto sem travar a queda.
11. Defesa deve reduzir dano e mostrar feedback azul.
12. Abaixar deve reduzir a hurtbox visualmente.
13. Fireball deve andar horizontalmente em velocidade legível e causar dano ao acertar.
14. A CPU do Player 2 deve variar aproximação, afastamento, pulo, socos, chutes, varredura, overhead, anti-air, agarrão curto, ataque aéreo, defesa e fireballs.
15. Rust deve parecer mais responsivo em anti-air e throw.
16. Duke deve controlar mais espaço com sweep, overhead e poke, mas ficar mais exposto quando erra.
17. Go no Combat Lab e na luta iniciada por `--p1 go` ou `--p2 go` deve parecer mais rápido e curto que os golpes genéricos equivalentes, pagando com menos vida.
18. A tela de preferências deve ligar/desligar HUD, ajuda e debug sem reiniciar o jogo.
19. A opção `Player 1 usa IA` ligada deve permitir CPU x CPU quando `Player 2 usa IA` tambem estiver ligada.
20. A opção `IA pode dar golpes` desligada deve impedir soco, chute e fireball da CPU, mantendo movimento/defesa.
21. A opção `Player 1 recebe dano` desligada deve impedir perda de vida do Rust.
22. A opção `Player 2 recebe dano` desligada deve impedir perda de vida do Java.
23. Gamepad Xbox deve controlar o Player 1 com left stick/D-pad, `A`, `X`, `Y`, `B`, `LB/LT` e `RB` quando o ambiente expõe controle ao Raylib.
24. `C` ou `View` deve alternar entre CPU e controle manual do Player 2.
25. `R` ou `Menu` deve reiniciar a partida.
26. `Esc` durante a luta deve voltar para a tela de preferências.
27. Pulo com direção pressionada deve sair em diagonal.
28. A vida deve chegar a zero e encerrar a luta.
29. Ao iniciar a próxima luta depois de uma vitória, o cenário deve avançar uma vez no ciclo `Sirius -> Fortaleza Tech Coast -> Java Street -> Sirius`.
30. O feedback visual deve deixar claro quando houve contato físico, golpe, bloqueio e projétil.

## Combat Lab

Para testar um golpe sem iniciar a luta completa:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move overhead
cargo run -- --lab combat --character duke --move throw
cargo run -- --lab combat --character go --move light_punch
```

Use o Combat Lab para verificar:

- pivot e linha do chão;
- hurtbox do personagem;
- hitbox ativa e inativa;
- frame atual, fase e janela ativa do golpe;
- spawn, altura e trajetória inicial do projectile.
- high/low/throw obedecendo as regras de defesa.

Rastreio técnico do Combat Lab, hitbox/hurtbox e arquivos de combate: [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md).

Controles do lab:

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
| Alternar fundo de arena | `A` |

## Limitações conhecidas

- A luta padrão ainda abre Rust x Java/Duke; Go entra na luta normal somente quando escolhido por CLI e ainda usa o spritesheet greybox genérico com cor própria.
- Rust e Duke ainda compartilham parte do kit genérico; o contraste principal já aparece em jab, heavy, anti-air, sweep, overhead e throw.
- As arenas bitmap são placeholders gerados/derivados de referências e não devem ser tratadas como arte final.
- O spritesheet de lutador é placeholder gerado localmente com formas simples e não deve ser tratado como arte final.
- Fireball no gamepad usa `RB` por enquanto; `RT` pode entrar depois quando tivermos leitura de gatilho com borda de pressionamento.
- Defesa é um experimento mínimo: já separa high/low/mid/throw/projectile, mas ainda não tem direção esquerda/direita nem defesa perfeita por timing.
- A CPU é um sparring dummy determinístico: decide em pequenos blocos de tempo, usa perfis diferentes por slot, varia movimento/ataque/especial/defesa e reage a projéteis sem ser perfeita.
- Não há combo, especial avançado, throw tech, knockdown ou IA adaptativa.
- Não há arte final, animação final, áudio final, pausa dedicada ou IA avançada.
- O balanceamento ainda não importa.
- A colisão é propositalmente simples e axis-aligned.
- O visual é debug/greybox, não direção de arte final.

## Caminho sugerido

### Próximo passo técnico

- Refinar sensação de movimento.
- Validar o kit tradicional em playtest e ajustar frame data.
- Separar melhor estados de ataque/hitstun se o kit atual for aceito.
- Criar testes para vitória/restart e bordas da arena.
- Ajustar heurísticas da CPU depois de playtest manual.

### Próximo passo de gameplay

- Decidir se sweep, overhead, anti-air e throw serão universais ou parte da identidade de cada personagem.
- Decidir se defesa por direção entra cedo ou fica para depois do feeling básico.
- Definir o primeiro diferencial mecânico mínimo de Rust e Java.

### Próximo passo de arte

- Usar este greybox para testar silhueta, proporção de braços/pernas e leitura de ataque sem overlays.
- Propor placeholders melhores sem perder legibilidade das caixas.
- Começar mood candidato usando `docs/templates/mood-proposal.md`.
