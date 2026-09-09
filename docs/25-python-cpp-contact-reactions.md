# 25 — Python e C++: reações sincronizadas por contato

## Pedido e escopo

O playtest após `caa3858` rejeitou a leitura das reações: durante a rajada de C++,
a adversária parecia uma estátua. A cobertura de estados da rodada 24 não bastou
para demonstrar que cada pancada tinha uma resposta corporal convincente.

Esta rodada estabelece e valida o padrão somente para **Python contra C++ e
C++ contra Python**. Os demais personagens ficam para uma rodada posterior.
Não altera dano, alcance, caixas, custo ou raridade dos golpes.

## Padrão de implementação

1. Cada contato confirmado escolhe um perfil visual explícito: cabeça, tronco,
   perna/baixo, defesa alta, defesa baixa, lançamento, queda ou recuperação.
2. O primeiro desenho já mostra impacto, com pose corporal própria. Os desenhos
   seguintes mostram continuação do movimento e recuperação; não depender de
   piscar a cor, rotacionar o corpo rígido ou começar em uma pose quase neutra.
3. A apresentação registra o instante do contato e sua duração visual, separada
   do stun físico. Cada contato da rajada de C++ reinicia uma resposta de até
   nove frames, antes da pancada seguinte aos dez frames.
4. A pose ofensiva da rajada e a reação do defensor compartilham a mesma agenda
   de contatos. Cada soco/chute precisa ser visível no instante em que causa dano.
5. Os atlas opcionais usam `reaction_head`, `reaction_body`, `reaction_low`,
   `reaction_guard_high`, `reaction_guard_low`, `reaction_launch`,
   `reaction_fall` e `reaction_rise`. Cada personagem recebe desenhos próprios;
   os clips anteriores e `combat_manifest` permanecem como base/fallback.
6. Queda e recuperação seguem o voo/chão do World; KO permanece caído. Captura,
   deglutição, defesa, hitstop, pausa, avanço unitário e replay respeitam o relógio
   de combate e não criam contatos ou movimentos físicos adicionais.

Decisão: [ADR 0018](adr/0018-contact-reaction-profiles.md).

| Contato ou fase | Clip do defensor | Leitura corporal |
|---|---|---|
| Overhead e golpes aéreos | `reaction_head` | Cabeça recua, ombros abrem e tronco recompõe a postura. |
| Socos leve/forte, chute e projétil no chão | `reaction_body` | Tronco dobra, braços protegem o abdômen e pernas absorvem. |
| Rasteira / contato baixo | `reaction_low` | Perna cede antes da queda. |
| Defesa em pé / abaixada | `reaction_guard_high` / `reaction_guard_low` | Guarda comprime e devolve o peso à base. |
| Anti-air, arremesso, assinatura e lançamento do super | `reaction_launch` | Corpo perde apoio, articula no voo e prepara a aterrissagem. |
| Aterrissagem / KO | `reaction_fall` | Impacto lateral, acomodação no chão; KO segura a última pose. |
| Recuperação após queda | `reaction_rise` | Apoio no braço, joelho, perna e retorno ao combate. |

A altura/guarda e o estado aéreo no contato têm precedência quando aplicáveis.
No arremesso, a captura segura a primeira pose de lançamento até a soltura real.
A sucção da Python usa a sequência de voo durante a aproximação à boca; depois
o alvo fica oculto durante a deglutição e retorna com queda/recuperação.

Na rajada, os oito contatos ocorrem nos ticks **344, 354, 364, 374, 384, 394,
404 e 414**. Os punhos e pés dessa arte ofensiva acertam a faixa de rosto/pescoço;
por isso os oito contatos usam reação de cabeça. A revisão em combate descartou
o perfil de tronco para o chute frontal: ele abaixava a Python sob a canela e
parecia esquiva. Cada contato percorre os quatro desenhos em nove
frames, conserva a última pose até o seguinte e reinicia no desenho de impacto.
O stun físico anterior de 24 frames continua independente dessa amostragem.

## Critérios de aceite

- Python reage individualmente a cada pancada da rajada de C++, no mesmo frame.
- Socos, chutes, rasteira, overhead, anti-air, golpes aéreos, projétil, arremesso,
  assinatura e cinematográfico têm resposta legível nas duas personagens.
- Defesa alta/baixa absorve com postura apropriada; esquiva por distância/altura
  e golpe que erra não acionam reação de dano.
- Ser atingida ao atacar, abaixar, andar ou estar no ar interrompe a ação visual
  correta; voo, pouso, queda, recuperação e KO não mostram idle prematuro.
- Contato, pico da reação e recuperação são conferidos em movimento em 60 fps,
  nas duas direções. O vídeo mostra as duas personagens juntas, com o corpo inteiro.
- Testes verificam cada contato e sua progressão, não apenas a existência de
  dois frames diferentes em qualquer instante da sequência completa.
- Documentação, prompts, atlas, clips, exemplos reproduzíveis e laudos descrevem
  o padrão e suas limitações para orientar a próxima rodada.

## Estado

Implementado e verificado em 9 de setembro de 2026 para o par solicitado.

- **64 desenhos novos**: oito clips de quatro desenhos para cada personagem.
  Fontes, prompts, exportação e limites de acabamento estão na produção de
  [Python](../assets/production/python/reactions-2026-09-09/README.md) e
  [C++](../assets/production/cpp/reactions-own-2026-09-09/README.md).
- Os 88 frames e 25 clips anteriores de cada candidato permanecem intactos;
  nenhum desenho novo introduz metadata de combate.
- **64 cenários** renderizados: 12 ataques e quatro situações de defesa por
  personagem, nos dois sentidos. As 16 janelas da rajada mostram impacto no
  mesmo tick do contato e todos os quatro desenhos antes da pancada seguinte.
- **346 testes passaram**, zero falhas, um teste de áudio com dispositivo
  ignorado na suíte padrão. Fmt e Clippy com todos os alvos/features passaram.
- [Vídeo de 86,3 s a 60 fps](../assets/showcase/python-cpp-reactions-2026-09-09.mp4),
  [80 frames consecutivos da rajada](evidence/python-cpp-reactions/cpp-barrage-80-frames.png)
  e [laudo reproduzível](evidence/python-cpp-reactions/README.md).

A revisão visual usa os frames reais do renderer e progressões extraídas do
vídeo; não substitui o playtest humano de fluidez. O áudio do vídeo é mixado
offline dos eventos do World, sem alteração do player nesta rodada. O padrão
fica pronto para extensão; nenhuma nova aprovação visual é atribuída ao resto
do elenco.

## Correção do treino sem dano

O playtest seguinte revelou uma lacuna na validação: com `Player 1 recebe dano`
ou `Player 2 recebe dano` desligado, o World pulava a rotina inteira de contato,
incluindo a ativação das reações. A rajada exibia `HIT -0`, mas o defensor ficava
em idle. Os sprites estavam presentes; o bloqueio era no código. As capturas
anteriores desta rodada usaram dano habilitado.

A opção deve impedir apenas a redução de HP. Contato confirmado continua
interrompendo a ação, ativando a reação/guarda, empurrando, arremessando ou
lançando e produzindo feedback sonoro conforme o golpe. Guarda não pode cobrar
chip quando o dano está desligado. Golpes que erram continuam sem reação.
Essa regra vale independentemente do personagem ou slot; a arte nova permanece
limitada ao piloto Python/C++.

Correção implementada: **349 testes passaram**, zero falhas, um teste de áudio
com dispositivo ignorado; Fmt e Clippy aprovados. A [nova captura com dano
desligado](evidence/python-cpp-reactions/no-damage/README.md) confirma HP cheio,
os quatro desenhos por pancada nos 80 ticks da rajada e queda/recuperação de
C++ ao receber o especial de Python.

## Aplicar o padrão na próxima rodada

1. Revisar a identidade e as poses do personagem contra a arte já integrada.
   Produzir quatro desenhos articulados para cada um dos oito clips, com impacto
   no primeiro desenho, fundo alpha real e ponto de apoio consistente.
2. Acrescentar os clips opcionais ao candidato, preservando os frames antigos e
   o manifesto de combate. Registrar fontes, prompts, recortes, pivôs e escala.
3. Habilitar o personagem no contrato `uses_contact_reactions`. Contatos comuns
   reutilizam o perfil existente; novos contatos cinematográficos precisam de uma
   agenda compartilhada entre pose ofensiva, efeito e reação do defensor.
4. Validar contato a contato no World, incluindo guarda, erro, ar, queda, KO,
   pausa, replay e flags de dano desligadas em cada slot. Comparar a metadata
   física antes/depois da troca de desenho.
5. Conferir o par em movimento e nos dois sentidos: impacto, pico, recuperação,
   articulação, escala, pés no chão e encaixe visual do golpe. Um teste de seleção
   de frames sozinho não aprova a qualidade da animação.
