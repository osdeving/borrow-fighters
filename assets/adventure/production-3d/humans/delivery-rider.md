# Entregador da Augusta em Blender

O entregador mantém a jaqueta oliva, a calça azul e o capacete escuro com viseira
azulada da cena. Corpo, rosto, olhos e mãos usam anatomia MakeHuman; a roupa é
ajustada à malha e o capacete foi modelado para este projeto. A scooter permanece
um modelo independente, com a mesma origem, orientação e escala do piloto.

O arquivo de execução é [delivery-rider.glb](../../models/delivery-rider.glb),
registrado em [humans.json](../../models/humans.json). O arquivo editável é
[delivery-rider.blend](delivery-rider.blend). Texturas estão incorporadas nos dois
arquivos; renders, créditos e relatórios desta pasta são fontes e evidências de
produção, sem dependência do jogo.

## Créditos e origem

- Anatomia, rig, pele, olhos, sobrancelhas e cílios: MakeHuman Community / MPFB2,
  ativos CC0. O código da ferramenta MPFB2 possui sua própria licença GPL-3.0;
  o pacote do jogo usa o modelo exportado e suas texturas. Versão e fontes da
  ferramenta constam na [proveniência geral](provenance.json).
- Jaqueta e calça: `male_casualsuit05`, do pacote oficial
  `makehuman_system_assets`. O [cabeçalho original](source/makehuman_system_assets/clothes/male_casualsuit05/male_casualsuit05.mhclo)
  registra a liberação CC0 em setembro de 2020 e os créditos Data Collection AB,
  Joel Palmius e Jonas Hauquier. O ajuste anatômico, as cores oliva/azul e a
  animação de condução foram preparados para Borrow Fighters.
- Botas: `toigo_ankle_boots_male`, pacote `shoes01`, autoria MRT e licença CC0 no
  [cabeçalho original](source/shoes01/clothes/toigo_ankle_boots_male/toigo_ankle_boots_male.mhclo).
- Capacete aberto, viseira curva, acabamento e ferragens: geometria original
  produzida para Borrow Fighters pelo script
  [delivery_rider.py](../../../../tools/blender/delivery_rider.py).

A [proveniência do entregador](delivery-rider.provenance.json) conserva URLs e
SHA-256 dos pacotes, do gerador usado, dos helpers, do Blender editável e do GLB.
Os arquivos originais selecionados permanecem em `source/`, inclusive os
cabeçalhos de licença.

## Escala e contato

Metros; Blender usa Z para cima e −Y para a frente, exportados como Y para cima e
+Z para a frente no glTF. A altura em pé, incluindo capacete e botas, é
1,782486 m. A pose `riding` usa a origem da scooter sem deslocamento acumulado.

| Apoio | Coordenada Blender em metros | Significado |
| --- | --- | --- |
| Quadril | `(0, 0.10, 0.95)` | Centro anatômico sobre o assento |
| Manoplas | `(±0.27, −0.44, 1.06)` | Contato da palma, não o centro do pulso |
| Pulsos | `(±0.27, −0.375, 1.09)` | Junta atrás e acima da manopla |
| Solas | `z = 0.4215` | Topo das ranhuras do apoio da scooter |
| Tornozelos | `(±0.15, −0.05, 0.509)` | Junta acima da sola da bota |

Os joelhos ficam lateralmente fora do escudo dianteiro. A cabeça compensa a
inclinação do tronco para que o rosto olhe ao longo da via. A ação `riding` tem
120 ticks a 60 Hz e retorna à pose inicial; `idle` também está presente.

## Verificação

O [relatório do GLB reimportado](delivery-rider-validation.json) verifica buffers
incorporados, uma malha/skin, 53 ossos, pesos e animações válidos. Nas amostras
0/30/60/90/120, as solas avaliadas ficam em aproximadamente 0,421246 m, as palmas
a aproximadamente 0,021 m dos centros das manoplas e os joelhos em `x ≈ ±0.343 m`.
O deslocamento da raiz e o erro de fechamento do ciclo são zero nessas medidas.

Os renders [frontal](previews/delivery-rider/rider-scooter-front.png),
[lateral](previews/delivery-rider/rider-scooter-side.png) e
[traseiro](previews/delivery-rider/rider-scooter-rear.png) foram revisados para
anatomia, roupa, assento, guidão e pés. Seus hashes e o escopo da revisão constam
em [delivery-rider-visual-review.json](delivery-rider-visual-review.json).
Esta evidência verifica o modelo isolado no Blender; a integração na rua é
validada nas capturas do capítulo.

```sh
blender --background --factory-startup --threads 6 --python-exit-code 1 \
  --python tools/blender/check_delivery_rider.py
```

A reprodução das fontes, quando necessária e com MPFB2/pacotes disponíveis,
usa `tools/blender/delivery_rider.py` com os mesmos argumentos do Blender.
O validador acima apenas lê o GLB e grava seu relatório; não reexporta os modelos.
