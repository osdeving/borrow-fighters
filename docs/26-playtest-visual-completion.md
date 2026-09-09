# 26 — Seleção, fluidez e conclusão visual do playtest

## Pedido e prioridade

Rodada autorizada em 9 de setembro de 2026, a partir de `v0.1.0-prototype.2`.
Entrega integrada à `main` pelo
[PR #19](https://github.com/osdeving/borrow-fighters/pull/19), preparada para
distribuição em [v0.1.0-prototype.3](releases/v0.1.0-prototype.3.md).
A prioridade é emparelhar as reações do elenco com o piloto Python/C++.
O pedido também inclui seleção visual de personagens, pausa/revanche, energia,
transições e substituição da voz de Old C. Balanceamento fino fica para depois.

## Entregas

1. **Reações completas:** Rust, Java, Old C e Go recebem os oito perfis de
   contato do [padrão 25](25-python-cpp-contact-reactions.md), com quatro
   desenhos articulados por clip (128 desenhos novos como alvo). Python/C++
   permanecem como referência. Conferir guarda, contatos repetidos, arremesso,
   queda, KO e dano desligado, nos dois sentidos e com o elenco completo.
2. **Seleção Linker:** retratos em grade, personagens completos animados dos
   dois lados, nome/identidade, cursores e confirmação P1/P2, seleção aleatória,
   vagas futuras identificadas e não selecionáveis, arena e modo de controle.
   A organização remete a jogos de luta tradicionais; desenho, termos e efeitos
   usam a identidade de programação do projeto. O elenco público atual é
   preservado; vagas futuras não fingem conteúdo disponível.
3. **Fluxo de partida:** pausa explícita por Esc/Start, continuar, reiniciar,
   trocar personagens e menu. Vitória oferece revanche e troca de personagens.
   Revanche preserva o confronto e a arena. Entradas usadas para confirmar uma
   tela não podem virar ataque, salto ou nova confirmação na tela seguinte.
4. **Energia:** medidor 0–100 por jogador, carga inicial 50 e cinematográfico
   com custo 100. Carga por contatos normais de ataque/defesa; nenhum ganho por
   ataques no vazio ou pelo próprio cinematográfico. Pedido recusado ou
   simultâneos cancelados não gastam energia. HUD explica carga/disponibilidade.
   Estes valores são um primeiro corte funcional, não balanceamento aprovado.
   Combat Lab e Move Showcase continuam com uso livre para revisão;
   Assistir demo (CPU × CPU) usa a mesma energia das lutas normais.
5. **Apresentação em movimento:** entrada das telas, movimento de foco,
   confirmação, revelação do confronto, início e resultado de luta. Uma pequena
   linguagem compartilhada de painéis, pulsos e conexões anima a UI sem framework
   genérico. Os efeitos preservam leitura de personagens, HUD e comandos.
6. **Voz de Old C:** procurar alternativa redistribuível e consistente; se não
   houver opção com qualidade verificável, compartilhar temporariamente a voz
   de Rust, como autorizado. Preservar os outros personagens e registrar fontes.

## Critérios de aceite

- Os seis personagens respondem corporalmente a contatos reais e sucessivos;
  nenhum clip novo altera sozinho corpo físico, hitboxes, dano ou frame data.
- Seleção funciona por teclado, mouse e gamepad; random só resolve personagens
  jogáveis, confirmação não troca aleatoriamente a escolha em cada frame, vagas
  futuras não iniciam partidas e cancelar permite corrigir a escolha.
- Previews mostram o personagem selecionado com animação, escala estável e
  identidade igual à usada na luta; P1/P2 continuam distinguíveis no mesmo slot.
- Pausa congela combate, cinematográficos e a posição do áudio correspondente;
  continuar não causa avanço acumulado, ataque residual nem reinício.
- Três partidas seguidas podem ser iniciadas, pausadas, retomadas e repetidas
  sem voltar a um menu técnico ou reiniciar por engano.
- Energia respeita teto/piso, custo único, reinício e treino livre; a barra
  corresponde ao estado do World e não é apenas decorativa.
- Revisão do renderer real cobre seleção, confirmação/random, pausa, resultado,
  começo da luta, energia e contatos dos personagens, com capturas e vídeo.
- Assets e fontes necessários continuam incluídos nos pacotes; o carregamento
  em caminhos Unicode Windows continua coberto pela regressão existente.
- Rodar fmt, testes e Clippy; validar links/YAML e registrar o que foi ou não
  verificado com hardware e áudio reais. Nada é marcado como concluído apenas
  por existir no disco ou passar em um teste de seleção de frames.

## Organização

Estado, navegação e energia ficam testáveis sem Raylib. Desenho e input de
plataforma ficam na borda `engine`; `App` conecta cenas, áudio e World.
A [ADR 0020](adr/0020-match-flow-selection-and-energy.md) registra os limites.
O [backlog](03-backlog.md) continua sendo a fonte de verdade da frente ativa.

## Estado da entrega

Implementação e revisão funcional concluídas em 9 de setembro de 2026.

| Entrega | Verificação |
|---|---|
| Reações do elenco | 128 desenhos novos, 120 cenários renderizados, 2.920 PNGs auditados, 128 janelas de rajada e oito KOs; [vídeo, amostras e relatórios](evidence/roster-contact-reactions/README.md). Python/C++ permanecem cobertos pelo piloto e pela matriz geral. |
| Seleção Linker | Retratos e previews dos cinco selecionáveis, escala/pivô estáveis, random, vagas futuras e confirmação dos dois lados; [capturas e vídeo](evidence/playtest-visual-flow/README.md). Go conserva suporte interno e as novas reações. |
| Pausa e revanche | Aplicação nativa sob Xvfb: 31 capturas revisadas, três partidas encerradas, duas revanches preservando confronto/arena, pausa/retomada e reinício com vida/energia restauradas; [revisão do fluxo](evidence/playtest-visual-flow/app-flow-ci/README.md). |
| Energia | Testes de custo, teto/piso, ganho por contato, cancelamento simultâneo, reinício e ferramentas livres; renderer mostra 100 após contatos e zero após cinematográfico aceito. |
| Movimento e efeitos | Abertura progressiva, foco, confirmação, previews animados, início/resultado e componentes de apresentação compartilhados; capturas e vídeo vêm do renderer real. |
| Voz Old C | Fallback de Rust autorizado, seis cópias exatas e nova camada vocal na entrada do super; [comparação, licença e preservação dos demais arquivos](../assets/audio/review/old-c-fallback-2026-09-09/README.md). |
| Código e distribuição | 393 testes, fmt e Clippy estrito aprovados. O [CI completo](https://github.com/osdeving/borrow-fighters/actions/runs/34394289582) passou no Windows e Linux, incluindo ZIP/instalador em caminhos com acentos, DEB/tar e instalação RPM em Fedora 44. |

O código da revisão completa é `4f1631f`. O ajuste posterior `591b694` apenas
alinha as duas metades clicáveis da arena ao painel compartilhado; testes,
Clippy e fmt passaram novamente. Os
[pacotes da revisão atualizada](https://github.com/osdeving/borrow-fighters/actions/runs/34395185385)
e o PR identificam o commit de origem de cada build. A publicação de versão
segue o [processo de release](06-release-process.md).

A navegação foi testada com teclado sintético enviado à própria janela. Não
houve gamepad/mouse físicos nem aprovação humana de timbre. O teste separado
com PulseAudio verificou pausa/retomada reais usando sondas silenciosas; o CI
de navegação estava sem dispositivo de áudio. A tentativa no desktop WSLg
compartilhado ficou inconclusiva e foi substituída pelo teste isolado aprovado.

A revisão dos contatos registra limites pontuais de acabamento: sobreposição do
HUD no ápice de arremessos, mãos altas de Go em guarda em pé, primeiro recuo de
Duke afastado da faísca e variação facial de Go. Os relógios, a articulação e a
matriz passaram; a comparação de frames não equivale à aprovação humana de
fluidez. Balanceamento fino e esse feedback de playtest formam a próxima rodada.

O workflow manual `Playtest Release` captura a seleção, random, confirmação,
energia, pausa e resultado com Raylib/Xvfb no Linux. Os PNGs ficam no artefato
`visual-review-linux` para revisão humana; não são publicados como pacotes do jogo.
O exemplo `capture_roster_flow_review` também aceita `--frames` para inspecionar
a transição e animação quadro a quadro. Captura de pixels não substitui testar
a navegação e o áudio na janela real.
O artefato `native-review-tools-linux` contém os dois exemplos e os executáveis
de testes para revisão local de GPU/áudio usando a mesma compilação do CI.

Nas execuções manuais, `contact-reaction-review-linux` guarda o vídeo contínuo
dos contatos no sentido principal, os relatórios das quatro condições (lado e
dano), a auditoria completa e um PNG sem retoque por cenário/clip. Todos os PNGs
da matriz são verificados no runner antes dessa seleção para download.
`native-app-flow-linux` registra entradas de teclado enviadas à própria janela
X11 do jogo, incluindo pausa, seleção e três partidas/revanches. As expectativas
do script precisam ser confirmadas nas capturas; não equivalem a teste de
gamepad físico nem a audição humana. O vídeo de seleção fica junto aos PNGs
em `visual-review-linux`.

O parâmetro manual `deep_review` fica ligado por padrão. Pode ser desligado em
ajustes apenas da interface depois que contatos e navegação já tiverem evidência
válida: fmt, testes, Clippy, pacotes e capturas/vídeo da seleção continuam sendo
executados. Os exemplos são compilados com otimização para gravar o renderer
sem o custo da instrumentação de desenvolvimento.
