# Novo Go — arte final e verificação de 07/09/2026

O Go foi redesenhado a partir do [novo master adulto](reference/master.png), conforme a direção solicitada: pelagem azul ardósia, olhos pequenos, proporções atléticas, calça e faixa escuras, mãos enfaixadas e patas descalças. O conjunto final contém **20 ações, 60 quadros e os dez golpes**, já integrado ao carregamento normal do jogo. O salto menor encontrado na primeira revisão foi substituído por uma nova geração e recapturado no runtime.

As fontes, os prompts e as tentativas rejeitadas permanecem versionados por ação. Nenhuma pose do Go cartoon foi apresentada como arte nova. Os 263 arquivos anteriores foram preservados nos diretórios `go-cartoon-archive`, e os seis insumos do contrato de combate continuam com os mesmos hashes. O [snapshot](historical-preservation.json) e o [mapeamento do arquivo](archive-mapping.json) registram essa preservação.

## Arte e preparação

O [inventário](inventory.json) lista as 20 ações. Os registros de [movimento e guarda](batch-a-review.json), [golpes terrestres](batch-b-review.md) e [aéreos, especial e resultados](batch-c-review.md) detalham fontes e seleção. Cada ação possui recortes e pivôs explícitos, com uma escala anatômica uniforme para toda a folha. A referência de altura sólida do idle é 264 px; não se usa a altura de uma pose com pernas recolhidas para encolher ou ampliar seu corpo.

- **Walk:** quatro desenhos de deslocamento em guarda, com contato, transferência de peso, fechamento da pata traseira e avanço da pata dianteira. Mantém a ordem da guarda e os 240 ms do ciclo. As quatro chaves foram vistas avançando e recuando, nos dois sentidos, com retorno apoiado ao idle.
- **Crouch e guarda baixa:** crouch usa entrada de 80 ms e sustentação de 280 ms. A chave central foi rejeitada por um detalhe inventado no queixo. A sustentação mede aproximadamente 123 px; crouch_block mede 124/133/123 px, com a chave de absorção ligeiramente mais alta. As durações totais foram preservadas.
- **Contatos terrestres:** o jab v4 resolveu o alcance curto com cotovelo dobrado; o soco forte v2 e o throw v2 baixaram o contato. No atlas final, sweep v2 mantém a palma sólida em −6 px e a pata em +1 px em relação ao chão, com 7 px entre os apoios. Essa diferença de perspectiva foi aceita na revisão, sem alteração de arte ou combate; substitui a estimativa de aproximadamente 3 px de cada lado feita na fonte. V3 foi preservada e rejeitada pela perda de definição sem melhora suficiente do apoio.
- **Aéreos e anti-air:** o chute aéreo v2 redesenhou joelho e tornozelo; a pata cabe na caixa em altura, com uma extremidade de garra aproximadamente 1 px além da borda frontal. O soco aéreo inclina o tronco para o contato baixo. O uppercut alcança a caixa acima da cabeça pela extensão desenhada do braço e do corpo, mantendo apoio e escala anatômica.
- **Especial:** v2 já coloca a palma na origem baseline no primeiro quadro: (+97,92; −123,84) em relação à âncora, espelhada em x do outro lado. A emissão foi vista com `special_00` em tempo visual zero. O projétil cyan existente continua separado e com o mesmo hash; v1 foi rejeitada pela mão alta.
- **Entrada e resultados:** ajuste de faixa, reverência e guarda na entrada; comemoração contida na vitória; cansaço, joelho ao chão e posição sentada apoiada na derrota. Nenhuma pose dobrada foi ampliada para ocupar a altura de uma figura em pé.

O alpha foi extraído com os helpers genéricos autorizados, sem `--rust-contour`. Foram inspecionados fundos claros/escuros, mãos, patas e margens. A preparação apenas recorta, registra e redimensiona uniformemente; os pixels dos personagens não foram redesenhados por código.

## Correção final do salto

A revisão independente encontrou cabeça e tronco menores em jump v1. A [nova fonte v2](jump/source-v2.png) foi gerada com o master fixo e o idle como referências anatômicas. V3 permanece preservada, mas foi rejeitada pelo abdômen alongado. O [laudo do salto](jump/refinement-review-v2.md) registra a comparação e os ajustes.

V2 usa escala única **0,37**, pivôs locais (336,745), (294,723) e (236,784), e tempos **60/340/400 ms**. A faixa superior da cabeça mede 53–55 px, contra 55 px no idle. As patas ficam recolhidas nas primeiras poses; a última sola sólida fica 1 px acima da âncora virtual. O ajuste final de registro da descida não mudou a trajetória física.

A [comparação por quadro](../../candidates/go/review/refinement-2026-09-07/jump-v2-frame-comparison.json) confirmou que somente os três desenhos de jump mudaram. Os outros **57 quadros** mantêm pixels RGBA, dimensões, pivôs, limites e tempos idênticos. A primeira passagem completa continua preservada como evidência anterior, identificada por seus próprios hashes.

## Evidência final

O [World final](../../candidates/go/review/refinement-2026-09-07/world/capture-summary.json) concluiu **638 PNGs e 1.845 estados**. Cada jogador executou os dez golpes na sua orientação e voltou ao idle, apoiado e sem recuperação pendente, após cada golpe. A vida final 86×86 resulta do acerto normal dos projéteis, sem restauração de HP. O baseline de combate carregado permaneceu igual ao da passagem anterior.

Foram revisados os [dez contatos nos dois sentidos](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/all-ten-moves-both-facings.png), a [caminhada](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/walk_in.png), o [retorno à guarda](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/walk-return-idle.png), o [salto completo](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/jump-full-arc.png) e as transições para [soco aéreo](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/air_punch-transition.png) e [chute aéreo](../../candidates/go/review/refinement-2026-09-07/world/evidence-panels/air_kick-transition.png). A anatomia do salto agora acompanha a guarda e os aéreos. O pouso ocorre no tick 543, com fundo físico em y616 e retorno ao idle apoiado. As poses recolhem e estendem as pernas; a trajetória continua vindo do World.

O [Lab completo anterior](../../candidates/go/review/refinement-2026-09-07/lab-before-jump-refinement/capture-summary.json) produziu **269 PNGs de 19 contextos**. A caminhada é coberta pelo World. Após a correção, o [Lab final de jump](../../candidates/go/review/refinement-2026-09-07/lab/capture-summary.json) acrescentou **10 PNGs das três chaves novas**. A comparação dos 57 quadros inalterados mantém rastreável a evidência dos outros contextos. O HUD do Lab sobrepõe parte da cabeça nas poses altas; o World mostra a trajetória sem essa limitação.

O agente de revisão percorreu as 60 chaves no Sprite Studio e depois [recapturou as três chaves de jump v2](../../candidates/go/review/refinement-2026-09-07/studio/jump-v2/README.md) em zoom 1.00x, conferindo os pivôs 124,276 / 109,268 / 87,290 e as durações do manifesto. Os registros anteriores dos outros 19 clips permanecem preservados. A inspeção do Studio usa orientação direita; o World complementa os golpes nas duas orientações.

O agente principal repetiu a [partida normal Go × Go](../../candidates/go/review/refinement-2026-09-07/native-match-result.png) após a exportação final, removendo explicitamente a variável de opt-in. A partida terminou em **53×0**, com vitória e derrota apoiadas. Os [hashes e comando](../../candidates/go/review/refinement-2026-09-07/native-match-assets.json) identificam o mesmo atlas final.

O [vídeo final](../../candidates/go/review/refinement-2026-09-07/world/runtime-motion.mp4) contém exatamente **1.845 quadros, 60 fps e 30,750 s**, conferidos por ffprobe. Mantém o tempo dos ticks, sem interpolação ou pausa extra no final. A captura é amostrada nas mudanças de chave/fase e a cada três ticks ativos; conserva-se o PNG anterior até a próxima captura. Os sete painéis recortam pixels nativos sem redimensionar ou retocar a arte. O [índice](../../candidates/go/review/refinement-2026-09-07/world/evidence-index.json) registra imagens, ticks e âncoras; os PNGs completos e o vídeo preservam a translação no cenário.

## Reprodução e verificações

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/go/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/go/production.json
python3 tools/art/render_sprite_review.py assets/candidates/go/go-fighter.sprite.json
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_sprite_review -- go target/art/go-final-lab
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_motion_review -- target/art/go-final-world go --attacks --debug-boxes
python3 tools/art/encode_motion_review.py target/art/go-final-world --fps 60
cargo test --test sprite_candidates
```

Use diretórios novos para repetir as capturas. `ffplay runtime-motion.mp4` permite pausar com Espaço e avançar um quadro com `s`. Os dois testes de candidatos passaram após jump v2; o agente principal também executou `cargo test --all-targets --all-features`, com 234 testes aprovados. A [auditoria estruturada](finalization-audit.json) confirma 60 quadros visíveis com alpha real, durações totais preservadas, nenhum campo novo de combate e hashes de execução.

O fechamento dos estados de revisão foi reexportado e comparado byte a byte: manifesto e atlas continuaram idênticos aos capturados. O manifesto final tem SHA-256 `fc5560a0478019b1a45ef13cde21c34c177d12ff345ad373993913c5eab2e4da`; o atlas, `60f5e1b428e851146072fc7e8d5a672b479fb62bb6fbba91308519d4d8efd028`.

A revisão cobre os quadros e sequências inspecionados. Os ataques próximos do World não acertam na distância desse cenário controlado; o Lab mostra seus contatos e a partida normal complementa a luta. A animação usa chaves discretas, com pequenas variações de pelagem e tecido entre fontes. Nenhuma regra de combate, dano, colisão ou física foi alterada para acomodar os desenhos.
