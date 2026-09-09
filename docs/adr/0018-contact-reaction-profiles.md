# ADR 0018 — Perfis e relógios visuais por contato

## Status

Aceito para o piloto Python/C++ solicitado após o playtest da rodada 24.

## Contexto

Na rajada, contatos a cada dez frames reiniciavam uma reação ajustada a 24 frames
de stun. Isso repetia o começo pouco expressivo do atlas. Testar a sequência
inteira não detectava a ausência de uma resposta legível em cada pancada.

## Decisão

Fighter expõe um retrato opcional do contato com perfil, idade e duração visual.
Python e C++ usam esse retrato nesta rodada. O World escolhe o perfil quando o
golpe realmente acerta ou é defendido; a apresentação seleciona o atlas e ajusta
seus desenhos à janela daquele contato. Contatos da rajada e poses ofensivas
consultam a mesma agenda, mantendo a física e o dano existentes.

Clips `reaction_*` são opcionais nos manifestos e não substituem a metadata de
combate. A primeira pose é impacto; as demais completam recoil e recuperação.
Fallback conserva a apresentação anterior quando não houver atlas próprio.

## Consequências

- Perfil e tempo pertencem ao combate, enquanto imagem, escala e pivô pertencem
  à apresentação. Duração da reação visual pode ser menor que o stun físico.
- Voo, queda, recuperação, guarda e KO precisam selecionar perfis coerentes.
- A verificação mede cada janela de contato e inspeciona a dupla em movimento.
- Estender a arte aos demais personagens exige outra rodada explícita; este
  piloto não os declara visualmente aprovados.
