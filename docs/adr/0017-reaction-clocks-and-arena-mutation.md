# ADR 0017 — Relógios de reação e mutação persistente de arena

## Status

Aceito e implementado para o pedido explícito da rodada 24.

## Decisão

Reações devem consumir estado e relógio do contato, com poses e transformação
visual próprias; não usar o relógio de idle/ataque para animar dano. O domínio
mantém autoridade sobre HP, stun, voo, queda e captura. Renderizadores compartilham
o mesmo retrato de reação em luta e ferramentas, com intensidade maior em supers.

Python e o novo roteiro de C++ estendem os dados explícitos de `SuperSequence`,
sem motor de cutscenes. A deglutição esconde temporariamente o alvo, mas não o
remove do World; retorno, guarda, reset e KO permanecem determinísticos.

A mutação de Rust solicita uma arena efetiva na partida. O renderer revela a
textura dessa arena em blocos até a troca; a aplicação sincroniza a identidade
efetiva, sua música e vida ambiente. Não usar uma imagem sobreposta persistente
como substituto do estado. Preferências de seleção de arena ficam preservadas.

## Consequências

`World::arena_override` mantém Sirius como arena efetiva até a recriação do World.
Nome, localização, textura, música e vida ambiente consultam esse estado. A nova
faixa é preparada pausada e inicia do começo quando o super termina.

Reset e replay restauram a arena base. A rotação do próximo round usa a seleção
original; a mutação não sobrescreve preferências do menu nem concede bônus de
combate. Luta, showcase e preview de super no Combat Lab compartilham os dados e
o relógio do World, incluindo captura, reações e retorno de Python.

Os detalhes de fases e os registros de verificação estão no
[plano e entrega](../24-reactions-and-transformations.md).
