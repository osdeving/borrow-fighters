# Arte do experimento de aventura — prólogo de Ada e manhã de Rust

Assets candidatos gerados em 10 de setembro de 2026 para a branch
`feature/rust-adventure-prologue`. Servem ao experimento jogável; ainda dependem
de revisão na aplicação e de aprovação humana de continuidade e fluidez.

## Ferramenta e referências

Todas as imagens novas e correções foram solicitadas ao `image_gen` integrado.
Não foi usado CLI, API externa, recorte por script, redimensionamento nem pintura
programática. Os PNGs selecionados são cópias com os bytes originais da ferramenta.
Os prompts completos ficam em [prompts](prompts/).

A identidade visual de Rust usa o [master existente](../production/rust/reference/master-existing.png),
derivado da [referência local fornecida](../references/sprinte-rust.md). Ada,
Assembly, a errática e os ambientes são explorações originais geradas a partir
do pedido da aventura e da [direção de arte](../../docs/07-art-direction.md).
Não foram usados assets externos de franquias. A mensagem misteriosa, os diálogos,
o texto e os efeitos temporizados pertencem ao compositor do jogo.

## Storyboard de Ada

- Arquivo: [ada-prologue.png](ada-prologue.png).
- Fonte do gerador: `exec-45f3c396-4cad-48ce-a8bc-43a1f31447c4.png`.
- PNG RGB, 2048 × 768; grade 3 colunas × 2 linhas, sem gutters.
- Source rect de cada célula: largura `2048 / 3`, altura `384`.
  As colunas possuem limites aproximados inteiros `[0, 683, 1365, 2048]`;
  a composição pode usar coordenadas fracionárias para os três quadros iguais.
- [Prompt](prompts/ada-prologue.txt).

| Índice | Coluna, linha | Conteúdo |
|---|---|---|
| 0 | 0, 0 | Ada escreve tranquilamente ao lado da máquina. |
| 1 | 1, 0 | A placa recebe um sinal abstrato misterioso. |
| 2 | 2, 0 | Ada toca a ligação entre matéria, luz e papel. |
| 3 | 0, 1 | Assembly surge com corpo incompleto. |
| 4 | 1, 1 | Ada contempla a mão com fascínio e preocupação. |
| 5 | 2, 1 | Máquina silenciosa e sinal distante à janela. |

Os seis quadros foram inspecionados visualmente. Ada mantém rosto, cabelo e roupa;
o primeiro quadro é humano e cotidiano. Assembly tem planos descontínuos ligados
por luz. Recomenda-se preencher a região cinematográfica 16:9 preservando proporção;
um zoom suave pode ocultar o pixel de divisão resultante da largura fracionária.

## Ambientes

- Arquivo: [adventure-environments.png](adventure-environments.png).
- Fonte do gerador: `exec-519d28dc-c516-4e1a-b03c-00fdedc9230d.png`.
- PNG RGB, 1182 × 1330; grade 1 coluna × 2 linhas, cada quadro 1182 × 665.
- [Prompt](prompts/adventure-environments.txt).

| Índice | Source rect (x, y, largura, altura) | Conteúdo |
|---|---|---|
| 0 | 0, 0, 1182, 665 | Quarto, cama esquerda, luz da janela e saída direita. |
| 1 | 0, 665, 1182, 665 | Rua brasileira, casas, vegetação e faixa inferior transitável. |

Ambos foram inspecionados sem personagens nem interface. No quarto, o colchão
ocupa aproximadamente `x=145..620, y=345..440` nas coordenadas da célula; alinhar
o corpo deitado à superfície próxima de `y=370`. O piso livre aparece abaixo
de `y=585`. Recomenda-se renderizar cada ambiente em 1280 × 720 com preservação
de proporção. A perspectiva e os objetos de fundo são pintura, sem colisão implícita.

## Rust: manhã e compaixão

- Arquivo: [rust-morning.png](rust-morning.png).
- Fonte do gerador: `exec-c8ec98d3-5a63-463c-b163-e2d23dcafec3.png`.
- PNG RGBA, 1448 × 1086; alpha verificado no arquivo: 1.106.174 pixels com
  alpha zero, 464.803 com alpha intermediário e 1.551 com alpha 255.
- [Prompt selecionado](prompts/rust-morning-final.txt).
- [Retângulos e ordem das poses](rust-morning-poses.json).

| Índice | Pose | Source rect (x, y, largura, altura) |
|---|---|---|
| 0 | Dorme de lado | 34, 158, 310, 148 |
| 1 | Abre os olhos | 396, 159, 313, 147 |
| 2 | Levanta o tronco | 756, 89, 322, 222 |
| 3 | Esfrega o olho | 1178, 63, 199, 248 |
| 4 | Senta à borda da cama | 98, 372, 181, 311 |
| 5 | Espreguiça | 430, 359, 226, 322 |
| 6 | Começa a levantar | 806, 397, 219, 258 |
| 7 | Em pé casual | 1184, 338, 175, 340 |
| 8 | Olhar compassivo para baixo | 100, 690, 174, 350 |
| 9 | Vira suavemente a cabeça | 465, 691, 164, 350 |
| 10 | Vira a cabeça para o outro lado | 815, 691, 174, 352 |
| 11 | Retorna ao olhar abaixado | 1181, 691, 176, 351 |

A referência de identidade foi mantida: cabelo castanho, goggles laranja,
moletom preto/laranja, calça cargo, tênis e acessórios. As poses são distintas,
com braços e tronco articulados. Os dois giros de cabeça são discretos e precisam
ser julgados na sequência real. O desenho da pose 6 usa uma mão baixa apoiada,
que deve aparecer como parte do movimento de levantar.

A grade solicitada foi 4 × 3, mas o gerador posicionou as linhas em alturas
ligeiramente diferentes. As faixas de inspeção foram `y=[0,330,687,1086]` e
`x=[0,362,724,1086,1448]`; usar os retângulos medidos acima evita cortar cabeças.
Para a cena em 1280 × 720, recomenda-se começar com altura em pé de 230 pixels,
escala constante `230 / 352` para todos os frames e origem no centro inferior
do retângulo. O corpo deitado precisa ser alinhado ao colchão pela cena.

## Entidade errática

- Arquivo: [erratic.png](erratic.png).
- Fonte do gerador: `exec-9408e82a-14bd-45cc-9483-b6177ebc4677.png`.
- PNG RGBA, 1774 × 887; alpha verificado no arquivo: 1.090.358 pixels com
  alpha zero, 482.927 com alpha intermediário e 253 com alpha 255.
- [Prompt selecionado](prompts/erratic.txt).
- [Retângulos e ordem das poses](erratic-poses.json).

| Índice | Pose | Source rect (x, y, largura, altura) |
|---|---|---|
| 0 | Espera instável | 103, 11, 250, 429 |
| 1 | Passo esquerdo | 514, 11, 290, 425 |
| 2 | Passo direito | 955, 11, 305, 426 |
| 3 | Antecipa o ataque | 1363, 71, 346, 369 |
| 4 | Investida com garra | 9, 523, 521, 312 |
| 5 | Recuo por impacto | 540, 466, 318, 383 |
| 6 | Cai apoiada em mão/joelho | 925, 594, 351, 261 |
| 7 | Exausta deitada | 1343, 686, 417, 139 |

O corpo é humanoide de matéria irregular escura e filamentos ciano, com cabeça
simples e garra legível, sem gore. A antecipação, investida, recuo, queda e
exaustão têm silhuetas distintas. Todos os desenhos apontam para a direita;
o compositor pode espelhar para o encontro com Rust.

A grade nominal é 4 × 2, mas a investida excede a largura de uma célula. Ela
ocupa a faixa inferior até `x=530`; a pose de recuo começa em `x=540`. Os
retângulos medidos isolam ambas sem cortar os dedos nem incluir outro corpo.
Recomenda-se começar com altura em pé de 260 pixels e escala constante
`260 / 429`; a investida usa largura maior mantendo a mesma escala.

## Rust: ações próprias da aventura

A revisão na aplicação identificou diferença de proporção entre a manhã e o
combate reutilizado. Este atlas adicional usa a manhã aprovada como única
referência de identidade e oferece as ações necessárias ao encontro local.

- Arquivo: [rust-actions.png](rust-actions.png).
- Fonte do gerador: `exec-38757214-8470-4f72-bec1-104bbe7527b2.png`.
- Referência: [rust-morning.png](rust-morning.png), mantendo rosto, cabelo,
  goggles, proporção cartoon, roupa, luvas e tênis.
- PNG RGBA, 1254 × 1254; alpha verificado: 1.072.826 pixels com alpha zero,
  497.643 intermediários e 2.047 com alpha 255.
- [Prompt selecionado](prompts/rust-actions-final.txt).
- [Retângulos e ordem das 16 poses](rust-actions-poses.json).

| Índices | Ação |
|---|---|
| 0–1 | Espera em guarda baixa, variação de respiração. |
| 2–5 | Quatro poses de caminhada para a direita. |
| 6–7 | Subida e descida do pulo. |
| 8–10 | Preparação, contato e recuperação do soco rápido. |
| 11–12 | Antecipação e contato do soco forte. |
| 13 | Defesa com os antebraços diante da cabeça e do peito. |
| 14 | Recuo por impacto. |
| 15 | Queda exausta de lado. |

A folha foi inspecionada com todas as poses de corpo inteiro, mãos fechadas
nos contatos e sem efeitos de impacto incorporados. Espera, caminhada, ataques
e defesa apontam para a direita. A derrota mostra o corpo deitado. As quatro
poses de caminhada variam apoio e postura; sua cadência final depende do runtime.

A grade nominal é 4 × 4, com faixas de inspeção `y=[0,329,641,940,1254]`.
A mão do soco forte passa alguns pixels da coluna nominal; o retângulo da pose
12 termina antes de `x=325` e a defesa seguinte só começa próxima de `x=394`.
Os retângulos medidos preservam os dois desenhos sem contaminação. Para combinar
com a manhã, começar com altura em pé de 230 pixels e escala constante baseada
na altura máxima dos retângulos; conferir a mudança de cena no renderer.

## Medição, rejeições e limites

Os retângulos foram medidos por leitura do alpha com limiar `alpha > 3` e
margem de dois pixels. O alpha muito baixo contém ruído espalhado fora dos
corpos; procurar bounds com `alpha > 0` tomaria esse ruído como parte da pose.
Nenhum pixel foi alterado, recortado, reescalado ou regravado por essa medição.
Os JSONs guardam somente coordenadas e etiquetas para o renderer.

As transparências foram conferidas numericamente e as folhas completas foram
inspecionadas visualmente. O alpha intermediário inclui os corpos com valores
próximos de 250, além de bordas suaves; não significa que os corpos estejam
quase invisíveis. A revisão da arte isolada não aprova automaticamente pivôs,
ritmo, continuidade entre poses ou composição com cama e chão na aplicação.

| Tentativa não selecionada | Resultado e motivo |
|---|---|
| `exec-e65c1852-287a-49b2-8ed2-b52cad7e2a49.png` | Rust inicial: poses legíveis, mas RGB com xadrez pintado. [Prompt](prompts/rust-morning.txt). |
| `exec-537edefe-5749-4eac-b423-8bb8034b22dd.png` | Remoção de fundo de Rust: repetiu RGB com xadrez. [Prompt](prompts/rust-morning-alpha-retry.txt). |
| `exec-66e1c36d-fe94-4f1a-b85e-6d6b2bc48b6f.png` | Segunda tentativa de alpha de Rust: repetiu xadrez e redesenhou parte das poses. [Prompt](prompts/rust-morning-alpha-retry-2.txt). |
| `exec-bb1709e9-b124-449c-8a97-8786e51e2589.png` | Ajuste da grade da errática: perdeu alpha e continuou excedendo a célula da investida. Preservado o original RGBA com metadados explícitos. [Prompt](prompts/erratic-grid-retry.txt). |
| `exec-5e5dcc03-9ba0-4ec8-ae8e-ebc4b33997bc.png` | Ações de Rust: desenho coeso, mas PNG RGB com xadrez pintado. Nova geração como recortes transparentes produziu o arquivo selecionado. [Prompt](prompts/rust-actions-initial.txt). |

As fontes da ferramenta permanecem no diretório de geração da sessão
`/home/willams/.codex/generated_images/01a08a65-7252-7a23-b6bb-dd68c787eb2e/`.
Os cinco arquivos selecionados também estão neste diretório do repositório.

## Integridade dos PNGs selecionados

| Arquivo | SHA-256 |
|---|---|
| `ada-prologue.png` | `22bc7fd7e1c97b0350caf9b8262432cd4e470edcfe101835dd86a58f379ede22` |
| `adventure-environments.png` | `5f7352b4123bbd5c5b592c2168bcc54e2d9e77a8906aa7f5339995b1c70ed31e` |
| `rust-morning.png` | `59c86cc14d4673be5ee99e8e7f707e40ca44779d06227ba5fabb822d2a60c12b` |
| `erratic.png` | `eeb61a73f64dad3440209bf3f48835554a6cb130fcbfbc1a38d03a8e4309604e` |
| `rust-actions.png` | `f0a7af2a9462263963bb3a24126e4c1e3728dcd929bcb57fa21a9c2bd13e864f` |
