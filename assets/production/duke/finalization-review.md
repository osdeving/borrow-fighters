# Duke — revisão final do refinamento

Revisão de 2026-09-07. Os quatro problemas apontados foram tratados: alternância da caminhada, defesa baixa compacta, trajetória descendente do overhead e halo/clarão cortado do projétil. Exportação: **20 clips, 61 frames**, em [manifesto](../../candidates/duke/duke-fighter.sprite.json) e [atlas](../../candidates/duke/duke-fighter-atlas.png).

## Fontes e seleção

| Item | Seleção real | Preparação | Duração preservada |
| --- | --- | --- | --- |
| walk | v5, recortes 0/1; contato oposto v7 derivado de v4; passagem v6 | Quatro poses diferentes; fontes isoladas normalizadas uniformemente por 0,49, depois escala comum 0,4169435216 | 112/113/112/113 ms = 450 ms |
| crouch_block | v2, três poses | Barriga comprimida e cone inclinado, mantendo tamanho do nariz, luvas e sapatos; escala uniforme 0,4169435216 | 80/100/180 ms = 360 ms |
| overhead | v6 início/fim; v8 contato isolado | Contato normalizado uniformemente por 0,53, depois escala comum 0,4169435216 | 250/133/283 ms = 666 ms |
| projectile | v1 novo | Nove grãos, alpha extraído e redução uniforme; textura separada 158×132, renderizador mantém 0,6 | Temporização do golpe inalterada |

Todos esses desenhos foram produzidos pela ferramenta integrada `image_gen`, com a [referência fixa](reference/master-existing.png) e continuidade do idle. Os pedidos e as fontes vN, inclusive tentativas rejeitadas, foram preservados. [Proveniência e rejeições](refinement-generation.json), [composição da caminhada](walk/composition-v1.json), [composição do overhead](overhead/composition-v1.json) e [projétil](projectile/README.md) registram os detalhes.

As outras **17 ações foram reaproveitadas sem alteração de seus dados preparados**: idle, crouch, jump, block, hit, punch_light, punch_heavy, kick, sweep, anti_air, air_punch, air_kick, throw, special, spawn, victory e defeat. A comparação com o snapshot anterior e os tempos estão em [refinement-validation.json](refinement-validation.json).

## Leitura visual e contato

- A caminhada alterna contato da perna próxima, passagem da distante, contato da distante e passagem da próxima. Há quatro desenhos reais; não foram acrescentadas cópias para aumentar a contagem. [Sequência em tamanho de runtime](../../candidates/duke/review/walk-frames.png) e [dois sentidos](finalization-evidence/walk-both-facings-2.png).
- A defesa baixa reduz a altura sólida para cerca de 138 px. O cone ultrapassa o corpo físico agachado de 128 px em aproximadamente 10 px; vapor é detalhe visual. [Captura com caixas](finalization-evidence/lab-crouch_block-f018-idle.png).
- No overhead, o cotovelo fica acima do punho e o antebraço desce para o contato. A luva ocupa aproximadamente x=63…102, y=−150…−109 em relação ao pivô, dentro da caixa original x=50,67…178,67, y=−170,67…−98,67. [Lab ativo](finalization-evidence/lab-overhead-f015-active.png) e [World nos dois sentidos](finalization-evidence/world-f1082-overhead.png).
- O projétil mantém grãos torrados e rastros creme; a nova distribuição é mais compacta horizontalmente e elimina o clarão cortado. [Alpha claro/escuro](projectile/alpha-review-v1.png), [disparo real](finalization-evidence/lab-special-f018-idle.png) e [trajeto após recuperação](finalization-evidence/lab-special-f032-idle.png).

A ferramenta devolveu RGB apesar do pedido de transparência. A remoção local de fundo estava autorizada. Foram usados os extratores existentes de magenta/quadriculado, sem `--rust-contour`. A extração inicial de v4 perdeu parte da barriga branca; foi rejeitada e substituída por v7 com matte magenta. As lacunas de fundo fechadas entre os braços foram revisadas e removidas por sementes explícitas em [prepare_refinement.py](prepare_refinement.py). A barriga, o logo, os contornos e o vapor foram inspecionados em fundos claros e escuros.

## Verificação

`cargo test --test sprite_candidates -- --nocapture`: **2 testes passaram**, cobrindo clips obrigatórios, ordem das fontes e correspondência entre a pose de ataque e cada tick de combate. Os nomes descritivos das fases do overhead foram corrigidos para `startup`, `active` e `recovery`; isso preservou bytes do atlas, tempos, pivôs e recortes. O SHA256 do atlas é `0488acccdb9537f993b53c27ce2a28a2f01e031235f6743aaf010904c37c5634`.

Combat Lab filtrado (`crouch_block,overhead,special`): **42 PNGs reais**, incluindo limites das fases do overhead, defesa baixa e projétil. [Resumo portátil](finalization-evidence/lab-summary.json). Capturas completas: `target/art/duke-finalization-lab`.

World com entradas normais e caixas de debug: **636 PNGs, 1.845 estados de simulação**, os dez golpes nos dois sentidos, as quatro poses de caminhada e retorno ao idle após cada golpe. [Vídeo portátil, 60 fps e 30,75 s](finalization-evidence/runtime-motion.mp4) e [resumo](finalization-evidence/world-summary.json). Capturas completas: `target/art/duke-finalization-world`.

A partida nativa `cargo run -- --fight --p1 duke --p2 rust`, executada no fluxo principal sem variável de candidatos, terminou **Duke 14 × 0 Rust**. [Resultado real](../../candidates/duke/review/refinement-2026-09-07/native-match-result.png) e [hashes dos assets carregados](../../candidates/duke/review/refinement-2026-09-07/native-match-assets.json). Vitória/derrota têm apoio no chão e os corpos permanecem inteiros na borda.

O World usa entradas controladas e ataques próximos sem contato à distância desse cenário; o Lab complementa a análise de alinhamento. O vídeo mantém a imagem GPU anterior entre amostras e não interpola poses. Esta revisão não mede balanceamento, áudio ou dispositivos de entrada. Nenhuma regra de combate, dimensão física, origem de projétil ou arquivo `src/` foi alterado por este refinamento.

## Reprodução

Após extrair alpha das fontes selecionadas com os scripts existentes, executar:

```sh
python3 assets/production/duke/prepare_refinement.py
python3 assets/production/duke/projectile/prepare.py
python3 tools/art/prepare_reviewed_actions.py assets/production/duke/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/duke/production.json
python3 tools/art/render_sprite_review.py assets/candidates/duke/duke-fighter.sprite.json
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_sprite_review -- duke target/art/duke-finalization-lab-new crouch_block,overhead,special
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_motion_review -- target/art/duke-finalization-world-new duke --attacks --debug-boxes
python3 tools/art/encode_motion_review.py target/art/duke-finalization-world-new
```

Os snapshots `action-before-refinement.json`, `review-plan-before-refinement.json` e `production-before-refinement.json` preservam o estado anterior. Sem commit ou push.
