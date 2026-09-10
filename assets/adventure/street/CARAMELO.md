# Caramelo da rua

Candidato da aventura gerado em 10 de setembro de 2026 com **image_gen
embutido**. [caramelo.png](caramelo.png) contém oito poses do mesmo cão
caramelo adulto, de pelo curto, focinho claro e orelhas assimétricas.
Todas apontam para a direita; o runtime pode espelhá-las pelo pivô.

| Índice | Pose |
|---|---|
| 0 | Parado, em pé |
| 1 | Sentado |
| 2 | Alerta |
| 3 | Farejando suavemente |
| 4 | Corrida: compressão e recolhimento |
| 5 | Corrida: suspensão estendida |
| 6 | Corrida: contato dianteiro |
| 7 | Corrida: passagem e impulso |

O atlas mede **1024 × 1536**, organizado em duas colunas e quatro linhas.
[caramelo.json](caramelo.json) fornece recortes explícitos, pivôs, medidas
de alpha e ordem sugerida da corrida. Os pivôs seguem o centro do tronco
na horizontal e o chão virtual na vertical. A suspensão conserva o espaço
sob as patas; não alinhar todas as poses pela borda inferior da silhueta.
Usar escala única calculada pela altura de referência de **341 pixels**
do cão parado, preservando-a ao sentar, farejar e correr. O desenho mantém
proporções naturais, com variação ilustrada entre poses; não representa
animação vetorial nem um rig de dimensões matematicamente idênticas.

O PNG selecionado possui alpha verdadeiro: **1.103.253 pixels vazios**,
469.611 com alpha intermediário e nenhum totalmente opaco; alpha máximo
254. O arquivo foi copiado byte por byte da geração integrada. Python foi
usado somente para ler pixels, medir recortes e escrever metadados. Não
houve retoque, recorte raster, remoção de fundo ou repintura por script.

O [prompt completo](prompts/caramelo.txt) registra anatomia, poses,
identidade e transparência. Não foram enviados arquivos de referência à
geração; o atlas local [props.png](props.png) foi apenas inspecionado para
orientar a descrição do estilo. Não foram usadas fotografias, samples,
marcas ou referências externas.

Fonte selecionada: `exec-b952f766-f333-484f-88b8-626827f01a8e.png`, preservada
em `/home/willams/.codex/generated_images/01a08bdb-6ada-71c1-beaa-e724cd3b8872/`.
SHA-256: `9fa17dcccab61b78ea6414aadcbedd41ef6aeb81d9637b8d83db8b570f48e8d3`.
Escopo na [entrega 32](../../../docs/32-cinematic-neighbourhood-arrival.md).

Verificação: oito silhuetas completas e separadas, direção direita,
dimensões RGBA, alpha vazio, recortes e pivôs dentro dos limites e SHA-256
registrado no JSON. A leitura e a escala finais devem ser conferidas na
cena em movimento.
