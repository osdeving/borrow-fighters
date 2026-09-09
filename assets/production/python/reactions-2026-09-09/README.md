# Python — reações por contato

32 desenhos novos de reação produzidos com `imagegen` integrado em 9 de setembro
de 2026, com a [referência de identidade](../reference/master-existing.png).
O objetivo é distinguir o recebimento de cada tipo de pancada por poses próprias
de cabeça, tronco, braços e pernas. O primeiro quadro já mostra o impacto, sem
começar pela postura de idle. O conjunto preserva a mulher adulta de cabelo
preto, camisa branca, saia preta, saltos e acessório de serpente azul/amarela.

| Clip novo | Desenhos | Leitura |
|---|---:|---|
| `reaction_head` | 4 | Cabeça e ombros lançados para trás; mão na face durante a recuperação. |
| `reaction_body` | 4 | Tronco dobra no abdômen, braços protegem as costelas e corpo retoma equilíbrio. |
| `reaction_low` | 4 | Joelho cede, apoio fica instável e a personagem desce numa postura baixa. |
| `reaction_guard_high` | 4 | Antebraços recebem o impacto diante da face, com compressão e retorno. |
| `reaction_guard_low` | 4 | Guarda agachada comprime, protege joelho/tronco e recupera apoio. |
| `reaction_launch` | 4 | Recoil aéreo, pernas recolhem e estendem durante a descida. |
| `reaction_fall` | 4 | Impacto lateral no chão, acomodação e pose caída final. |
| `reaction_rise` | 4 | Apoio no cotovelo, mãos, joelho e retorno à postura em pé. |

As poses recebem um golpe vindo da direita e são espelhadas pelo runtime para
o outro lado. O chão usa uma queda lateral compacta, com cabeça à esquerda e pés
à direita. A última pose de `reaction_fall` permanece totalmente caída; levantar
pertence a um clip separado. Todas as poses de ar e chão mantêm a roupa completa.

## Produção e transparência

As quatro folhas originais e as extrações RGBA ficam lado a lado. Os prompts de
[impactos](prompt-hits.txt), [guardas](prompt-guards.txt), [voo](prompt-launch.txt)
e [chão/levantamento](prompt-ground-rise.txt) registram cada quadro pedido.
Voo e chão foram separados após duas tentativas de folha combinada sem saída
utilizável. O [registro das chamadas](generation-inputs.json) identifica fontes,
seleções e tentativas descartadas.

As folhas originais continham checkerboard pintado. A [extração pela ferramenta](prompt-alpha.txt)
produziu os quatro arquivos `*-alpha.png`; nenhum chroma key, remoção local de
matte ou redesenho por script foi aplicado. O [laudo de geração](generation.json)
registra dimensões, alpha e hashes. A imagem de chão conserva valores RGB de
fundo sob alpha zero; esses pixels são transparentes no renderer. Uma tentativa
adicional foi descartada após verificar que a primeira extração já era válida.

## Integração e pivôs

O [exportador](export.py) recorta poses explicitamente, aplica uma única escala
por folha e empacota o RGBA preservado em
[`python-reactions-atlas.png`](../../../candidates/python/python-reactions-atlas.png).
O atlas tem **1792 × 2816 pixels**, em quatro colunas e oito linhas. Cada célula
mede **448 × 352**, com pivô **(224, 310)**. A escala do manifesto continua 1,0.

As escalas de empacotamento são 0,68 para impactos, 0,55 para guardas, 0,40 para
voo e 0,57 para chão/levantamento. Isso aproxima as poses estendidas da altura
anterior de cerca de 270 pixels e conserva a menor altura das poses dobradas.
Pivôs de pé e apoio no chão são explícitos em coordenadas da folha original;
o voo conserva uma referência de apoio comum para que recolher as pernas não
desloque artificialmente o tronco. `trimmed_bounds` descreve alpha acima de 32
somente como metadata; os pixels exportados conservam também o alpha mais baixo.

Os oito clips opcionais foram acrescentados ao
[manifesto candidato](../../../candidates/python/python-fighter.sprite.json),
com quatro desenhos distintos e não repetidos por clip, 70 ms por desenho e
`loop: false`. O relógio de reação do combate governa a reprodução efetiva.
Os **88 frames e 25 clips anteriores**, a imagem original e a escala permanecem
idênticos ao [baseline preservado](preserved-baseline.json). Os frames novos não
declaram metadata `combat`; corpo físico, ataques e manifesto de colisão não
foram alterados por esta produção.

O [laudo de exportação](export-audit.json) registra cada recorte, pivô, escala e
hash. Para reproduzir o empacotamento a partir da raiz do repositório:

```sh
python3 assets/production/python/reactions-2026-09-09/export.py
```

As folhas e o atlas foram inspecionados para distinguir as silhuetas e a
progressão das poses. A validação de seleção, contato real, duas direções e
timing no jogo é registrada pela rodada de integração; esta nota de produção
não substitui essa verificação.
