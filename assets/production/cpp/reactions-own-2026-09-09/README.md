# C++ — reações próprias por contato

Produção de 9 de setembro de 2026 para tornar visível a pancada recebida pela
C++ contra Python. São **32 desenhos novos em oito clips**, gerados com o
`imagegen` integrado e acrescentados ao [manifesto candidato](../../../candidates/cpp/cpp-fighter.sprite.json).
O [atlas RGBA](../../../candidates/cpp/cpp-reactions-own-2026-09-09.png) mede
1536 × 3072 px; cada quadro ocupa uma célula de 384 × 384 px.

| Clip | Quatro desenhos articulados |
|---|---|
| `reaction_head` | Cabeça/chin levantam no impacto, arco para trás, mão na face, guarda dolorida. |
| `reaction_body` | Dobra imediata do abdômen, compressão maior, apoio na barriga e recuperação parcial. |
| `reaction_low` | Joelho cede, perna é recolhida, desequilíbrio baixo e apoio retomado. |
| `reaction_guard_high` | Antebraços recebem o golpe, ombros cedem e a guarda alta se recompõe. |
| `reaction_guard_low` | Proteção baixa comprime o corpo e recupera mantendo o agachamento. |
| `reaction_launch` | Recoil já no ar, subida arqueada, pernas recolhidas e início da descida. |
| `reaction_fall` | Corpo reclina, costas/ombro recebem o chão, rola para o lado e permanece caído. |
| `reaction_rise` | Apoio no antebraço, mãos/joelho, meia subida e guarda recuperada. |

A identidade vem do [master existente](../reference/master-existing.png) e da
[folha de idle revisada](../idle/source-v1.png): mulher adulta original, cabelo
castanho dourado ondulado com flor dourada, camisa branca amarrada, calça preta,
luvas e botas pretas/douradas e bolsa circular. Todas as poses têm orientação
3/4 para a direita. Os acertos chegam da direita, deslocando o gesto para a
esquerda. Mãos, pernas, pescoço e torso mudam de articulação; os clips de pancada
começam no impacto, sem inserir idle antes da reação.

As três folhas de desenho são `source-hits.png`, `source-guards.png` e
`source-air.png`. As primeiras saídas tinham xadrez pintado e não serviam para
composição. Chamadas separadas do próprio `imagegen` fizeram a extração de
fundo e entregaram `rgba-hits.png`, `rgba-guards.png` e `rgba-air.png` com alpha
real. Esses três RGBA são as fontes selecionadas. Prompts completos, hashes,
recortes e decisões estão em [generation.json](generation.json); os clips
preparados seguem o formato de produção em [production.json](production.json).

Para reproduzir o empacotamento, com Pillow e NumPy instalados:

```sh
python3 assets/production/cpp/reactions-own-2026-09-09/pack.py
```

[pack.py](pack.py) e [components.py](components.py) apenas identificam os desenhos
já gerados, recortam, aplicam escala uniforme por folha e empacotam. Todos os
pixels RGBA não transparentes de cada fonte são preservados antes da escala;
a atribuição às células separa bordas muito próximas sem desenhar partes novas.
Não há síntese de arte, remoção de fundo, recoloração ou limpeza de alpha local.

A medição de `trimmed_bounds` e dos pivôs no chão considera alpha acima de
16/255. Resíduos quase invisíveis vindos do gerador estendem-se além do corpo;
essa medição evita tratá-los como pé ou chão. O alpha da imagem continua intacto.
Os 28 quadros que precisam apoio têm `pivot.y` no limite inferior dessa área
visível; os quatro de lançamento conservam pivô próprio de voo.

Os 88 frames e 25 clips anteriores permanecem idênticos, na mesma ordem.
Campos principais e notas anteriores foram preservados; uma nota adicional
encaminha a procedência dos novos clips. Nenhum quadro novo tem metadata
`combat`, e esta produção não modifica ataques, hurtboxes ou corpo físico.

A revisão das folhas e dos oito previews preparados confirmou silhuetas
articuladas distintas, corpos inteiros, queda horizontal e subida em quatro
etapas. Há uma franja fina vermelha/dourada em partes do contorno da extração
RGBA. Três tentativas adicionais do `imagegen` perderam a transparência ou
reintroduziram a franja com alterações de cor; foram descartadas. A composição
preliminar na arena manteve boa leitura e contorno discreto. Essa limitação de
acabamento está registrada para revisão posterior, sem apresentá-la como VFX
intencional ou como arte final perfeita.
