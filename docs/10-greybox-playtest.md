# 10 — Greybox Playtest

## Status

Esta versão jogável permite testar partidas locais, seleção de personagens, pausa e revanche. O playtest verifica clareza dos comandos, leitura de combate e continuidade entre as lutas. O corte atual está descrito em [conclusão visual do playtest](26-playtest-visual-completion.md).

## O que já existe

- Janela Raylib.
- Loop de jogo com fixed timestep.
- Arenas bitmap placeholder `Sirius`, `Fortaleza Tech Coast`, `Java Street`, `BioTIC`, `Porto Digital` e `Vale do Pinhao`.
- Menu principal com seleção visual de personagens, treino e opções runtime.
- Livro `Lore / Roster` carregado de JSON runtime, com capítulos de história e fichas dos personagens da demo.
- Seleção de P1/P2, modo e arena antes da luta; volume de música em `Options`.
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
- Primeiro corte de identidade mecânica: Rust com respostas mais rápidas/curtas; Duke com ferramentas mais longas/pesadas e mais puníveis; Go como rushdown mantido para CLI/ferramentas; C como fundamentos de alcance/risco; Python como punisher ágil de dano moderado.
- Fireball horizontal simples com ritmo diferente por personagem.
- CPU de playtest para um ou dois jogadores, com perfis diferentes e acoes variadas.
- Opção para IA mover/defender sem dar golpes.
- Opção para Player 1 não receber dano.
- Movimento com aceleração/desaceleração.
- Hurtbox visível quando debug de combate está ligado.
- Hitbox/alcance dos golpes visível quando debug de combate está ligado.
- Dano fixo.
- Barra de vida com número.
- Hit spark, block spark, dano flutuante, trail de projectile, luz de chão em stun e animações leves de fundo por arena.
- Condição de vitória.
- Pausa com Continuar, Reiniciar, Trocar personagens e Menu; resultado com Revanche, Trocar personagens e Menu.
- HUD, ajuda de controles e debug visual configuráveis.
- Testes de regras de combate sem abrir janela.
- Move Showcase com dois lutadores, situações específicas por golpe, resultado real de contato e quatro exemplos de defesa.
- Um especial de assinatura por personagem da demo, com animação própria e recuperação punível.
- Um especial cinematográfico adicional para os seis personagens: Rust, Duke/Java, C, C++ e Python usam sequências autorais de captura; Go preserva contato local. Nas lutas, cada jogador começa com 50/100 de energia e precisa de 100 para ativá-lo; Combat Lab e Move Showcase mantêm acesso livre.
- Knockdown de 36 frames para rasteira, agarrão e slide da Python contra alvo no chão.
- Combat Lab com reprodução de golpes e poses estáticas de inspeção.

## Como rodar

Para jogar a distribuição publicada, siga as instruções de download e instalação na
[página de releases](https://github.com/osdeving/borrow-fighters/releases). O pacote
inclui o jogo e os assets; quem joga não precisa instalar Rust ou compilar.

Para executar a partir do código-fonte, os requisitos são:

- Rust estável.
- Dependências nativas exigidas por Raylib/raylib-rs no seu sistema operacional.

Comando:

```bash
cargo run
cargo run -- --fight --p1 go --p2 duke
cargo run -- --fight --p1 c --p2 rust
cargo run -- --fight --p1 python --p2 duke
cargo run -- --fight --p1 cpp --p2 c
```

Use `--p1`/`--player-one` e `--p2`/`--player-two` para iniciar matchups específicos sem tela de seleção. Valores aceitos: `rust`, `rustacean`, `duke`, `java`, `go`, `golang`, `gopher`, `c`, `langc`, `c-lang`, `clang`, `python`, `py`, `python.py`, `cpp`, `c++`, `cplusplus`, `c-plus-plus`, `cxx` e `cpp.cpp`. Use `--fight` ou `--skip-menu` para abrir diretamente na luta. A seleção visual oferece Rust, Duke/Java, C, Python e C++, além de uma opção aleatória entre esses cinco; os espaços futuros não podem ser confirmados. Go/Gopher fica disponível por CLI e ferramentas enquanto permanece fora da seleção pública.

Checks úteis:

```bash
cargo fmt
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

O GitHub também roda `Rust Check` no PR para validar formatação, testes e clippy em Linux.

## Move Showcase

Abra pelo menu `Training > Move Showcase`. A cena usa o Player 1 da configuração de luta confirmada, a arena atual e um adversário do mesmo elenco. Também abre diretamente por CLI:

```bash
cargo run -- --showcase --character rust --move anti_air --repeat
cargo run -- --showcase --character duke --move throw --repeat
cargo run -- --showcase --character c --move sweep --reverse
cargo run -- --showcase --character python --move signature_special --repeat
cargo run -- --showcase --character cpp --move signature_special --repeat --reverse
cargo run -- --showcase --character rust --move cinematic_special --repeat
cargo run -- --showcase --character go --move cinematic_special --repeat
```

Cada personagem da demo tem 16 situações: 12 ataques, contando projétil, assinatura e cinematográfico, e quatro demonstrações de defesa. A aproximação é real: o oponente anda em direção ao jab, salta em direção ao anti-air e ao Template Arc, protege o tronco contra a rasteira ou agacha contra o overhead. O agarrão pega um oponente próximo em guarda. Os ataques aéreos acontecem durante um salto real do atacante. As quatro defesas mostram guarda em pé contra jab, guarda em pé contra overhead, guarda baixa contra rasteira e bloqueio de projétil.

Dano e resultado vêm das colisões do `World`, incluindo blockstun, hitstun e queda. Cada situação restaura vida e posição antes da próxima apresentação. Os cinematográficos ficam livres de custo de energia para permitir repetição e inspeção. `HIT`, `BLOCKED` e `WHIFF` descrevem o contato realmente observado; não há dano artificial para completar a demonstração.

| Atalho | Ação |
|---|---|
| `Tab` / `Shift+Tab` | Próxima / anterior situação |
| `Enter` / `Home` | Reiniciar a situação |
| `Espaço` | Pausar / continuar |
| `.` | Avançar um frame enquanto pausado |
| `L` | Alternar autoplay de todas as situações / repetição da atual |
| `X` | Espelhar os dois lutadores e reiniciar |
| `PageUp` / `PageDown` | Trocar personagem entre os cinco da demo |
| `Esc` | Voltar ao menu |

`--repeat` inicia repetindo a situação escolhida e `--reverse` inicia com o personagem à direita. Trocar personagem preserva o golpe selecionado, a pausa, a repetição e o lado. Para acompanhar a revisão mecânica, consulte a [matriz de golpes](15-character-combat-matrix.md) e a [auditoria determinística](evidence/mvp-balance/README.md).

## Combat Lab

Para abrir uma cena limpa de inspeção de golpe:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move sweep
cargo run -- --lab combat --character duke --move anti-air
cargo run -- --lab combat --character go --move kick
cargo run -- --lab combat --character c --move projectile
cargo run -- --lab combat --character python --move heavy_punch
cargo run -- --lab combat --character cpp --move overhead
cargo run -- --lab combat --character python --move signature_special
```

Para abrir uma pose estática:

```bash
cargo run -- --lab combat --character rust --pose crouch
cargo run -- --lab combat --character duke --pose victory
cargo run -- --lab combat --character go --pose jump
cargo run -- --lab combat --character c --pose idle
cargo run -- --lab combat --character python --pose hit
```

No Combat Lab, `Tab` / `Shift+Tab` alterna golpes, `PageDown` / `PageUp` alterna poses, `Enter` reinicia, `Espaço` pausa, `.` avança um frame quando pausado, `Home` volta ao frame 0, `H` alterna hurtbox, `B` alterna hitbox, `P` alterna pivot/eixos, `D` alterna dummy, `A` alterna o fundo de arena e `Esc` volta ao menu quando o lab foi aberto por `Training`.

O Combat Lab abre com o fundo `Sirius` ligado para validar contraste de golpe/sprite contra cenário. Use `A` para remover o fundo e voltar ao grid limpo. O laboratório permite ativar os cinematográficos sem acumular energia.

## Menu Principal

Na primeira abertura, o guia **Como jogar** mostra controles de teclado e gamepad,
defesa, condição de vitória e três modos: **Jogar contra CPU** (você é P1),
**Duelo local** (P1 e P2 manuais) e **Assistir demo** (CPU contra CPU). Escolher um
modo abre a seleção de personagens com essa configuração. Confirme P1, confirme
P2 e então escolha **Lutar** para começar. **Ir ao menu principal** abre as outras
opções antes de jogar. O jogo volta ao menu principal nas próximas aberturas, sempre
começando com P1 manual contra P2 CPU. A escolha do modo vale para a sessão.

O guia pode ser reaberto em **Como jogar** no menu. Sair dele por uma opção ou
`Esc` grava `onboarding-v1.seen` no diretório de dados do usuário: `%LOCALAPPDATA%/BorrowFighters`
no Windows e `$XDG_DATA_HOME/borrow-fighters` (ou `~/.local/share/borrow-fighters`)
no Linux. Se a gravação falhar, o jogo continua e o guia reaparece na próxima
abertura. `--fight`, `--showcase`, `--lab` e `--tool` preservam a entrada direta.

Nos menus, use `Setas` ou `W/S` para navegar e `Enter`, `Espaço` ou o botão `A` do controle para confirmar. O mouse seleciona ao passar sobre uma linha e confirma com clique esquerdo. Em Lore/Options, `A/D`, `←/→` e clique esquerdo/direito ajustam capítulo, ficha e volume. `Esc` volta de submenus e ferramentas; durante a luta, abre **Pausa**. O cursor permanece livre para sair da janela; em WSL, o cursor `Linker` acompanha o ponteiro apenas com a janela em foco. Para sair do jogo, use a opção de sair no menu ou o botão nativo de fechar.

Fluxo atual:

- `Quick Fight` e `Versus Setup` abrem a **seleção de personagens**.
- `Training` abre `Move Showcase`, `Combat Lab` ou `Sprite Viewer`.
- `Lore / Roster` abre o livro do Linker e fichas de Rust, Duke/Java, C, Python e C++.
- `Options` ajusta volume da música, gravação local, CPU, dano, HUD, ajuda, debug e gamepad.
- `Como jogar` reabre este guia e permite escolher o modo antes da seleção.

Na seleção contra CPU ou na demo, use `WASD`, setas ou `D-pad` para mover o cursor; `Enter`/`F`/`Espaço`, o botão `A` ou clique confirma o personagem do lado ativo. No duelo local pelo teclado, P1 usa `WASD` e confirma com `F`; P2 usa setas e confirma com `Enter`. `Espaço` continua confirmando o lado ativo. Confirme **P1**, depois **P2** e, com os dois prontos, confirme **Lutar** (`Enter`/`Start` ou clique). Dois controles podem confirmar seus lados separadamente. Clique na prévia de P1/P2 para escolher o lado ativo. `Tab` ou clique em **Modo** alterna **Você × CPU**, **Duelo local** e **CPU × CPU**; mudar modo limpa as confirmações. `Q/E` ou as setas da arena mudam o cenário. No controle de P1, `Select`/`Back` alterna modo e `LB/RB` alterna arena; P2 mantém seu cursor independente. `Esc`/`B` desfaz uma confirmação antes de voltar ao menu. A opção aleatória escolhe apenas lutadores disponíveis; espaços futuros ficam bloqueados.

Ao começar uma luta, os personagens entram em cena e aparece a contagem `11`, `10`, `01`, `Fight!`. Intro e contagem bloqueiam movimento e ataques. Zerar a vida rival decide o resultado depois de terminar qualquer sequência cinematográfica ativa.

Durante a luta, `Esc` ou `Menu`/`Start` abre **Pausa**, com **Continuar**, **Reiniciar**, **Trocar personagens** e **Menu**. A simulação e o áudio da luta param; continuar retoma do mesmo ponto. `R` é o reinício rápido fora dos overlays. Depois da pose final, **Revanche**, **Trocar personagens** e **Menu** aparecem abaixo dos lutadores. Use setas/direcional ou mouse para escolher e `Enter`/`A` para confirmar. No resultado, `Start` também confirma; na pausa, `Esc`/`Start`/`B` continua. O cenário permanece visível até iniciar a próxima luta.

| Preferência | Padrão | O que testar |
|---|---|---|
| Personagem Player 1 | rust.rs | A próxima luta deve iniciar com o personagem escolhido para o Player 1. |
| Personagem Player 2 | duke.java | A próxima luta deve iniciar com o personagem escolhido para o Player 2. |
| Arena | Sirius Light Ring / Campinas, SP | A próxima luta deve iniciar no cenário escolhido. |
| Lore / Roster | Manual do Linker | Capítulos e fichas devem carregar de `assets/lore/story.json` sem recompilar. |
| Volume da música | 50% | A música deve baixar/subir em passos de 10% sem afetar SFX e vozes. |
| Player 1 usa IA | Desligado | O Player 1 responde ao teclado/gamepad; `Assistir demo` liga a IA. |
| Player 2 usa IA | Ligado | O Player 2 deve ser controlado automaticamente. |
| IA pode dar golpes | Ligado | Quando desligado, lutadores controlados por IA devem andar, pular, afastar, aproximar e defender, mas não atacar. |
| Player 1 recebe dano | Ligado | Quando desligado, Player 1 mantém HP, mas reage aos golpes, defende e pode ser arremessado. |
| Player 2 recebe dano | Ligado | Quando desligado, Player 2 mantém HP, mas reage aos golpes, defende e pode ser arremessado. |
| Mostrar HUD | Ligado | Barras de vida e título no topo aparecem/desaparecem. |
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
| Fireball / projétil | `G` | `Right Ctrl` ou `KP0` | `RB` |
| Especial de assinatura | `T` | `\` (Backslash) | `RT` |
| Especial cinematográfico | `Y` | `]` (Right bracket) | Segurar `LB` + pressionar `RT` |
| Alternar P2 CPU/manual | `C` | `C` | `View` |
| Pausar / continuar | `Esc` | `Esc` | `Menu` / `Start` |
| Reiniciar rapidamente, fora dos overlays | `R` | `R` | Pausa > Reiniciar |

O primeiro gamepad conectado controla o Player 1 quando a IA do Player 1 está desligada. O segundo gamepad controla o Player 2 quando a IA do Player 2 está desligada. O aplicativo começa com P1 manual e P2 CPU. **Como jogar > Duelo local** desliga as duas IAs; **Assistir demo** liga ambas. Quando a CPU de um jogador está ligada, os comandos manuais daquele jogador são ignorados.

Quando ambos os jogadores usam IA, Rust e Java usam perfis diferentes para evitar movimentos espelhados. Rust tende a preservar mais média distância e usar especial com mais frequência; Java tende a pressionar mais de perto. A IA decide em pequenos blocos de tempo e pode andar, afastar, pular, abaixar, bloquear, socar, chutar, tentar varredura, overhead, anti-air, agarrão curto, ataque aéreo e soltar especial.

Com `Mostrar debug de combate` ligado, o topo da tela mostra `Pad P1` e `P2` como `ON` quando Raylib detecta o controle. Se um controle Bluetooth estiver pareado mas aparecer `OFF`, confirme se o sistema que executa `cargo run` expõe joystick/gamepad para o Raylib. Em WSL ou ambiente remoto, pode ser necessário testar no host nativo ou encaminhar o dispositivo.

## Defesa e recuperação

Segure `Q` para Player 1 ou `U` para Player 2; no gamepad, `LB` ou `LT`. A guarda é uma ação explícita e não exige segurar a direção contrária.

| Ataque recebido | Resposta |
|---|---|
| Socos/chutes médios e projétil | Guarda em pé ou abaixada |
| Rasteira e Serpent Slide | Guarda abaixada ou salto com antecedência |
| Overhead, GC Slam e ataques aéreos | Guarda em pé |
| Agarrão | Saltar, sair de alcance ou interromper o startup |

Durante blockstun, o lutador mantém a altura da guarda que bloqueou o golpe. Quando um overhead quebra guarda baixa, começa hitstun e termina o blockstun anterior. Chip deixa pelo menos 1 de vida, preservando a chance de responder.

Rasteiras, agarrões e o slide da Python causam queda quando atingem um alvo no chão. A recuperação de 36 frames impede novos hits enquanto caído. Agarrões não pegam alvos no ar, em hitstun/blockstun nem nos seis primeiros frames após recuperar. Não há throw tech ou juggle de oponente caído neste corte.

Especiais de assinatura são interrompíveis e ficam expostos após bloqueio ou erro. Teste um jab imediato após bloquear de perto. O projétil conserva seu botão; sua conjuração agora impede outro ataque, pulo ou guarda até terminar o tempo da pose. Um golpe sofrido interrompe essa pose.

Os cinematográficos são Ownership Eclipse (Rust), Garbage Collector (Java), Million Goroutines (Go), General Protection Fault / #GP (C), import devour (Python) e Undefined Behavior: Footgun (C++). `Right Shift` continua sendo soco forte de P2.

Rust, Java, C, C++ e Python iniciam uma sequência de 5–10,53 segundos quando o atacante está livre no chão e tem 100 de energia na luta. A captura alcança qualquer distância; bloquear em pé ou abaixado **no instante da ativação** reduz o dano e impede KO por chip. Afastar-se depois da captura não cancela o golpe. Há uma pausa inicial de oito ticks, seguida das fases próprias de cada personagem: troca persistente para Sirius em blocos, chuva de lixo/coleta/queda gigante, terminais/tela azul/BIOS/reboot, notebook/tiro no pé/corrida/rajada/reboot de C++ e transformação de Python em cobra gigante, deglutição, reversão e celebração. C++ sofre apenas a reação cômica, sem perder a própria vida, e corre fisicamente até o alvo antes de acertar. Comandos simultâneos elegíveis dos dois jogadores anulam as duas solicitações, permitindo tentar novamente; nenhum lado tem prioridade fixa.

Cada luta começa com 50/100 de energia por jogador; a ativação aceita consome 100. Golpes comuns que acertam rendem +12 para quem atacou e +8 para quem recebeu; no bloqueio, +4 para quem atacou e +6 para quem defendeu. Errar ou aguardar não enche a barra, e os contatos do próprio cinematográfico não recuperam energia. Com energia insuficiente ou comando recusado, não há cobrança. A energia volta a 50 ao reiniciar. Combat Lab e Move Showcase mantêm o acesso livre.

Movimentos e projéteis comuns aguardam a sequência terminar; somente os contatos programados causam dano. Mesmo ao zerar a vida, o resultado do round espera a restauração visual. Go conserva o golpe local interrompível: alcance e antecipação continuam relevantes. O Combat Lab reproduz os cinco supers com um `World` completo, e o Showcase oferece pausa, avanço por frame, repetição e troca de lado, respeitando a duração própria. O alvo engolido por Python volta durante a reversão; há somente um contato de dano. A arena criada por Rust continua em Sirius depois do golpe, e reiniciar/reproduzir a luta restaura a arena base. Ver [roteiros e contrato](24-reactions-and-transformations.md) e [timings técnicos](12-technical-combat-guide.md#especiais-cinematográficos-adicionais).

## Como ler a tela

| Elemento | Significado |
|---|---|
| Barra de energia 0–100 | Começa em 50; cheia libera um cinematográfico de custo 100 |
| Partes azuis | Rust / Player 1 |
| Partes laranja | Java / Player 2 |
| Braços e pernas do sprite | Pose/ação atual sem depender do debug |
| Outline branco | Corpo físico do personagem, só com debug ligado |
| Caixas verdes | Hurtboxes de cabeça, tronco e pernas, só com debug ligado |
| Caixa vermelha | Alcance do golpe corpo-a-corpo, só com debug ligado |
| Caixa/círculo ciano com rastro | Fireball |
| Corpo amarelo | Ataque em fase ativa |
| Linhas/círculos amarelos no contato | Golpe acertou |
| Anel/escudo azul no contato | Defesa reduziu dano |
| Luz de chão vermelha/azul | Lutador em hitstun ou blockstun |
| Blips/linhas/chuva no fundo | Animação leve de cenário, sem significado de combate |
| `-8`, `-12`, `-16` | Dano aplicado |
| Linha magenta | Colisão corpo-corpo bloqueando passagem, só com debug ligado |
| Fundo Sirius/Fortaleza/Java Street/BioTIC/Porto Digital/Vale do Pinhao | Arena placeholder, não arte final |

Hitboxes, hurtboxes, retângulos de reação/guarda, limites da arena, labels de golpe e linha de colisão aparecem somente com `Mostrar debug de combate` ligado. A ajuda de comandos no rodapé aparece somente com `Mostrar ajuda de controles` ligado.

## O que testar agora

1. Um jogador não deve atravessar o outro.
2. A arena bitmap deve ajudar o mood sem esconder lutadores, hitboxes, hurtboxes ou HUD.
3. Soco fraco deve ser mais curto e mais rápido.
4. Soco forte deve alcançar mais longe e causar mais dano.
5. Chute deve acertar em uma altura mais baixa.
6. Varredura baixa deve manter postura abaixada, atingir canela/pé e exigir defesa abaixada; um salto antecipado deve escapar.
7. Overhead deve exigir defesa em pé.
8. Agarrão curto deve ignorar defesa e provocar queda, mas errar fora de alcance, contra oponente no ar ou ainda protegido pela recuperação.
9. Anti-air deve cobrir região acima/frente do personagem.
10. Ataques aéreos devem funcionar durante salto sem travar a queda.
11. Defesa deve reduzir dano e mostrar feedback azul.
12. Abaixar deve reduzir a hurtbox visualmente.
13. Fireball deve andar horizontalmente em velocidade legível e causar dano ao acertar.
14. A CPU do Player 2 deve variar aproximação, afastamento, pulo, socos, chutes, varredura, overhead, anti-air, agarrão curto, ataque aéreo, defesa e fireballs.
15. Rust deve parecer mais responsivo em anti-air e throw.
16. Duke deve controlar mais espaço com sweep, overhead e poke, mas ficar mais exposto quando erra.
17. Go no Combat Lab ou na luta iniciada por `--p1 go`/`--p2 go` deve parecer mais rápido e curto que os golpes genéricos equivalentes, pagando com menos vida; ele não deve aparecer na seleção pública nem na escolha aleatória.
18. C no Combat Lab, no menu ou na luta iniciada por `--p1 c`/`--p2 c` deve aparecer na escala correta, com entrada, atlas de luta e projectile carregados, e jogar como fundamentos de alcance maior com whiff mais punível.
19. O projectile do C deve ler claramente como stream de bits, com `0` e `1` visiveis durante a luta.
20. Python no Combat Lab, no menu ou na luta iniciada por `--p1 python`/`--p2 python` deve aparecer com atlas de luta, entrada cinematografica e projectile carregados; o soco fraco deve ler como bote da cobra, o soco forte como ataque da própria personagem e a personagem deve compensar dano menor com startup/recovery mais leves.
21. Rust, Duke, Go, C, Python e C++ devem ter projectiles com ritmo diferente: Rust médio, Duke pesado/lento, Go rápido/curto, C médio/rápido, Python rápido/médio, C++ rápido/médio.
22. Rust, Duke, Go, C, Python e C++ devem emitir voz/esforço no início dos golpes próximos e projectile cast.
23. Rust e Duke/Java devem ter esforço audível em cada golpe próximo do loadout, sem depender só de fallback curto.
24. `Lore / Roster` deve mostrar o livro do Linker, trocar capítulo/personagem com A/D e exibir retrato/ficha de Rust, Duke/Java, C, Python e C++.
25. Editar `assets/lore/story.json` e reiniciar o jogo deve alterar o texto do livro sem recompilar.
26. Na seleção visual, `Q/E`, `LB/RB` no controle de P1 ou as setas clicáveis da arena devem atualizar a prévia; a luta confirmada deve começar na arena escolhida. Com dois controles, `Select`/`Back` de P1 deve permitir preparar Duelo Local sem teclado; `B` continua cancelando e `Start` inicia apenas com os dois personagens confirmados.
27. `Options > Music Volume` deve baixar/subir a música em passos de 10%, sem afetar vozes e impactos.
28. O submenu `Options` deve ligar/desligar HUD, ajuda e debug sem reiniciar o jogo.
29. A opção `Player 1 usa IA` ligada deve permitir CPU x CPU quando `Player 2 usa IA` tambem estiver ligada.
30. A opção `IA pode dar golpes` desligada deve impedir soco, chute e fireball da CPU, mantendo movimento/defesa.
31. A opção `Player 1 recebe dano` desligada deve impedir perda de vida do Player 1, mantendo animação de impacto/guarda, empurrão, queda e recuperação.
32. A opção `Player 2 recebe dano` desligada deve fazer o mesmo para Player 2. Conferir soco, chute, projétil, arremesso, assinatura e cada pancada da rajada de C++; `HIT -0` não deve deixar o alvo em idle.
33. Gamepad Xbox deve controlar o Player 1 com left stick/D-pad, `A`, `X`, `Y`, `B`, `LB/LT` e `RB` quando o ambiente expõe controle ao Raylib.
34. `C` ou `View` deve alternar entre CPU e controle manual do Player 2.
35. `R` deve reiniciar rapidamente fora de pausa/resultado; no controle, `Menu`/`Start` abre a pausa e `Reiniciar` refaz a luta com vida e energia iniciais.
36. `Esc` ou `Menu`/`Start` durante a luta deve abrir a pausa, congelando movimento, projéteis, intro, contagem e cinematográficos. Continuar deve retomar do mesmo ponto; o botão de abertura não pode confirmar uma ação no mesmo instante.
37. `Training > Combat Lab` deve abrir o laboratório e `Esc` deve voltar ao menu.
38. `Training > Move Showcase` deve mostrar os dois atores nas situações de golpe/defesa; `Esc` deve voltar ao menu e repetir um cinematográfico não deve exigir energia.
39. `Training > Sprite Viewer` deve abrir o viewer e `Esc` deve voltar ao menu.
40. Pulo com direção pressionada deve sair em diagonal.
41. A vida deve chegar a zero e encerrar a luta.
42. A tela de resultado deve manter as poses visíveis e oferecer `Revanche`, `Trocar personagens` e `Menu`. Revanche reinicia o confronto; a rotação automática de arena, quando ativa, avança somente no começo da próxima luta, enquanto uma arena escolhida na seleção deve ser respeitada.
43. O feedback visual deve deixar claro quando houve contato físico, golpe, bloqueio e projétil por hitspark, block pulse, trail e luz de chão em stun.
44. O mouse deve alcançar qualquer ponto da janela e sair dela sem retornar ao centro, tanto no menu quanto na luta. Em WSL, `Linker` acompanha o movimento e desaparece fora da janela ou ao perder foco.
45. Hover e clique devem navegar por todos os menus, incluindo `Back` e `Exit`; cliques fora das linhas não devem ativar a seleção anterior. O mouse parado não deve impedir navegação por teclado/gamepad.
46. No roster, clique deve escolher e confirmar o personagem do lado ativo; clicar na prévia P1/P2 deve selecionar o lado. Arena e modo têm áreas próprias. Nos menus de Lore/Options, clique esquerdo/direito continua avançando/voltando capítulo e volume; flags alternam com clique esquerdo.
47. O botão nativo de fechar deve encerrar a janela durante a luta e nas ferramentas. Durante a luta, `Esc > Menu` e depois a opção de sair deve encerrar pelo menu.
48. O tutorial, a opção de jogar e `Versus` devem abrir a seleção. Confirmar P1 não pode confirmar P2 nem iniciar a luta no mesmo evento; depois das duas confirmações, um novo comando em `Lutar` inicia o confronto.
49. `Tab` na seleção deve alternar CPU, duelo local e demonstração, limpando confirmações antigas. O aleatório deve escolher apenas um dos cinco lutadores públicos; slots futuros não iniciam luta.
50. Energia deve começar em 50, subir somente em contatos normais e parar em 100. Cinematográfico recusado por falta de energia não dispara áudio/animação nem gasta energia; com 100, um uso aceito consome a barra uma única vez.
51. Pausar e continuar no meio de um cinematográfico deve preservar sua fase e o áudio. Reiniciar, trocar personagens ou ir ao menu deve encerrar a sequência antiga.
52. Combat Lab e Showcase devem reproduzir cinematográficos livremente com pausa, avanço de frame e repetição.

## Combat Lab

Para testar um golpe sem iniciar a luta completa:

```bash
cargo run -- --lab combat --character rust --move light_punch
cargo run -- --lab combat --character duke --move projectile
cargo run -- --lab combat --character rust --move overhead
cargo run -- --lab combat --character duke --move throw
cargo run -- --lab combat --character go --move light_punch
cargo run -- --lab combat --character c --move projectile
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
| Voltar ao menu quando aberto por `Training` | `Esc` |

## Limitações conhecidas

- A luta padrão ainda abre rust.rs x duke.java; old.c, python.py e cpp.cpp entram pela seleção da demo, e gopher.go entra por CLI, Combat Lab ou Sprite Viewer enquanto fica fora do menu público.
- Rust e Duke ainda compartilham alguns golpes universais, mas já têm ferramentas próprias para definir ritmo.
- C, Python e C++ ainda usam ataques aéreos universais, mas já possuem kit terrestre, throw, projectile, vida e arquétipo próprios.
- As arenas bitmap são placeholders gerados/derivados de referências e não devem ser tratadas como arte final.
- O spritesheet de lutador é placeholder gerado localmente com formas simples e não deve ser tratado como arte final.
- Fireball usa `RB`; especial de assinatura usa a borda do botão `RT` exposta pelo mapeamento Raylib. `LB` segurado + borda de `RT` seleciona exclusivamente o cinematográfico; a guarda não impede seu início.
- Defesa é um experimento mínimo: já separa high/low/mid/throw/projectile, mas ainda não tem direção esquerda/direita nem defesa perfeita por timing.
- A CPU é um sparring dummy determinístico: decide em pequenos blocos de tempo, usa perfis diferentes por slot, varia movimento/ataque/especial/defesa e reage a projéteis sem ser perfeita.
- Não há combo tree, throw tech, juggle no chão ou IA adaptativa. A energia limita apenas cinematográficos; projéteis e especiais de assinatura preservam seus tempos e não gastam essa barra.
- Arte, animação e áudio continuam sujeitos a revisão de playtest; a pausa está implementada, mas não há IA avançada.
- O balanceamento de MVP é verificado por contrajogo e testes repetíveis; refinamento competitivo depende de playtest humano.
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
- Playtestar Borrow Break e GC Slam contra os outros três especiais, observando interrupção, alcance e recuperação.

### Próximo passo de arte

- Usar este greybox para testar silhueta, proporção de braços/pernas e leitura de ataque sem overlays.
- Propor placeholders melhores sem perder legibilidade das caixas.
- Começar mood candidato usando `docs/templates/mood-proposal.md`.
