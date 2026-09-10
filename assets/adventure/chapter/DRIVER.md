# Motorista — duas poses para a conversa

Arte candidata do [capítulo Depois do silêncio](../../../docs/33-after-the-silence.md):
um homem brasileiro adulto, cerca de 45 anos, de camisa azul-clara, calça carvão
e sapatos marrons, um pouco abalado e sem ferimentos. A atuação permite a Rust
verificar seu estado depois do acidente. A revisão no cenário cabe à integração.

O [atlas](driver.png) contém duas poses completas para a esquerda. A primeira
fala com a palma aberta; a segunda agradece com a mão no peito e um leve aceno
de cabeça. Corpo, roupa, direção e escala permanecem coerentes entre elas.

| Índice | Pose | Recorte `[x, y, largura, altura]` | Apoio no recorte |
|---|---|---|---|
| 0 | `driver-speaking` | `[78, 74, 450, 1113]` | `[326, 1111]` |
| 1 | `driver-grateful` | `[784, 83, 301, 1104]` | `[180, 1102]` |

Os [metadados](driver.json) guardam os recortes, apoios, alturas e procedência.
Os apoios seguem o centro da pelve e o chão comum `y = 1185` no atlas. Aplicar
a mesma escala nas duas poses, usando **1108 px** de altura de referência.
Uma altura de 170–200 px no mundo corresponde a escala de aproximadamente
0,1534–0,1805; a integração decide a profundidade e a altura junto a Rust.
A leve redução de altura da cabeça na segunda pose pertence ao gesto.

## Origem e transparência

- Geração: ferramenta integrada `image_gen`, em 10/09/2026; imagem nova a
  partir do [prompt completo](prompts/driver.txt), sem CLI/API externo.
- Referências do próprio projeto, apenas inspecionadas visualmente:
  [moradores](../street/neighbours.png) e [lojista](../street/shopkeeper.png),
  para contorno, materiais pintados e proporções. Nenhuma imagem foi enviada
  como alvo de edição; o motorista é uma identidade original.
- Fonte selecionada:
  `/home/willams/.codex/generated_images/01a08c38-e933-72d1-a884-aeb4f0c99241/exec-086263ff-ba7e-4100-82e6-18f0efadd84c.png`.
- PNG entregue: **1254 × 1254**, modo **RGBA**, **878782 bytes**. A ferramenta
  retornou essa resolução para o pedido de 1024 × 1024; não houve redimensionamento.
- SHA-256 do PNG:
  `10aef02391b50458663be7d23c6cde79d5bb43bf63fd522e4b8a6d5e70e5a38e`.
- Alpha: **1116630** pixels zero, **454659** intermediários e **1227** opacos.
  O PNG e seu alpha são idênticos aos da fonte gerada, byte a byte.

Python/Pillow foi usado somente para ler dimensões/alpha, copiar o arquivo
sem alteração e escrever metadados. Não houve remoção de fundo, recorte,
repintura, composição ou regravação da imagem. Os bounds usam `alpha > 3`
com margem de 2 px. Os **11824** pixels de alpha 1–3 da fonte continuam
preservados; o intervalo entre as silhuetas visíveis supera 250 px e contém
apenas alpha 0–1. Não há fundo pintado, carro, texto ou sombra de chão.

A inspeção estática confirmou direção, poses completas, identidade, margens
e apoios. O teste nativo de leitura, escala e continuidade da conversa pertence
à integração do capítulo, conforme a [ADR 0027](../../../docs/adr/0027-chapter-spatial-direction.md).
