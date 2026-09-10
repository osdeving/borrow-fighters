# 27 — Aventura de Rust: prólogo e primeiro encontro

## Estado e autorização

Experimento de implementação autorizado em 10 de setembro de 2026, na branch
`feature/rust-adventure-prologue`. O pedido substitui o recorte anterior de
Vínculo/Sirius por um episódio curto: Ada trabalhando, mensagem misteriosa,
despertar de Assembly, salto temporal, manhã de Rust, ataque de uma errática
e vitória com pesar.

A [ADR 0021](adr/0021-isolated-adventure-experiment.md) registra o isolamento
entre aventura e luta. O [diário de implementação](worklogs/rust-adventure-prologue.md)
registra o que foi realizado e verificado. O episódio está implementado e
verificado na branch, com [vídeo e evidências](evidence/adventure-prologue/README.md).
Este documento preserva o escopo e os critérios da entrega. O [backlog](03-backlog.md)
continua sendo a fonte de verdade das frentes ativas. A versão distribuída de
luta permanece o [Prototype 3](releases/v0.1.0-prototype.3.md).

## Entrega jogável

```sh
cargo run --no-default-features --features adventure --bin borrow-adventure
```

Ada recebe a mensagem, Assembly desperta e o mistério fica em aberto. Muito
tempo depois, Rust acorda, sai para a rua e enfrenta uma errática sob controle
do jogador. Vitória leva ao olhar abaixado e balanço de cabeça, com a frase
“Não precisava ser assim.” Derrota permite tentar novamente no encontro.

As cenas combinam quadros ilustrados, poses, câmera, transições e efeitos;
o primeiro corte não é animação corporal contínua em todos os quadros. Há
movimento, salto, ataques leve/forte, defesa, pausa, replay e áudio próprio.
Arte e regras de ação pertencem à aventura. `cargo run` mantém a luta como
padrão. A [evidência](evidence/adventure-prologue/README.md) registra 412 testes
com ambos os modos, execução pela janela e limites do playtest automatizado.

## Intenção do experimento

Uma aventura de ação lateral, solo, protagonizada por Rust, com animações 2D
expressivas e mistério. O primeiro episódio deve fazer o jogador se importar
com ele através de algo pequeno e concreto: uma manhã comum interrompida por
uma ameaça, uma luta que o jogador precisa vencer e uma reação que revela
quem Rust é.

O combate serve ao acontecimento. A cena final não é o resultado de uma partida
com placar, seleção de adversário e revanche; é a consequência do encontro.
A relação entre movimento, ameaça e resposta deve funcionar durante o controle
manual, e a passagem entre narrativa e gameplay precisa ser clara.

A ambição de exploração por capítulos continua como horizonte. Este primeiro
experimento valida personagem, controle e encenação antes de ampliar o mundo.

## Lore aceita para esta rodada

O [worldbuilding](12-worldbuilding.md) e o [livro do jogo](../assets/lore/story.json)
foram sincronizados a partir das definições do usuário:

- **EPs são pessoas criadas e místicas**, não exatamente humanas. Têm emoções,
  vontade própria e diversidade moral. Há humanos e EPs nobres, cruéis,
  cuidadosos, confusos ou ambíguos; origem não determina caráter.
- **Rust é a última e mais recente EP pura conhecida** no presente da aventura.
  É justo, nobre e capaz de mais humanidade que muitos humanos. Defende
  coexistência, liberdade, cuidado e responsabilidade. “Pura” descreve sua
  origem como EP, sem declarar superioridade moral sobre híbridos ou humanos.
- **Ada começa como humana comum.** É a primeira usuária humana do Linker e
  torna-se híbrida humana e EP ao se aprofundar nessa força.
- **Assembly é a primeira EP a despertar**, após Ada aprender sobre o Linker
  e receber uma mensagem misteriosa. A autoria da mensagem, sua intenção e a
  relação exata entre a transformação de Ada e esse despertar ficam em aberto.
- **Frontenzos não invocam erráticas deliberadamente.** O mau uso do Linker cria
  condições cósmicas para o surgimento involuntário dessas entidades. Isso não
  torna todo humano responsável por toda ameaça. O episódio não precisa
  atribuir a errática a um frontenzo específico.

O Linker é uma força antiga; o prólogo mostra seu primeiro acesso humano e o
primeiro despertar de uma EP, sem explicar a origem da própria força. Naruto
e Fullmetal Alchemist são analogias de tom emocional e místico, sem importar
personagens, acontecimentos ou cosmologias dessas obras.

## Sequência autorizada

| Trecho | O que acontece | Intenção e limite |
|---|---|---|
| Ada no trabalho | Ada aparece humana, envolvida em uma atividade cotidiana de estudo/trabalho | Torná-la reconhecível como pessoa antes do acontecimento extraordinário |
| A mensagem | Após aprender sobre o Linker, ela recebe uma mensagem misteriosa | Texto curto e reação de Ada; não revelar remetente, plano ou mecanismo cósmico |
| Assembly desperta | A primeira EP ganha presença diante do acontecimento | Animação com peso e estranheza; o despertar acontece sem explicação completa |
| Muito tempo depois | Texto digitado em terminal conduz ao salto temporal | Indicar passagem de tempo sem fixar uma cronologia nova desnecessária |
| A manhã de Rust | Rust acorda como em uma manhã comum | Respiração, despertar e atenção; apresentar sua pessoa antes do ataque |
| A ameaça | Uma errática tenta matar Rust e o controle manual sustenta o encontro | O jogador se move, lê ataques e reage; o resultado depende de jogar |
| A consequência | Rust vence e balança a cabeça com pesar pelo que precisou fazer | Um gesto curto de necessidade, sem celebração triunfal; fim do episódio |

O jogador pode revelar o texto completo, avançar e pular o prólogo. Derrota
reinicia o encontro, sem reapresentar obrigatoriamente toda a abertura. Uma
nova execução permite rever a sequência. O gesto final deve ocorrer depois
da vitória efetiva, nunca substituí-la por um resultado automático.

O terminal é parte da encenação: contraste legível, poucas linhas por bloco
e palavras originais do universo do Linker. Nenhum texto explica toda a lore
ou exige conhecimento de programação para entender a ameaça e a reação de Rust.

## Menor gameplay completo

- Rust se move num pequeno espaço lateral com chão e limites legíveis.
- Uma errática possui aproximação, antecipação, ataque e recuperação próprios;
  ataca Rust e pode vencê-lo se ele não responder.
- Rust dispõe de movimento e ataque suficientes para evitar e punir a ameaça.
  Alcance, contato, dano e reação precisam corresponder ao que se vê.
- O encontro possui vitória, derrota, pausa e nova tentativa. A progressão
  narrativa depende do estado do encontro, sem regras de rounds ou energia
  herdadas implicitamente do jogo de luta.
- O encerramento mostra a reação de Rust e oferece uma forma clara de rever
  ou encerrar o experimento.

O primeiro teste tem uma ameaça, um espaço e uma consequência. Ele não precisa
de vários inimigos, árvore de habilidades, combos extensos, inventário ou
exploração de uma cidade inteira para cumprir essa história.

## Animação e produção visual

O alvo é atuação expressiva, como a ambição demonstrada pelos especiais do
Prototype 3, aplicada também aos gestos cotidianos. As poses e os efeitos
existentes de Rust podem servir como base; a adequação à aventura precisa ser
conferida em movimento. Cenas compostas com camadas e arte provisória são úteis
para revisar ritmo, sem equivaler a animação corporal final.

| Grupo | Produção limitada ao episódio |
|---|---|
| Ada | Trabalho, percepção da mensagem e reação; origem humana visível |
| Assembly | Aparição/despertar com presença e mistério |
| Rust | Despertar, movimento, ação/reação do encontro e balanço de cabeça com pesar |
| Errática | Presença, aproximação, ataque legível, dano e derrota |
| Ambiente | Composição de trabalho de Ada e pequeno espaço da manhã de Rust |
| Interface | Terminal, indicação de comandos, pausa e tentativa/encerramento |

A [direção de arte](07-art-direction.md) orienta o tom. Silhuetas e movimento
precisam continuar legíveis durante efeitos; cenário, terminal e câmera não
podem esconder o contato do combate. Uma demonstração gravada deve incluir a
passagem de controle e o gesto final, além dos quadros mais vistosos.

## Isolamento de implementação

Aventura e luta têm features Cargo e binários próprios. O primeiro núcleo
compartilhado contém utilidades como `math` e `runtime_paths`; narrativa,
movimento, inimigo e regras de progressão da aventura pertencem ao experimento.
A [ADR 0021](adr/0021-isolated-adventure-experiment.md) e a
[arquitetura](08-code-architecture.md) detalham os limites efetivos.

Não introduzir regras da aventura no estado das partidas para aproveitar
seleção, rounds ou cinematográficos de dois lutadores. Recursos visuais podem
ser reaproveitados quando cabem na cena, sem assumir que os sistemas de luta
sejam o modelo da aventura. O diário registra comandos, organização e revisão
real do binário, em vez de tratar a existência dos arquivos como entrega.

## Critérios de aceite

1. A execução percorre Ada, mensagem, Assembly, salto temporal, manhã de Rust,
   encontro controlado pelo jogador e reação final, na ordem autorizada.
2. Ada aparece inicialmente humana; a cena não revela a autoria da mensagem
   nem apresenta uma explicação nova para a transformação ou para o Linker.
3. Rust parece estar começando um dia comum. Seu gesto depois da vitória
   comunica pesar e necessidade, sem festa ou hostilidade a todas as EPs.
4. A errática tenta atingir e matar Rust. Contato válido causa dano; ataques
   fora de alcance não acertam. O jogador pode vencer ou perder por suas ações.
5. Pausa congela o encontro; retomar preserva seu estado. Derrota permite
   tentar novamente sem assistir obrigatoriamente ao prólogo.
6. Texto digitado pode ser revelado/avançado e a abertura pode ser pulada.
   Entradas de confirmação não se tornam ataques involuntários na transição.
7. Features/binários preservam o jogo de luta, e regras de aventura têm testes
   independentes. Formatação, Clippy e testes apropriados passam; limitações de
   execução são registradas com precisão no diário.
8. Capturas ou vídeo do renderer mostram a sequência e o contato real; revisão
   de estados sozinha não recebe status de aprovação humana da animação.
9. Em playtest humano posterior, registrar entendimento da ameaça, clareza dos
   comandos, leitura do pesar de Rust e vontade de continuar. Diversão e
   interesse do público continuam hipóteses até essa observação.

## Ideias estacionadas

A proposta anterior sugeria **Vínculo**, plataformas instáveis, um capítulo em
**Sirius**, uma entidade de apoio, um desvio com pista de Ada e um núcleo de
ruptura. Essas ideias estão preservadas como possibilidades posteriores; não
integram a implementação autorizada deste episódio. Também ficam para depois
campanha de 10–15 minutos, múltiplos heróis, mapa aberto e confronto final com
Assembly. Retomá-las exige reavaliar escopo a partir do primeiro encontro.

O principal risco desta rodada é produzir uma abertura vistosa com pouco
controle interessante, ou perder a humanidade de Rust atrás dos efeitos.
A revisão precisa considerar o encontro e o gesto final com o mesmo cuidado
que Ada e Assembly.
