# C++ — Undefined Behavior: Footgun

Dois atlas novos, cada um com seis poses e 1536 × 1024 pixels RGBA, produzidos
com `imagegen` integrado em 9 de setembro de 2026. A
[referência existente](../../cpp/reference/master-existing.png) fixa a mulher
adulta, cabelo castanho/dourado, camisa branca, calça preta, luvas, botas com
ornamentos dourados e bolsa circular preta/dourada. A bolsa permanece em todas
as poses, incluindo os saltos e chutes.

| Índice | [footgun-comedy.png](footgun-comedy.png) | [footgun-barrage.png](footgun-barrage.png) |
| --- | --- | --- |
| 0 | Bazuca grande apontada ao próprio pé. | Corrida inclinada para frente. |
| 1 | Disparo no próprio pé e reação surpresa. | Soco estendido. |
| 2 | Segurar o pé levantado. | Outra chave de soco com torso girado. |
| 3 | Segunda chave de pulo sobre um pé. | Chute frontal. |
| 4 | Raiva e punhos cerrados. | Chute alto. |
| 5 | Arranque de corrida. | Golpe final ascendente. |

O [renderer](../../../../src/engine/render/authored_actors.rs) usa as poses novas
durante preparação, disparo, pulos, raiva, perseguição, rajada e retorno. O
deslocamento até o alvo é fornecido pelo `World`; oito contatos da rajada
alternam socos e chutes antes do finalizador. O erro no pé não causa ferimento
gráfico nem dano ao próprio personagem.

Os recortes e pivôs são explícitos; os atlas não são divididos em células iguais.
Na folha de comédia, o fim de uma bota e o cabelo da linha seguinte compartilham
três linhas de pixels. O renderer desenha essas regiões em partes separadas,
preservando a bota e evitando pixels da pose vizinha sem editar o arquivo.

As folhas iniciais vieram com checkerboard pintado. Extrações de fundo feitas
pela própria ferramenta produziram alpha verdadeiro; a folha de comédia exigiu
uma segunda tentativa com pedido curto. O atlas final de comédia tem 69,06% dos
pixels transparentes; o de rajada, 66,22%. Os PNGs selecionados foram copiados
intactos, sem pós-processamento de raster.

Prompts: [comédia](prompt-comedy.txt), [rajada](prompt-barrage.txt),
[extração inicial](../background-extraction-prompt.txt),
[segunda tentativa](prompt-background-retry.txt). Fontes da ferramenta,
componentes de alpha e hashes: [generation.json](generation.json).
Contrato: [rodada23](../../../../docs/23-authored-super-sequences.md).
