# 34 — Chegada, biografias e passagem destrutível

Implementação solicitada em 12/09/2026, após o primeiro capítulo. Preserva a
identidade e os cenários existentes; amplia os conteúdos exclusivos da aventura.

- A pipa continua abrindo a rua. A primeira EP ganha uma chegada própria:
  foco no céu, queda contínua, enquadramento jogável antes do impacto, apoio
  de joelho/mão, poeira e recuperação. O contato com o chão inicia o tumulto.
- Rust usa oito poses de caminhada com apoios estáveis e avanço pela distância
  percorrida, compartilhadas pelo prólogo, gameplay e trajetos de conversa.
  Os apoios do despertar são editáveis e Rust caminha até a saída do quarto.
- Duke chega de limousine a uma torre espelhada na Paulista e preside a
  reunião na cabeceira da mesa. Sua forma de mascote permanece reconhecível.
- Old C trabalha num setup rústico atual, com computador antigo, livros K&R
  e de ciência da computação, preservando sua identidade grisalha e jeans.
- Na travessa, caixas e destroços bloqueiam a passagem. Socos e o novo chute
  causam dano por contato; peças racham, caem e se fragmentam. Depois da
  abertura e travessia, duas EPs com tuning individual guardam a passagem.
  Vitória exige derrotar ambas; retry restaura o encontro inteiro.

## Controles e edição

**V / RT** chuta, além de **J/F / X** para soco e **K/H / Y** para golpe forte.
Os demais controles de movimento, salto, interação e defesa são preservados.
Durante a chegada da EP, **Enter / RB** conclui a aterrissagem e entrega o
combate; o comando de pular tudo mantém seu comportamento.

| Conteúdo | Arquivo de edição |
| --- | --- |
| Câmera, queda e impacto da EP | [ep-arrival.json](../assets/adventure/street/ep-arrival.json) |
| Poses, apoios e clip de Rust | [Catálogo](../assets/adventure/locomotion/catalog.json) |
| Passada e apoios da manhã | [Movimento](../assets/adventure/locomotion/motion.json), [despertar](../assets/adventure/locomotion/waking.json) |
| Peças de Duke/Old C | [Catálogo visual](../assets/adventure/opening/scenes/catalog.json) |
| Composição e trajetórias das biografias | [Cenas](../assets/adventure/opening/scenes/scenes.json) |
| Textos das cenas/livros/terminal | [Português](../assets/adventure/texts/pt-BR.json) |
| Posições, resistência e inimigos do capítulo | [Mundo](../assets/adventure/chapter/world.json) |
| Instruções e falas do capítulo | [Textos do capítulo](../assets/adventure/chapter/chapter-texts.json) |

O catálogo compartilhado aceita PNGs avulsos e recortes de atlas; as instâncias
não dependem da disposição física dos arquivos. Novas peças e mudanças locais
não exigem regenerar o cenário. Biografias usam câmera e transforms interpolados,
com textos presos aos objetos. A apresentação dura 64 segundos e a trilha
original acompanha os novos cortes. [Guia das biografias](../assets/adventure/opening/scenes/README.md).

**F5** recarrega textos e conteúdo visual editável durante a sessão, sem zerar
a história. Geometria e resistência do mundo são carregadas ao iniciar o
capítulo; reabra a sessão após editar esses dados. A recarga de cada conjunto
valida suas referências antes da substituição.

[Decisão de arquitetura](adr/0028-editable-cinematic-tracks-and-destructibles.md) ·
[Evidências e limites](evidence/cinematic-expansion/README.md) ·
[Diário de trabalho](worklogs/cinematic-expansion.md).
