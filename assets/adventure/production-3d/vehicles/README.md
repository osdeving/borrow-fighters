# Veículos 3D da Augusta

Modelos originais feitos no Blender 4.5.4 LTS a partir das formas e cores dos
veículos que já circulavam na Augusta. O gerador não usa modelos, texturas ou
marcas de terceiros. Esta tradução visual não modifica ruas, faixas, rotas,
velocidade, sequência narrativa ou câmeras.

- [Gerador editável](../../../../tools/blender/vehicles.py).
- [Catálogo de runtime](../../models/vehicles.json).
- [Proveniência, dimensões, sockets e hashes](vehicles.provenance.json).
- [Validação dos GLBs](validation.json).
- [Avaliação da animação após reimportar os GLBs no Blender](rig-validation.json).
- [Revisão visual dos nove ângulos e hashes das imagens](visual-review.json).

Os arquivos `.blend` incluem o modelo articulado e um estúdio isolado para
revisão. O GLB exporta somente o modelo, sua armadura e os sockets: o piso,
as luzes e a câmera de estúdio não fazem parte da cena do jogo.

| Modelo | Fonte Blender | GLB de runtime | Revisão isolada |
| --- | --- | --- | --- |
| Hatch azul, também usado com pintura bordô | [hatch.blend](hatch.blend) | [hatch.glb](../../models/vehicles/hatch.glb) | [Frente](previews/hatch-front.png), [lateral](previews/hatch-side.png), [traseira](previews/hatch-rear.png) |
| Táxi creme com sinal âmbar e duas faixas nas portas | [taxi.blend](taxi.blend) | [taxi.glb](../../models/vehicles/taxi.glb) | [Frente](previews/taxi-front.png), [lateral](previews/taxi-side.png), [traseira](previews/taxi-rear.png) |
| Scooter ferrugem com caixa bordô | [delivery-scooter.blend](delivery-scooter.blend) | [delivery-scooter.glb](../../models/vehicles/delivery-scooter.glb) | [Frente](previews/delivery-scooter-front.png), [lateral](previews/delivery-scooter-side.png), [traseira](previews/delivery-scooter-rear.png) |

## Contrato de integração

Metros; Blender usa `Z` para cima e `-Y` para frente. O glTF usa `+Y` para cima
e `+Z` para frente. A origem permanece no chão e no centro longitudinal do
veículo, com pneus tangentes ao plano do chão. A escala do jogo deve usar o
comprimento existente do carro dividido por `length_m`, sem modificar a rota.

`drive` gira cada roda uma volta em um segundo: quadros `0..60`, a 60 Hz,
com extremos equivalentes. A raiz e a carroceria não se deslocam. Cada vértice
tem exatamente um osso de influência, evitando deformação de pneus e aros.
A fase deve acompanhar `wheel_angle / (2π)` do estado de trânsito existente.

O campo `paint_material` é o índice de material do **Raylib**, que acrescenta
um material padrão antes dos materiais do glTF; portanto, corresponde ao
índice glTF de `body_paint` mais um. Somente esse material recebe a variante
azul/bordô do hatch. Os fatores de cor dos materiais são lineares, conforme
glTF; a iluminação do runtime deve preservar essa interpretação.

Carros incluem bancos, encostos, apoios de cabeça, costuras, console,
volante, retrovisores, limpadores, janelas, faróis, lanternas, rodas e pneus.
A scooter inclui transmissão, suspensão, freios, guidão, espelhos,
instrumentos, banco, piso, escapamento, caixa e suas ferragens. As placas
permanecem vazias, sem inventar números ou marcas.

## Ocupantes separados

Os GLBs de veículos não contêm pessoas. O entregador e eventuais motoristas
usam as malhas humanas anatômicas da produção 3D. Os sockets de mãos, pés e
quadril estão exportados como nós e também registrados na proveniência,
em coordenadas Blender e glTF. Não existe um torso geométrico provisório
escondido nestes modelos.

A scooter informa `height_m = 1.70` para o envelope de scooter **com piloto**;
a malha mecânica, sozinha, é mais baixa. Isso preserva a escala de 170 pixels
do conjunto existente quando o piloto separado for integrado.

## Regeneração e verificação

Na raiz do repositório:

```sh
blender --background --factory-startup --threads 6 --python-exit-code 1 --python tools/blender/vehicles.py
python3 tools/blender/validate_vehicles.py
blender --background --factory-startup --threads 6 --python-exit-code 1 --python tools/blender/check_vehicle_rigs.py
```

`--only hatch`, `--only taxi` e `--only delivery-scooter` limitam a geração a
um veículo. `--no-render` exporta sem gerar as imagens de revisão. O validador
confere os buffers, índices, normais, pesos rígidos, contato com o chão,
materiais, duração e fechamento do ciclo, ausência de movimento da raiz e
ausência de imagens externas ou câmeras no GLB. Ele registra o SHA-256 do
arquivo efetivamente inspecionado.

A segunda verificação reimporta os GLBs no Blender e avalia a malha nos quadros
0, 15 e 60: a carroceria deve ficar parada; os vértices das rodas devem girar
em torno do eixo sem deslocamento axial e voltar às posições iniciais. Isso
confere a armadura e as matrizes de vínculo exportadas, além das curvas do arquivo.
