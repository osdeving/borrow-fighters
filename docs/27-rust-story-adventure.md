# 27 — Proposta: aventura de Rust e os ecos do Linker

## Estado e intenção

Proposta exploratória de 10 de setembro de 2026, após o Prototype 3. O pedido
atual é explorar uma aventura 2D single-player protagonizada por Rust, com
animações expressivas, prólogo mostrando Ada e Assembly e texto digitado em
terminal antes da aventura. O [backlog](03-backlog.md) registra essa frente e
preserva as tarefas e os bugs do playtest de luta.

Esta rodada documenta a direção e o menor teste jogável recomendado. Mecânicas,
diálogos, título de capítulo e acontecimentos novos abaixo são propostas, não
cânone aprovado nem implementação entregue. A versão distribuída continua
descrita nas [notas do Prototype 3](releases/v0.1.0-prototype.3.md).

## Recomendação

Uma aventura de ação lateral por capítulos, com ambientes conectados, pequenos
desvios para descobertas, travessia, encontros de combate e cenas autorais.
Rust é o protagonista jogável. A progressão vem de alcançar lugares, compreender
o que aconteceu e aprender a usar o Linker para proteger pessoas e entidades.

Promessa proposta: **conduzir Rust por um Brasil onde abstrações ganham corpo,
conter rupturas e descobrir por que a primeira entidade reconhece sua assinatura.**

| Formato considerado | O que favorece | Custo ou limite para esta intenção |
|---|---|---|
| Campanha de duelos com cenas entre lutas | Aproveita diretamente as partidas atuais | A exploração e a descoberta ficam concentradas em cenas e texto |
| Beat 'em up de progressão lateral | Movimento pela fase e encontros com vários inimigos | Exige combate de grupos e pode repetir corredores de combate |
| Aventura lateral por capítulos — recomendada | Alterna descoberta, travessia, personagens e ação; permite dirigir cada cena | Precisa de espaços exploráveis, câmera, interações e inimigos próprios |
| Metroidvania amplo | Exploração não linear e retorno com novas habilidades | Mapa, progressão, revisitas e testes crescem juntos; avaliar após o capítulo piloto |

Como referências de intenção, [Ori and the Blind Forest](https://www.orithegame.com/blind-forest/)
associa aventura de plataforma, atuação animada e história emocional;
[Prince of Persia: The Lost Crown](https://news.ubisoft.com/en-us/article/e1SD5gR3TWqk5GPAjWHSz)
combina combate, plataforma e exploração. A proposta aproveita essas relações
entre ação e narrativa como inspiração, sem assumir o volume de conteúdo ou a
direção visual desses jogos.

## História através das ações

O [worldbuilding](12-worldbuilding.md) estabelece a convivência como conflito
central. Rust protege entidades estáveis e contém as erráticas; Duke negociou
sua permanência entre humanos; Assembly quer devolver as abstrações aos
circuitos e impedir o acesso humano ao Linker.

O ciclo proposto é: **avistar uma anomalia → explorar e entender sua causa →
atravessar ou conter usando o Linker → observar uma consequência → descobrir
uma pista que conduz ao próximo lugar.**

O ambiente deve mostrar a vida afetada pela ruptura: uma passagem desaparece,
uma entidade estável tenta manter um equipamento funcionando, um registro
apresenta a mesma assinatura encontrada numa máquina antiga. Ajudar essa entidade
e restabelecer a passagem torna a posição de Rust compreensível através do jogo.

Os terminais oferecem pistas curtas, imagens e conversas. A rota principal
continua compreensível para quem lê pouco e para quem nunca programou. Um desvio
opcional deve revelar algo específico sobre Ada ou o lugar visitado, em vez de
servir apenas para acumular objetos. Narrativa ramificada e sistema de moralidade
ficam para avaliação posterior.

## Uma habilidade para testar: Vínculo

Nome provisório. Rust estabiliza uma forma que está perdendo sua presença física.
É uma metáfora de vínculo e responsabilidade, não uma simulação do compilador.

- **Travessia:** uma plataforma intermitente permanece sólida enquanto vinculada.
- **Combate:** uma criatura fora de fase torna-se atingível enquanto vinculada;
  ela continua andando e atacando. A habilidade não congela qualquer adversário.
- **Escolha:** Rust mantém apenas um vínculo por vez. Vincular outro alvo devolve
  o anterior ao seu comportamento normal, com antecipação visual legível.

No primeiro teste, o comando alterna o vínculo no alvo elegível à frente, dentro
de um alcance visível. Marcadores indicam qual alvo será afetado. Não há mira
livre, física de cordas, árvore de poderes ou medidor adicional. A ligação é
mostrada por contorno, símbolo e movimento, além da cor.

Primeiro aprende-se com uma plataforma sobre piso seguro. Depois, com uma
criatura isolada. Só então uma sala combina travessia e combate: manter um
caminho sólido ou tornar a ameaça vulnerável. A troca jamais deve produzir
queda inevitável sem aviso; erros devolvem Rust a um ponto seguro próximo.

Esse poder é a hipótese principal de diversão. O menor teste deve descobrir
se escolher e trocar o vínculo cria decisões interessantes. Se virar apenas
um botão obrigatório antes de todo ataque, precisa ser revisto.

## Abertura proposta: Ada, Assembly e o terminal

O pedido atual coloca Ada em cena, expandindo a apresentação indireta sugerida
nos documentos anteriores. Ela mantém presença intelectual e misteriosa. O
prólogo mostra um acontecimento decisivo e guarda suas causas completas para
a aventura.

Base provisória: conforme o worldbuilding, **o Linker já existia; Ada foi a
primeira humana conhecida a acessá-lo e libertou Assembly**. A ficção pertence
ao universo do jogo, sem pretensão de reconstituição histórica.

Há uma divergência editorial a resolver antes do roteiro final: o prólogo de
[story.json](../assets/lore/story.json) descreve o surgimento do Linker de outra
forma e o capítulo de Ada contém uma frase ambígua sobre a emergência de
Assembly. Esta proposta usa o worldbuilding como referência provisória; o texto
do menu não foi reescrito nesta rodada.

Alvo de duração: 45–60 segundos de cena, seguidos de 10–15 segundos de terminal.
O jogador pode revelar o texto inteiro, avançar e pular o prólogo; repetição após
morte começa no checkpoint da aventura. A sequência pode ser revista no menu.

| Momento | Imagem e movimento | Função narrativa |
|---|---|---|
| 1. A intenção | Close de Ada escrevendo; símbolos se alinham com movimentos de uma máquina | Mostrar alguém compreendendo uma ligação impossível |
| 2. A resposta | Fios de luz percorrem papel, metal e espaço; Ada percebe que a máquina respondeu além do esperado | Introduzir o Linker através de uma ação |
| 3. A travessia | Assembly emerge incompleto, alternando matéria e lacunas; Ada e a entidade se observam | Dar presença e estranheza à primeira entidade |
| 4. A interrupção | Assembly toca um símbolo; som e movimento cessam, deixando um sinal isolado | Sugerir sua filosofia e manter uma pergunta aberta |
| 5. O presente | O sinal reaparece como cursor num terminal em Sirius | Ligar o prólogo ao lugar onde Rust começa |

Rascunho de texto original, digitado em blocos curtos. O ritmo de um terminal
que parece responder ao observador é a referência pedida; palavras, grafismo e
encenação terão identidade própria do Linker.

```text
> recuperando um registro sem origem...

Antes que símbolos ganhassem corpo,
Ada ouviu uma resposta.

A primeira entidade atravessou.
Depois, quis fechar a passagem.

> sinal localizado: SIRIUS / CAMPINAS
> entidade em campo: RUST
> há alguém do outro lado.
```

A câmera se afasta do terminal e revela Rust no mesmo espaço. Um ruído além de
uma porta indica o primeiro objetivo; o controle passa ao jogador. Esse corte
conecta a história diretamente à exploração.

## Primeiro capítulo: O sinal que não devia existir

Título provisório. Um setor fictício inspirado no Sirius sofre uma ruptura;
Rust procura uma entidade presa e encontra um registro que reconhece sua
assinatura. O capítulo deve entregar uma pequena história completa: chegar,
compreender a ameaça, ajudar, conter a ruptura e sair com uma nova pergunta.

Alvo de experiência: **10–15 minutos**, validado em playtest, incluindo a abertura.

| Trecho | Ação do jogador | Descoberta ou recompensa |
|---|---|---|
| Entrada e observação | Andar, saltar e seguir o sinal por um espaço seguro | O laboratório abriga algo vivo além das máquinas |
| Passagem em falha | Estabilizar a primeira plataforma e liberar a travessia | O Vínculo muda fisicamente o mundo |
| Encontro e desvio | Ajudar uma entidade estável; explorar uma pequena sala opcional | Uma pista de Ada dá sentido ao sinal do prólogo |
| Contenção | Enfrentar uma criatura fora de fase; depois combinar vínculo e travessia | A habilidade tem utilidade e limites compartilhados |
| Núcleo da ruptura | Enfrentar uma versão maior da mesma ameaça, com padrão legível | Rust contém o núcleo numa animação autoral curta |
| Consequência | Atravessar o espaço agora estabilizado e recuperar o registro | A assinatura de Assembly reconhece Rust; surge a próxima pista |

Assembly aparece como presença ou assinatura neste capítulo. Sua luta final
permanece uma possibilidade de arco futuro. Duke, Old C e Python podem guiar
capítulos posteriores em seus lugares de origem; esse horizonte não implica
produzir todas as cidades ou tornar o elenco inteiro jogável agora.

## Animação como parte da aventura

A ambição é ter atuação e movimento expressivos durante toda a experiência.
Priorizar aceleração e parada de Rust, aterrissagem, gesto de vínculo, reação ao
ambiente, olhar e interação; esses movimentos serão vistos muitas vezes.
Reservar encenações mais longas para revelações e encerramentos de capítulo.

Os especiais existentes demonstram uma linguagem de poses, efeitos e áudio que
pode informar as novas cenas. Sequências de dois lutadores numa arena precisam
de adaptação para funcionar em travessia, combate com inimigos e câmera móvel.

O prólogo pode combinar desenhos em camadas, câmera, luz e poses animadas, com
movimento mais elaborado no gesto de Ada e na aparição de Assembly. Um animatic
serve para revisar duração e continuidade antes da produção dos desenhos.

| Grupo de produção | Limite inicial proposto |
|---|---|
| Protagonista | Rust; avaliar reaproveitamento de andar, salto, golpes e reações; acrescentar vínculo e interação |
| Cenário | Um setor de Sirius com três salas principais, um pequeno desvio e elementos de travessia |
| Encontros | Um tipo de inimigo e uma variação maior para o núcleo; no máximo dois inimigos simultâneos no piloto |
| Personagem de apoio | Uma entidade estável com animação curta de presença/interação |
| Prólogo | Quatro composições principais e a transição pelo terminal; Ada e Assembly com atuação limitada à cena |
| Momento de espetáculo | Uma contenção final de 3–5 segundos, acionada após sucesso do jogador |

As arenas bitmap existentes são referências e possíveis bases de composição;
não representam fases extensas prontas. Desenhar camadas, chão e plataformas
exploráveis será trabalho adicional. O mesmo vale para inimigos, câmera e
retorno ao checkpoint: reaproveitamento precisa ser verificado, não presumido.

## Ordem de trabalho sugerida

1. **Fechar o tratamento curto:** conciliar a origem com o texto do menu,
   desenhar a sequência de abertura e o percurso do capítulo. Registrar decisões
   criativas aceitas; uma mudança consolidada de direção pede ADR e atualização
   de visão, Mini-GDD, escopo e arte.
2. **Provar três minutos de jogo:** após pedido de implementação, montar uma
   sala simples com Rust, plataforma instável, criatura e Vínculo. Testar movimento,
   legibilidade e uma decisão de troca. Usar poses existentes e formas provisórias.
3. **Revisar o animatic em paralelo ao teste de gameplay:** conferir Ada,
   Assembly, terminal e passagem de controle com quadros provisórios.
4. **Produzir o capítulo de 10–15 minutos:** depois de validar a habilidade,
   acrescentar o desvio, personagem de apoio, checkpoint e contenção final;
   investir animação nas ações mais vistas e nos dois momentos narrativos.
5. **Playtestar o capítulo:** observar entendimento, fluidez e vontade de
   continuar; usar o resultado para decidir a expansão da aventura.

O teste de três minutos valida gameplay. O capítulo piloto valida a combinação
de história, exploração, ação e apresentação. Nenhum deles assume campanha
completa, mapa aberto, crafting, inventário de equipamentos ou múltiplos heróis.

## Critérios de avaliação e riscos

- O jogador explica o efeito e a limitação do Vínculo sem precisar saber Rust.
- A mesma habilidade resolve uma travessia e um encontro, com decisão visível
  sobre qual alvo manter estável.
- O jogador entende quem está ajudando e por que continuar até o núcleo.
- A descoberta opcional acrescenta contexto, sem bloquear quem a ignorar.
- O movimento de Rust é agradável antes das cenas de espetáculo.
- Morrer permite retomar rapidamente; texto digitado e prólogo não impõem espera
  em novas tentativas. Nenhum efeito visual esconde o alvo ou a plataforma.
- No playtest, registrar onde as pessoas se perdem, pulam texto, repetem uma
  ação sem entender e demonstram interesse espontâneo pelo próximo trecho.

Os principais riscos são concentrar a produção na abertura e deixar pouco jogo,
transformar o Vínculo numa tarefa repetitiva e ampliar cidades/inimigos antes de
validar uma sala. O teste pequeno e o orçamento de animações respondem a esses
riscos. A proposta é candidata a experimento posterior ao corte de luta 0.1;
diversão, duração final e interesse do público ainda precisam ser demonstrados.
