# Novo Go — produção separada

Substituir a aparência anterior por uma versão menos cartoon e mais realista, conforme a nova direção do usuário. O master e o idle que o agente principal selecionar em `reference/` passam a ser a autoridade visual. A identidade e as poses do Go anterior não devem ser usadas como referência de arte nova.

O conjunto contém **20 clips, 60 chaves e os dez golpes**. Todos os lotes foram realmente gerados e revisados; o atlas completo está integrado em `assets/candidates/go`. A seleção de duas poses para crouch reduziu a previsão de 61 para 60 chaves, conservando sua duração. A validação nativa final está concluída; veja o [laudo final](finalization-review.md). Não há alteração de gameplay.

## Três lotes independentes

| Lote | Ações | Responsabilidade |
| --- | --- | --- |
| A — movimento e guarda | idle, walk, jump, crouch, block, crouch_block, hit | Fixar escala/apoios com o idle novo; alternar as quatro chaves de walk; manter a profundidade entre crouch e guarda baixa; registrar salto sem uma segunda trajetória desenhada. |
| B — golpes terrestres | punch_light, punch_heavy, kick, sweep, overhead, throw | Gerar preparação, contato e recuperação próprios; desenhar punhos/pés nas caixas existentes e manter apoio ao chão. |
| C — aéreos, especial e resultados | anti_air, air_punch, air_kick, special, spawn, victory, defeat | Manter a âncora corporal dos aéreos, alcançar a caixa alta do anti-air, emitir desde o primeiro quadro do especial e concluir entrada/resultados com a nova identidade. |

Lote A: agente principal; lote B: audit_moves; lote C: verification_pipeline. Cada produtor edita somente as pastas das ações do seu lote e seu registro de revisão. O integrador controla `reference/`, `review-plan.json`, `production.json` e o atlas combinado. Se o idle já vier pronto do coordenador, o lote A o recebe como arte realmente gerada, sem duplicar a geração.

O [inventário](inventory.json) define a divisão e os tempos de todos os clips. Os [contratos por ação](action-contracts.json) incluem contato em ambas as orientações, fases, âncoras, nomes e breve orientação de desenho. Os contatos foram conferidos contra `MoveSpec` atual, não contra a aparência descartada.

## Contratos que permanecem

- Corpo físico: largura101,333px, altura224px e crouch128px. Âncora no centro horizontal/fundo do corpo. O alvo visual anterior de264px serve apenas como referência de registro; a anatomia do novo master deve ser medida antes de escolher uma escala uniforme por ação.
- Fonte direita: `source_point = source_pivot + world_offset / action_scale`. À esquerda, pontos trocam o sinal de x; a caixa vira `x_left = -(x_right + width)`. O runtime faz o espelhamento.
- Fonte e pivô devem ser medidos de novo. Não copiar os pivôs numéricos, alturas de fonte ou retângulos do desenho anterior.
- Nove ataques próximos usam `MoveSpec` e os tempos inclusivos existentes. Preparação termina em `floor(active_start*1000/60)`; recuperação começa em `floor((active_end+1)*1000/60)`. Todos os ataques ativos usam corpo em pé, inclusive sweep.
- Especial emite no tick0, na borda próxima do projétil em (+97,92;−123,84), ou (−97,92;−123,84) espelhado. Projétil48×32px permanece separado; o primeiro desenho já emite. Duração visual16ticks, cooldown44ticks. Preservar80/80/107ms na folha planejada.
- Todas as chaves de uma ação compartilham escala anatômica. Recalibrar posicionamento da fonte é permitido; aumentar/diminuir uma pose para fazê-la caber na caixa não é.

| Golpe | Preparação/contato/recuperação | Caixa direita relativa à âncora: x, y, largura, altura |
| --- | --- | --- |
| punch_light | 50/83/100ms | 50,667; −141,333; 56; 40 |
| punch_heavy | 183/167/233ms | 50,667; −146,667; 128; 56 |
| kick | 116/117/150ms | 50,667; −82,667; 114,667; 45,333 |
| sweep | 166/150/217ms | 50,667; −136; 149,333; 40 |
| overhead | 166/100/200ms | 50,667; −168; 93,333; 64 |
| anti_air | 116/134/250ms | 50,667; −338,667; 93,333; 109,333 |
| air_punch | 83/150/133ms | 50,667; −130,667; 96; 56 |
| air_kick | 83/133/117ms | 50,667; −106,667; 104; 56 |
| throw | 100/50/216ms | 50,667; −184; 61,333; 160 |

O JSON preserva a precisão completa; a tabela arredonda somente para leitura.

## Fluxo executado — registro da produção em staging

1. Receber e visualizar master/idle novos selecionados pelo coordenador. Fixar rosto, material, roupa/acessórios, luz e proporção a partir deles.
2. Usar imagegen real, uma ação por folha, com master/idle e apenas referências novas coerentes. Salvar prompt e fonte por versão; tentativas rejeitadas permanecem rastreáveis. Não apresentar recortes do Go antigo como geração nova.
3. Extrair alpha com o helper genérico autorizado, **sem `--rust-contour`**. Inspecionar fundo claro/escuro, mãos, pés e detalhes completos.
4. Preencher retângulos/pivôs e escala única por ação depois de ver os pixels gerados. Conferir apoio, cabeça/cintura, contato, recuperação e as duas orientações. Não aprovar quadros ausentes ou apenas por existência do arquivo.
5. Integrador reúne somente as ações realmente revisadas. Fazer a primeira exportação em `target/art/go-replacement-candidate`, mantendo `character=go`; **não omitir `--output-dir`**, pois o padrão sobrescreveria `assets/candidates/go`.
6. Depois da revisão combinada, o coordenador executa a substituição planejada e o gate nativo: Lab com baseline real, World com caminhada/salto/dez golpes nos dois sentidos, retorno à guarda, vídeo60fps e partida com resultado. Os harnesses atuais carregam o caminho normal do personagem; uma captura do Go antigo não comprova a versão nova ainda em staging.

Comandos da primeira exportação em staging, antes da promoção:

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/go-replacement/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/go-replacement/production.json --output-dir target/art/go-replacement-candidate
python3 tools/art/render_sprite_review.py target/art/go-replacement-candidate/go-fighter.sprite.json
```

Os 263 arquivos anteriores e seis insumos do contrato foram conferidos antes da promoção. O Go antigo foi movido integralmente para `assets/production/go-cartoon-archive` e `assets/candidates/go-cartoon-archive`; a produção nova passou de `go-replacement` para `go`. O [snapshot original](historical-preservation.json) e o [mapeamento da mudança](archive-mapping.json) preservam a rastreabilidade. Os nomes antigos nos comandos acima descrevem a etapa de staging, já encerrada.

Para reconstruir o conjunto atual:

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/go/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/go/production.json
python3 tools/art/render_sprite_review.py assets/candidates/go/go-fighter.sprite.json
```
