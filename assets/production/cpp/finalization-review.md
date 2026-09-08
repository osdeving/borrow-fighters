# C++ — acabamento e verificação de 07/09/2026

Os três defeitos priorizados foram refinados com novas gerações: caminhada com alternância de pernas, guarda baixa na altura do agachamento e bota do chute aéreo junto à caixa ativa. O agente principal revisou os quadros e aceitou esses refinamentos. O candidato mantém **20 ações, 61 quadros e os dez golpes**, com os mesmos tempos e regras de combate.

## Arte selecionada

| Ação | Resultado | Fonte e registro |
| --- | --- | --- |
| walk | Quatro chaves em 400ms: contato com perna próxima dourada, passagem sobre a perna distante, contato oposto com perna lisa e passagem sobre a perna próxima. Contorno sólido dos apoios em 0px da âncora nos quatro quadros; altura 267–269px, consistente com idle 267–268px. | [Folha selecionada v8](walk/assembled-selected-v8.png), [metadados](walk/action.json), [montagem reproduzível](walk/assemble-selected-v8.py). |
| crouch_block | Três guardas com altura sólida 118/114/114px, contra crouch 118/115/113px. O pé de apoio permanece na âncora. A defesa anterior media aproximadamente 92–95px e produzia uma queda desnecessária na transição. | [Fonte v6](crouch_block/source-v6.png), [alpha](crouch_block/keyed-v6.png), [metadados](crouch_block/action.json). |
| air_kick | Redesenho localizado do tornozelo/bota em dorsiflexão. Contato central em aproximadamente (+160,8; −72,2), dentro da caixa original. Extremo do salto fica a cerca de 2px da borda inferior, contra aproximadamente 15px antes. Cabeça, cintura, trajetória física e pivôs do ataque foram preservados. | [Fonte v3](air_kick/source-v3.png), [alpha](air_kick/keyed-v3.png), [metadados](air_kick/action.json). |

A [comparação nativa antes/depois](../../candidates/cpp/review/refinement-2026-09-07/before-after-native.png) mostra a correção do contato e da altura da guarda. O [painel das três chaves de crouch e guarda](../../candidates/cpp/review/refinement-2026-09-07/crouch-guard-native-keys.png) usa recortes de capturas do renderer real sem redimensionamento ou retoque. Ele compara poses; não representa uma sequência de inputs da partida.

A caminhada reúne **quadros completos** das gerações v6 (0/1/2) e v7 (3). Não há montagem de membros, poses desenhadas por código ou escala individual por quadro. A escala uniforme 268/638 corresponde ao novo tamanho anatômico da folha. Os pivôs 708/710/707/712 compensam somente a posição vertical do desenho na fonte; o ciclo conserva 100ms por chave. A faixa dourada e a perna lisa tornam a alternância dos contatos identificável, embora a oscilação dos braços permaneça discreta.

A guarda v6 parte da crouch v2 e leva a mão traseira à proteção do rosto/peito, conservando a profundidade corporal. Os pivôs foram recalibrados para os apoios da nova fonte, sem alteração do corpo físico. Conserva escala 268/689 e tempos 110/90/140ms. O chute aéreo conserva escala 268/650, pivôs virtuais e tempos 116/167/150ms.

## Fontes, identidade e tentativas

Referências consultadas antes das gerações: [master](reference/master-existing.png), [idle](idle/source-v1.png) e as fontes anteriores da ação. A personagem permanece uma mulher adulta de cabelo castanho/dourado, camisa branca amarrada, calça preta, botas de salto com dourado, luvas, bolsa redonda e alça contínua. O projétil C++ existente foi reaproveitado após a revisão de legibilidade; nenhuma nova arte era necessária para ele.

- Walk v5 e v6 completos foram descartados pela quarta passagem; v7 corrigiu a passagem, mas regrediu a terceira chave. As quatro chaves selecionadas e os retângulos exatos estão em `source_components` e no script de montagem. As três folhas completas permanecem preservadas.
- Air kick v2 elevou a perna inteira e foi rejeitado; v3 corrige a bota mantendo a postura descendente.
- Crouch block v5 levantou excessivamente o tronco e foi rejeitado; v6 conservou a profundidade da crouch.

Todos os prompts e PNGs originais foram preservados por versão. O alpha foi extraído pelo helper genérico `remove_generated_magenta.py`, **sem `--rust-contour`**. Recortes terminam antes de um pixel residual isolado no canto inferior esquerdo das folhas; os pixels dos personagens não foram retocados. As configurações e hashes anteriores estão no [snapshot](snapshots/2026-09-07-before-finalization/sha256.json).

## Evidência de execução

O [Lab filtrado](../../candidates/cpp/review/refinement-2026-09-07/lab/capture-summary.json) capturou 36 PNGs reais de air_kick, crouch e crouch_block; 12 imagens selecionadas acompanham o resumo. O manifesto visual carregado coincide com o exportado e o combate continua usando `cpp-fighter-atlas-a.png` do baseline. As capturas mostram a preparação, o contato e a recuperação do chute, além das três chaves de cada postura baixa. O Lab usa orientação direita; o World complementa a revisão nas duas orientações.

O [World](../../candidates/cpp/review/refinement-2026-09-07/world/capture-summary.json) concluiu 624 capturas e 1.845 estados simulados. Cada jogador executou os dez golpes na sua orientação e voltou ao idle, apoiado, após cada golpe. Foram inspecionados os quatro desenhos de caminhada avançando/recuando, a volta à guarda, as duas transições aéreas e as poses ativas/emissão dos dez golpes nos dois lados. As 10 recuperações foram verificadas pelo harness. A vida final 91×91 reflete o acerto normal dos projéteis, sem restauração de HP. A imagem carregada permaneceu com SHA-256 `8e85e765b6257d590fe9de32aae8887b19e3964db647ca629de5a73811d34bb3` durante a execução.

O [vídeo](../../candidates/cpp/review/refinement-2026-09-07/world/runtime-motion.mp4) tem exatamente 1.845 quadros, 60fps e 30,750s, confirmados por ffprobe. Mantém o tempo dos ticks, sem interpolação nem pausa extra no final. A captura é amostrada: conserva os PNGs entre os ticks capturados, além de registrar mudanças de pose/fase; não contém um PNG diferente para cada tick. O [índice](../../candidates/cpp/review/refinement-2026-09-07/world/evidence-index.json) identifica os recortes e âncoras dos seis painéis. [Caminhada](../../candidates/cpp/review/refinement-2026-09-07/world/evidence-panels/walk_in.png), [retorno ao idle](../../candidates/cpp/review/refinement-2026-09-07/world/evidence-panels/walk-return-idle.png), [chute aéreo](../../candidates/cpp/review/refinement-2026-09-07/world/evidence-panels/air_kick-transition.png) e [dez golpes nos dois sentidos](../../candidates/cpp/review/refinement-2026-09-07/world/evidence-panels/all-ten-moves-both-facings.png) estão disponíveis em pixels nativos. O salto dedicado pousou no tick543 com fundo físico em y616, retornando ao idle; o contorno sólido dos pés coincide com a âncora.

O agente principal também abriu uma [partida nativa C++ × Rust](../../candidates/cpp/review/refinement-2026-09-07/native-match-result.png), sem variável de opt-in, e inspecionou luta, agachamento, apoio dos pés e resultado. A partida terminou em 49×0. Os [hashes e comando](../../candidates/cpp/review/refinement-2026-09-07/native-match-assets.json) identificam o mesmo candidato; o log permanece em `target/art/cpp-finalization-match/app.log`.

## Reprodução

```sh
python3 assets/production/cpp/walk/assemble-selected-v8.py
python3 tools/art/prepare_reviewed_actions.py assets/production/cpp/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/cpp/production.json
python3 tools/art/render_sprite_review.py assets/candidates/cpp/cpp-fighter.sprite.json
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_sprite_review -- cpp target/art/cpp-finalization-lab air_kick,crouch,crouch_block
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_motion_review -- target/art/cpp-finalization-world cpp --attacks --debug-boxes
python3 tools/art/encode_motion_review.py target/art/cpp-finalization-world --fps 60
cargo test --test sprite_candidates
```

Use diretórios novos para repetir as capturas. Os dois testes `sprite_candidates` passaram, verificando cobertura, ordem das chaves e correspondência dos desenhos com todos os ticks e limites ativos. A [auditoria do refinamento](finalization-audit.json) confirma alpha real, 61 quadros visíveis, tempos preservados e ausência de novos campos de combate.

O espelhamento continua refletindo os símbolos da bolsa. A animação usa chaves discretas, com pequenas variações de detalhes entre fontes. As conclusões visuais se restringem aos quadros e sequências inspecionados; a aprovação do refinamento não é uma alegação de perfeição pixel a pixel.
