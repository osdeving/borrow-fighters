# C++ — produção de sequências próprias

Escopo: 20 ações visuais existentes, sem modificar estados, hitboxes, hurtboxes, duração dos golpes, velocidade ou física. O projétil existente permanece separado. Esta produção usa o piloto Rust como processo técnico; as imagens são candidatas até revisão artística final.

## Referências e identidade

- [Master original preservado](reference/master-existing.png), descrito em [identity.md](reference/identity.md).
- [Idle gerado](idle/source-v1.png), referência fixa adicional nas demais chamadas.
- [Contrato de contatos](../contact-contract.json), seção `cpp`.

Preservar mulher adulta de proporções humanas esguias, cabelo longo castanho/dourado ondulado, ornamento dourado, pele quente, camisa branca curta amarrada, calça preta, luvas e botas pretas/douradas. A bolsa redonda preta/dourada e sua alça continuam em todas as poses. No especial, a mesma bolsa é projetada à frente pela alça contínua para coincidir com a origem já existente do projétil.

## Sequências

Uma chamada real de imagegen por folha e uma ação por folha. Idle, salto, agachamento, defesas, hit, nove ataques próximos, especial, spawn, vitória e derrota usam três desenhos por sequência; caminhada usa quatro. Os ataques têm preparação, contato e recuperação nos limites de ticks do contrato. O salto usa takeoff/rise/fall por 60/340/400ms; a física fornece a trajetória. Derrota termina ajoelhada e apoiada, mantendo a escala anatômica.

Fontes RGB e prompts versionados são preservados. Chroma magenta uniforme foi solicitado em cada chamada; a remoção local de fundo e a exportação foram autorizadas pelo usuário. Usa-se o helper genérico de magenta, sem o tratamento específico do Rust. Componentes desconectados menores que 32 pixels são removidos apenas quando registrados no relatório de alpha.

Cada `action.json` registra retângulos, pivôs, duração e a escala sugerida da fonte. O [review-plan.json](review-plan.json) contém uma escala uniforme por ação, visando idle de 268px; nenhuma pose recebe escala individual. `prepare_reviewed_actions.py` transforma os recortes em pixels de exibição; o exporter gera um atlas separado sob `assets/candidates/cpp`. As prévias mostram os dois sentidos e fundos claro/escuro.

## Divisão da produção

O agente coverage_inventory produz as 11 ações de movimento, defesa, reação e apresentação e consolida a revisão/exportação. O agente asset_export produz punch_light, punch_heavy, kick, sweep, overhead e throw. O agente animation_runtime produz anti_air, air_punch e air_kick. Todos usam as mesmas referências permanentes e o mesmo contrato; nenhuma geração usa a imagem de outra personagem como identidade.

Capturas GPU completas devem ficar sob `target/art`; somente relatório e amostras selecionadas ficam junto do candidato. A revisão final local registra cobertura e limitações em `review.md`.
