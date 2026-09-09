# Reações articuladas do elenco — 2026-09-09

Esta rodada completa Rust, Duke, Old C e Go com **128 desenhos novos**: oito
sequências de quatro poses por personagem. Python e C++ continuam com seus
desenhos e contratos anteriores. O perfil e o relógio de contato já existentes
escolhem os novos clips; dano, hitstun, knockback, hitbox e hurtbox permanecem iguais.

## Fontes e exportação

A ferramenta integrada `image_gen.imagegen` gerou duas folhas de 16 poses por
personagem, usando a identidade/idle já aprovada no repositório.
A primeira saída veio RGB com tabuleiro pintado; uma edição pela mesma ferramenta
extraiu os personagens para RGBA real. A primeira extração da folha inferior de
Rust continuou RGB e foi descartada; a segunda é a fonte aceita.
Prompts e caminhos das gerações originais ficam nos diretórios individuais.

[export.py](export.py) recorta os desenhos, aplica uma escala uniforme por sequência
de quatro poses e os empacota em células 448×352, com pivô (224,310).
Não há rotação, deformação de corpo rígido, recoloração nem pintura local.
O alpha gerado permanece intacto dentro de cada recorte. A caixa recortada usa o
corpo visível e quatro pixels de margem; ruído quase transparente fora da caixa
não entra no atlas. As duas primeiras poses aéreas do C usam retângulos compostos:
as botas de uma e o cabelo da outra cruzam a divisão vertical sem se tocar.

A escala considera a anatomia do idle: 272 px em Rust, 256 em Duke, 275 em C e
269 em Go. Uma pose dobrada não é ampliada pela sua própria caixa de alpha.
As guardas baixas têm uma escala de sequência própria para manter a leitura
agachada. Cada `export-audit.json` registra retângulos, pivôs, escala, hashes de
fonte/atlas e pixels de cada desenho. `preserved-baseline.json` registra os
frames e clips preexistentes.

Reprodução, a partir da raiz, com Python e Pillow:

```sh
python3 assets/production/roster-reactions-2026-09-09/export.py --preview
python3 assets/production/roster-reactions-2026-09-09/export.py
```

`--preview` grava apenas arquivos de produção. Sem a opção, o comando acrescenta
os clips e grava os quatro atlas de runtime. A exportação é idempotente e recusa
alterar o baseline guardado.

## Contrato e revisão

Cada personagem acrescenta `reaction_head`, `reaction_body`, `reaction_low`,
`reaction_guard_high`, `reaction_guard_low`, `reaction_launch`,
`reaction_fall` e `reaction_rise`: quatro frames distintos, 70 ms, sem loop.
O primeiro desenho representa impacto; a amostragem ajusta os quatro desenhos
ao relógio real de contato. Queda termina no chão e recuperação termina em guarda.

Os painéis `review-game-scale.png` foram inspecionados com fundos claros e
escuros, no tamanho do jogo e com a linha de apoio. Rust preserva goggles,
moletom e ferramentas; Duke mantém cone, nariz e vapor sem rosto inventado;
C mantém idade, cabelo, roupa e livro; Go mantém a identidade adulta de
pelagem azul, olhos pequenos, calça e patas. O livro do C fica parcialmente
oculto pelo tronco na transição de um joelho da recuperação.

O cabelo do primeiro impacto alto do C foi conferido na fonte: alpha máximo 3
na primeira linha e 5 nas cinco primeiras linhas, com cabelo visível começando
abaixo. A borda não corta a silhueta. Os halos finos vistos na saída ampliada
não aparecem como tabuleiro ou franja colorida nos painéis em escala de jogo.

A [auditoria independente](../../../docs/evidence/roster-contact-reactions/asset-review.json)
confirma 128 desenhos distintos e preservação de 334 frames, 95 clips e PNGs antigos.
A [evidência de contato real](../../../docs/evidence/roster-contact-reactions/README.md)
complementa a revisão das fontes; o painel de sprites sozinho não comprova sincronia.

## Produções

- [Rust](../rust/reactions-own-2026-09-09/README.md)
- [Duke/Java](../duke/reactions-own-2026-09-09/README.md)
- [Old C](../c/reactions-own-2026-09-09/README.md)
- [Go](../go/reactions-own-2026-09-09/README.md)

Contrato anterior: [piloto Python/C++](../../../docs/25-python-cpp-contact-reactions.md)
e [ADR 0018](../../../docs/adr/0018-contact-reaction-profiles.md).
