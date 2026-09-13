# Humanos 3D de Augusta

Fontes editáveis, mapas originais CC0 e receitas dos humanos do capítulo. A geometria vem da anatomia humana do MakeHuman/MPFB; roupas são malhas ajustadas ao corpo, e o rig tem 53 ossos com dedos articulados. Os GLBs usados pelo jogo ficam em [models](../../models), separados das fontes Blender.

A direção de cor, cabelo e figurino segue os desenhos já usados no capítulo: [C++](../../actors/cpp/source/model-turnaround.png), [Julia/cafetão/segurança](../../actors/julia/source/npcs.png), [figurinos A](../../chapters/cpp-augusta/sprites/nightlife-cast-a.png) e [figurinos B](../../chapters/cpp-augusta/sprites/nightlife-cast-b.png). As folhas de perfil existentes também servem de referência de silhueta lateral. A criatura EP continua com sua arte anterior.

## Autoria e reprodução

Consulte [provenance.json](provenance.json) para URLs, revisão do MPFB, hashes das fontes, licenças e adaptações. Os mapas recebidos não foram repintados. Cor e acabamento usam fatores de material, mapas normais originais e geometria Blender. Os créditos da ferramenta GPL e dos dados CC0 são registrados separadamente.

A instalação e os comandos gerais estão em [tools/blender/README.md](../../../../tools/blender/README.md). Exemplos, a partir da raiz do repositório:

```sh
blender --background --factory-startup --threads 6 --python tools/blender/humans.py -- --review /tmp/cpp-studio
blender --background --factory-startup --threads 6 --python tools/blender/human_cast.py -- --actor julia --review /tmp/augusta-human-studio
blender --background --factory-startup --threads 6 --python tools/blender/human_cast.py -- --actor broker --review /tmp/augusta-human-studio
blender --background --factory-startup --threads 6 --python tools/blender/crowd_humans.py -- --wardrobe plum-dress --review /tmp/augusta-crowd-studio
```

As demais opções de `--wardrobe` são `leather`, `amber-jacket`, `denim`, `teal-dress`, `white-shirt`, `red-blouse` e `long-coat`. A opção `--preview` desse gerador permite examinar a roupa antes de exportar o GLB. Segurança e entregador têm geradores próprios, [security.py](../../../../tools/blender/security.py) e [delivery_rider.py](../../../../tools/blender/delivery_rider.py), e proveniência complementar neste diretório.

Cada geração preserva o `.blend` comprimido, com objetos separados, materiais, texturas empacotadas e ações. A exportação reúne as superfícies apenas no GLB. Os `*-model.json` contêm a altura física medida, a taxa de animação e os nomes das ações; o runtime lê a coleção publicada em [humans.json](../../models/humans.json). Alterações de animação podem ser reaplicadas ao `.blend` sem reconstruir a roupa usando `export_actions.py`.

## Contrato e revisão

As unidades são metros: Blender Z para cima/frente −Y; glTF Y para cima/frente +Z; os pés têm origem no chão. O runtime preserva a altura e as trajetórias já definidas na cena. O cafetão usa o braço direito e Julia o esquerdo para o contato, com ações `restrain`, `restrained` e `pull`.

A pele dos figurantes usa a topologia anatômica original com normais suaves; roupas mantêm uma subdivisão de acabamento e cabelo não recebe subdivisão adicional. Isso reduz o custo dos 22 habitantes sem trocar humanos por formas geométricas simplificadas.

O validador `tools/blender/validate_humans.py` confere pesos, ossos, geometria e ações dos GLBs efetivamente exportados. A leitura visual exige frames: `review_human.py` renderiza ciclos na duração correta a 24 fps; o laboratório e a captura de Augusta mostram as mesmas malhas na iluminação do jogo. Verificar mãos, contato, cotovelos, joelhos, roupas e chão em ambos os sentidos antes de publicar a coleção.
