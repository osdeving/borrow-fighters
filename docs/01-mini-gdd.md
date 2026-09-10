# 01 — Mini-GDD

## 1. Resumo

**Borrow Fighters** explora uma aventura 2D narrativa protagonizada por Rust,
com pessoas de natureza mística inspiradas em linguagens e conceitos técnicos.
O jogo de luta já entregue permanece como Prototype 3; seus requisitos iniciais
estão preservados neste documento como histórico.

O objetivo atual é implementar o [episódio curto da aventura](27-rust-story-adventure.md):
Ada trabalha como uma humana comum, recebe uma mensagem misteriosa após
aprender sobre o Linker, e Assembly desperta. Muito tempo depois, Rust acorda
para uma manhã comum, é atacado por uma errática, vence em gameplay e balança
a cabeça com pesar pelo que precisou fazer.

A direção narrativa coloca essas entidades em arenas brasileiras de ciência, tecnologia, arquitetura e inovação. O pano de fundo é cósmico e sóbrio: **O Linker** é uma força antiga que liga símbolos, matéria, circuitos e crença humana, permitindo que entidades programáticas surjam no mundo real.

Ada é a primeira usuária humana do Linker e se torna híbrida humana e EP.
Assembly é a primeira EP a despertar. A autoria da mensagem e a relação completa
entre esses acontecimentos permanecem mistérios. EPs têm emoções e diversidade
moral; os humanos também. Rust é a última e mais recente EP pura conhecida e
defende a coexistência. A [lore](12-worldbuilding.md) orienta esses personagens.

## 2. Gênero

- Experimento atual: aventura de ação 2D narrativa solo.
- Side-view.
- Episódio curto com prólogo, passagem de controle, encontro e consequência.
- Produto existente: luta 2D local em partidas curtas.

## 3. Plataforma inicial

- PC desktop.
- Desenvolvimento local.
- Sem compromisso inicial com Steam, consoles ou mobile.

## 4. Stack inicial

- Linguagem: Rust.
- Biblioteca/engine inicial: Raylib/Raylib-rs.
- Arquitetura: micro-engine própria evolutiva.
- Arte inicial: placeholder.
- Som inicial: placeholder ou ausente.

A aventura usa features Cargo e binário próprios, com regras independentes das
partidas de luta. Compartilha apenas utilidades de base como `math` e
`runtime_paths`, conforme a [ADR 0021](adr/0021-isolated-adventure-experiment.md).
O [diário](worklogs/rust-adventure-prologue.md) registra o estado verificável da
implementação. As seções de loop, mecânicas e critérios do primeiro protótipo
abaixo documentam o corte de luta; os critérios atuais estão na [entrega 27](27-rust-story-adventure.md).

## 5. Loop do primeiro slice de luta — histórico

1. Jogador se move.
2. Jogador tenta acertar o adversário.
3. Ataques geram hitboxes.
4. Hitboxes colidem com hurtboxes.
5. Dano é aplicado.
6. Barra de vida diminui.
7. Partida termina quando a vida chega a zero.
8. Jogo pode ser reiniciado.

## 6. Mecânicas do primeiro slice de luta — histórico

### Movimento

- Andar para esquerda/direita.
- Pular.
- Cair.
- Virar automaticamente para o adversário.

### Ataque

- Ataque básico inicial.
- Ataque pesado futuramente.
- Ataque especial futuramente.

### Colisão

Separar:

- colisão física com chão/limites;
- hurtbox: área vulnerável do personagem;
- hitbox: área ofensiva do golpe.

### Dano

- Dano fixo inicialmente.
- Knockback simples pode ser adicionado depois.
- Sem balanceamento complexo no início.

## 7. Personagens iniciais

### Rustacean / Rust

Arquétipo: lutador técnico, seguro e preciso.

Papel narrativo: protagonista justo, nobre e cuidadoso, capaz de agir com mais
humanidade que muitos humanos. Rust é a última e mais recente EP pura conhecida.
Defende a coexistência entre humanos e EPs com segurança, liberdade e
responsabilidade. Enfrenta erráticas quando precisa proteger vidas e sente
pesar pelo que é necessário fazer.

Possíveis golpes:

- Borrow Check.
- Lifetime Lock.
- Ownership Transfer.
- Panic!
- Zero-Cost Counter.

### Duke / Java

Arquétipo: lutador verboso, resistente e cheio de pressão.

Papel narrativo: veterano carismático. Duke é teatral, antigo, cheio de cerimônia e mais sábio do que parece. Ele fez acordos com humanos, empresas e instituições para permanecer no mundo real por mais tempo. Essa aliança lhe dá estabilidade, influência e um aspecto corporativo, mas também o torna moralmente ambíguo.

Possíveis golpes:

- System.out.println Barrage.
- Garbage Collector Sweep.
- AbstractFactory Uppercut.
- NullPointer Trap.
- Enterprise Combo.

### Assembly

Arquétipo: boss final não-jogável, antigo, poderoso e quase místico.

Assembly representa a camada anterior às abstrações confortáveis. Ele não é vilão simples: é a primeira consequência do despertar humano para o Linker. Quer impedir que humanos conheçam essa força e constranger entidades programáticas de volta aos circuitos, à matéria e aos limites da máquina.

Por estar sendo esquecido, Assembly fica parcialmente fora de fase. Partes do corpo alternam entre matéria, lacunas, `0` e `1`. Isso explica por que ele não é jogável no primeiro arco: sua existência física é instável demais, embora seu poder continue enorme.

### Usuários humanos do Linker

Programadores humanos são pessoas com diferentes níveis de acesso ao Linker.
Alguns conseguem perceber EPs, ajudá-las a se estabilizar ou cooperar com elas.
As entidades mantêm vontade própria, emoções e diferenças morais.

Essa é a camada metalinguística do jogo: aprender comandos, timing e personagem representa aprender a operar uma entidade pelo Linker.

### Frontenzos

Frontenzos são humanos com pouca aderência ao Linker e excesso de confiança.
Seu mau uso da força cria condições cósmicas para o surgimento involuntário de
EPs erráticas. Não são invocadores deliberados dessas entidades.

Rust defende a coexistência e contém ameaças concretas. Isso não torna toda EP
inimiga, todo humano culpado ou a vitória sobre uma errática motivo de festa.

Mais detalhes de história, personagem e arenas estão em [`docs/12-worldbuilding.md`](12-worldbuilding.md). No protótipo jogável, um corte curto dessa história aparece no menu `Lore / Roster`, carregado de `assets/lore/story.json` para permitir edição sem rebuild.

## 8. Arenas

As arenas principais devem ser inspiradas em locais brasileiros de tecnologia, ciência, arquitetura e inovação.

Direções iniciais:

- MASP, em São Paulo, como portal urbano e arena suspensa;
- Sirius/LNLS, em Campinas, como altar científico de luz e matéria;
- Brasília/BioTIC, como cidade planejada e biotecnológica;
- Fortaleza, como costa tecnológica de energia, formação e sistemas;
- Curitiba/Vale do Pinhão, como smart city cartunesca;
- Recife/Porto Digital, como passado e futuro no mesmo commit.

A lista completa de possibilidades e ganchos narrativos vive em [`docs/12-worldbuilding.md`](12-worldbuilding.md).

## 9. Vertical slice inicial de luta — histórico

O vertical slice deve demonstrar uma luta curta entre dois personagens com:

- identidade visual mínima;
- pelo menos dois ataques por personagem;
- feedback visual de impacto;
- barra de vida funcional;
- uma arena simples;
- tela de vitória;
- controles responsivos;
- código modular o suficiente para adicionar novos personagens.

## 10. Fora de escopo inicial da luta — histórico

- Online multiplayer.
- Menu completo.
- Modo história completo.
- Vários personagens.
- Vários cenários.
- Sistema complexo de combo.
- Inteligência artificial avançada.
- Arte final.
- Animações finais.
- Trilha sonora final.
- Balanceamento refinado.

## 11. Critério de sucesso do primeiro protótipo de luta — histórico

O primeiro protótipo será considerado bem-sucedido quando:

- dois personagens aparecerem na tela;
- ambos puderem se mover;
- um personagem puder atacar o outro;
- o dano for aplicado corretamente;
- a barra de vida diminuir;
- a partida terminar quando a vida chegar a zero;
- o jogo puder ser reiniciado.
