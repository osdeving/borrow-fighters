# Arte da abertura da aventura

Recursos visuais preparados em 10 de setembro de 2026 para a abertura em forma
de trailer. São candidatos para composição no renderer; nomes, manchetes,
legendas, logo e animação tipográfica pertencem ao jogo e não foram incorporados
às imagens novas.

## Direção e referências

A [direção de arte](../../../docs/07-art-direction.md) prioriza leitura,
silhueta e um Brasil cotidiano com profundidade. A [lore](../../../docs/12-worldbuilding.md)
trata EPs como pessoas místicas, com diferenças morais e emoções.

As cenas novas de C++ e Python usam suas identidades existentes:
[C++](../../production/cpp/reference/identity.md) e
[Python](../../production/python/reference/identity.md). Os masters e as folhas
idle foram inspecionados antes das solicitações. O storyboard de
[Ada](../ada-prologue.png) serviu apenas como referência de pintura, luz e
profundidade; Ada não é personagem dessas cenas.

O novo pedido define C++ como mulher adulta que trabalhou como profissional
do sexo e desperta para o Linker, tornando-se heroína. Sua história é tratada
sem erotização nem juízo moral: a cena mostra vida urbana cotidiana, sem ato
sexual, nudez, clientes ou transações. O figurino narrativo preserva blusa branca,
calça preta, cabelo castanho/dourado, bolsa e detalhes dourados, com blusa fechada
até um decote cotidiano e cobrindo a cintura. O texto editável do trailer pode
explicitar a biografia; a imagem não tenta inferi-la por estereótipos.

Python aparece como professora universitária, adulta, ensinando humanos sobre
EPs. Sua demonstração mostra um modelo pedagógico de figuras humanoides e
ligações, não a criação deliberada de uma nova pessoa. O quadro usa formas
visuais; a informação escrita pertence ao renderer.

## Montagem do elenco

[roster.json](roster.json) descreve os PNGs, suas dimensões, os retângulos das
poses, a altura máxima, a origem e o SHA-256. Os arquivos abaixo foram apenas
copiados, mantendo bytes originais, e todos possuem alpha real.

| ID | PNG | Origem | Poses selecionadas |
|---|---|---|---|
| Rust | [roster/rust.png](roster/rust.png) | [Ações da aventura](../rust-actions.png) | 2 esperas |
| Duke/Java | [roster/duke.png](roster/duke.png) | [Idle existente](../../production/duke/idle/prepared.png) | 4 |
| Old C | [roster/c.png](roster/c.png) | [Idle existente](../../production/c/idle/prepared.png) | 3 |
| Python | [roster/python.png](roster/python.png) | [Idle existente](../../production/python/idle/prepared.png) | 4 |
| C++ | [roster/cpp.png](roster/cpp.png) | [Idle existente](../../production/cpp/idle/prepared.png) | 3 |
| Go | [roster/go.png](roster/go.png) | [Idle existente](../../production/go/idle/prepared.png) | 3 |

Os retângulos foram medidos por leitura do alpha (`alpha > 3`) com margem de
dois pixels, sem alterar ou recortar o PNG. Usar `rects` para cada pose; Rust
mantém outras ações no arquivo de origem, mas somente as duas esperas foram
selecionadas para a montagem. O vapor acima de Duke faz parte da figura e foi
preservado. Go usa o master adulto azul-ardósia da produção atual.

Esses recursos não importam manifests, dano, animações de combate ou código do
jogo de luta para o runtime da aventura. São imagens e coordenadas de desenho.

## Cenas novas

Os dípticos narrativos foram solicitados ao `image_gen` integrado, um pedido
por personagem. Não foi usado CLI, API externa ou edição de pixels por script.
Os PNGs selecionados são cópias byte a byte dos arquivos do gerador.

### C++ — cotidiano e despertar

- [cpp-origin.png](cpp-origin.png): RGB, 1182 × 1330.
- [cpp-origin-frames.json](cpp-origin-frames.json): dois source rects de
  1182 × 665, `y=0` e `y=665`.
- Fonte integrada: `exec-18c5242a-5ea4-4306-80cd-3211d9b28ed7.png`.
- SHA-256: `0c44b46b12151b5e484b3623044f6ba8ac958faf9158b3589eef47cff32b4e14`.
- [Prompt completo](prompts/cpp-origin.txt).

O quadro superior mostra a personagem junto de um ponto de ônibus à noite,
percebendo uma pequena luz na mão. O inferior mostra a mesma mulher em postura
de proteção, conduzindo fios de luz âmbar e fragmentos de matéria. Rosto,
cabelo, acessório dourado e bolsa permanecem reconhecíveis. A cintura está
coberta e o enquadramento enfatiza rosto, gesto e poder. Os dois painéis foram
inspecionados; não contêm descrição escrita da biografia nem ação sexual.

A região esquerda do primeiro quadro preserva espaço para a manchete ou a
frase da personagem no renderer. No segundo, a luz e a mão ocupam a direita;
evitar texto sobre o gesto. Os quadros têm relação próxima de 16:9 e podem
preencher uma tela 1280 × 720 preservando proporção.

### Python — ensino e compreensão

- [python-teacher.png](python-teacher.png): RGB, 1182 × 1330.
- [python-teacher-frames.json](python-teacher-frames.json): dois source rects
  de 1182 × 665, `y=0` e `y=665`.
- Fonte integrada: `exec-4a51994d-2e0c-4700-95ac-8e7c00bfac0e.png`.
- SHA-256: `0a9e933088235ce9b411fefaaae264e1997b02288ef5eacff1aa7a4eba8c76de`.
- [Prompt completo](prompts/python-teacher.txt).

O primeiro quadro mostra Python explicando três figuras humanoides no quadro
a uma turma de universitários adultos. O segundo mostra um modelo luminoso
estável ao lado de outro incompleto, enquanto ela conduz a explicação com as
mãos. Cabelo preto longo, blusa branca, saia preta, cinto e cobra azul/amarela
preservam sua identidade. O campus tem luz diurna e vegetação brasileira.

Os dois painéis foram inspecionados. O quadro contém figuras e ligações sem
texto falso; os personagens humanos e os modelos de EP têm leitura distinta.
O close inferior preserva rosto, gesto e modelo visual; posicionar legendas em
uma faixa própria para não cobrir essa informação. As imagens podem preencher
1280 × 720 com preservação de proporção.

## Verificação e limites

Os seis PNGs copiados e os dois dípticos gerados foram inspecionados. As cópias
mantêm bytes idênticos às fontes; os arquivos gerados mantêm bytes idênticos aos
outputs do `image_gen`. JSONs foram decodificados e seus retângulos conferidos
contra as dimensões dos arquivos. Os links locais deste documento foram validados.

Nenhum PNG foi recortado, ampliado, corrigido ou regravado por script. A leitura
de pixels serviu apenas para medir alpha, retângulos e hashes. A geração não
utilizou fallback CLI ou uma API externa. Os dois pedidos narrativos tiveram
uma saída selecionada cada, sem edição posterior.

As fontes geradas permanecem em
`/home/willams/.codex/generated_images/01a08a65-7252-7a23-b6bb-dd68c787eb2e/`,
além das cópias selecionadas neste diretório. Os prompts registram imagens de
referência por papel: identidade de personagem e estilo de pintura.

Esta verificação não aprova automaticamente o ritmo do trailer, enquadramento
após zoom, legibilidade de manchetes, mixagem, transições ou logo animado.
Esses elementos precisam ser revisados na abertura executada pela aplicação.
