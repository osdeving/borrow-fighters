# Biografias — quadros pintados

Duke e Old C usam quatro ilustrações completas, com duas tomadas distintas por
personagem. A revisão prioriza o acabamento integrado de Ada/Python/C++:
perspectiva, luz, anatomia e oclusão são resolvidas na própria pintura. O usuário
autorizou este formato para as biografias; o mundo jogável continua modular.
Veja a [ADR 0030](../../../../docs/adr/0030-painted-biography-shots.md).

## Conteúdo e edição

[catalog.json](catalog.json) contém **somente quatro peças**, uma imagem por
peça. [scenes.json](scenes.json), versão 2, contém `illustration`, `caption`,
`duration_ticks` e a câmera de cada tomada. Não há composição de pessoas,
mesas ou carros por sprites, máscaras de braços ou textos presos a objetos
nessas biografias. O runtime carrega somente os quatro PNGs selecionados.

Para trocar uma tomada, altere a imagem e as dimensões do respectivo frame no
catálogo; cada frame tem apoio `[0, 0]` e largura de referência 1280. As quatro
imagens precisam ser distintas. O tempo e a legenda da outra tomada não mudam.
Para ajustar o movimento, edite `camera`: cada key contém `tick`, `position`
(alvo em coordenadas de referência) e `scale` (zoom). O horizonte permanece
nivelado. A interpolação smoothstep mantém a câmera suave nas extremidades.
A validação rejeita câmera que exponha uma borda vazia da ilustração.

As legendas continuam em [pt-BR.json](../../texts/pt-BR.json), chaves
`opening.duke.*` e `opening.c.*`. Nome e legenda ocupam a região inferior,
deixando rosto, mãos, teclado, pizza e reunião legíveis. A imagem não contém
legendas nem títulos de livros: os pequenos esquemas visuais são parte da arte.

Cada personagem ocupa 720 ticks (12 s), em duas tomadas de 360 ticks (6 s).
Duke ocupa 29–41 s; Old C, 41–53 s. A abertura mantém suas transições, trilha,
pausa e controles. **F5** recarrega o conjunto transacionalmente, conservando
relógios; referência, câmera, imagem ou texto inválidos preservam a versão
anterior. Mudanças em dados/imagens não exigem recompilar o binário compatível
com a versão 2. A composição anterior de sprites não é aceita por este modelo.

## Curadoria

| Tomada | Composição aprovada |
| --- | --- |
| Duke / Paulista | Limousine já estacionada paralela à guia, Duke saindo ao lado da porta e chauffeur; pés e rodas em contato com calçada/asfalto. Não há deslocamento artificial do carro sobre a pintura. |
| Duke / reunião | Mesa horizontal natural, Duke na cabeceira, seis executivos sentados e secretária anotando; escala, mãos, cadeiras e oclusão pintadas juntas. |
| Old C / oficina | Setup atual em sala rústica: monitor fino visto por trás, teclado sob as mãos, café, pizza, livros e estantes. |
| Old C / fundamentos | Outro lado de câmera e outra ação: rosto próximo, Old C estudando um livro e desenhando diagramas; mesma identidade, camisa e oficina. Não é zoom da primeira imagem. |

Movimentos suaves de enquadramento dão continuidade a esses quadros estáticos;
não há vídeo pré-renderizado usado pelo jogo. Detalhes internos de um quadro
são corrigidos somente na imagem daquela tomada. Para evidência do resultado
com texto, letterbox e transições reais, veja a
[revisão nativa](../../../../docs/evidence/cinematic-polish/biographies/README.md).

## Procedência

Criadas com `image_gen` integrado em 12/09/2026. Referências de identidade:
[Old C](../roster/c.png) e [Duke](../roster/duke.png). Referências de acabamento:
[Python](../python-teacher.png), [C++](../cpp-origin.png) e
[Ada](../../ada-prologue.png). Não foi usada a montagem antiga como referência
de geometria. O segundo Old C usa o primeiro quadro novo para continuidade;
a reunião usa o novo Duke da chegada para manter roupa e proporções.

Os quatro resultados têm 1672×941 pixels e foram copiados byte a byte, sem
regravação ou colagem. Prompts exatos estão ligados abaixo. Os originais
`exec-*.png` permanecem no diretório de geração da sessão.

| Arquivo | Resultado selecionado | Prompt |
| --- | --- | --- |
| [duke-paulista-painted.png](duke-paulista-painted.png) | `exec-1f91a1c5-d84a-4df1-9c6d-5847dcd1aad6.png` | [Chegada](prompts/duke-paulista-painted.txt) |
| [duke-boardroom-painted.png](duke-boardroom-painted.png) | `exec-b08515f9-5a84-4bed-ae75-b870dba2b276.png` | [Reunião](prompts/duke-boardroom-painted.txt) |
| [old-c-workshop-painted.png](old-c-workshop-painted.png) | `exec-c04979be-c15e-4e14-8623-13ee24680071.png` | [Oficina](prompts/old-c-workshop-painted.txt) |
| [old-c-foundations-painted.png](old-c-foundations-painted.png) | `exec-d7138f6b-69e5-431a-87ad-d13da803cb54.png` | [Fundamentos](prompts/old-c-foundations-painted.txt) |

SHA-256 dos PNGs selecionados:

- `duke-paulista-painted.png`: `18f321469d47a7e18226fdc7ec9dec7cf76fa458f09df4d50cb9cc0b0fb67e40`.
- `duke-boardroom-painted.png`: `afabaabf9a4f1c1782bc7d6f3e1820fc4b0007bf3d62892bb0ad39346c9370a8`.
- `old-c-workshop-painted.png`: `6561b03e0bee7559951bf9516541ac71b2c304c5faa42e09b72fddcd2db88384`.
- `old-c-foundations-painted.png`: `76d2a7f75f6e9b83e66c8743289216b68122c3a0299271d31162e9a7192803eb`.

Arte e prompts das tentativas anteriores permanecem preservados como fontes de
produção, fora do catálogo e do carregamento. Sua procedência está no
[registro arquivado](ARCHIVED-MODULAR.md), que não descreve o runtime atual.
