# Go — recaptura nativa do salto v2

O novo manifesto foi recarregado pelo seletor GTK real do Sprite Studio e comparado integralmente ao JSON exibido na UI. Foram percorridos **somente os três quadros de jump**, por botões nativos via AT-SPI, em **zoom 1.00x**. As capturas mostram a fonte `assets/production/go/jump/source-v2.png`, preparada com escala uniforme **0,37**; o manifesto exportado permanece em escala **1**.

| Quadro | Pivô exibido no Studio | Duração | Captura real |
|---|---|---:|---|
| jump_00 | 124, 276 | 60 ms | [PNG](jump-00.png) |
| jump_01 | 109, 268 | 340 ms | [PNG](jump-01.png) |
| jump_02 | 87, 290 | 400 ms | [PNG](jump-02.png) |

Os três valores de pivô, duração e escala exibidos conferem com o manifesto carregado. O [painel](jump-v2-native-panel.png) reúne recortes da janela sem redimensionar ou retocar a arte. Cabeça, peito e faixa mantêm o volume da nova fonte; preparação e ápice deixam as patas suspensas, e a descida estende uma perna até perto da âncora virtual. Mãos, patas e topo da cabeça estão completos. A altura reduzida das poses recolhidas é própria da flexão das pernas; o alvo de idle exibido no Studio não deve ser aplicado à altura total dessas poses.

Esta evidência **substitui apenas os três quadros do jump anterior**, cuja anatomia reduzida foi identificada na auditoria independente depois da primeira inspeção. Os 60 PNGs e o relatório anteriores ficam preservados; os outros **19 clips / 57 quadros não foram recapturados nesta etapa**. Sua evidência continua no diretório pai, com os hashes da execução original.

O [relatório novo](review-captures.json) registra a fonte, valores da interface, conjunto carregado e hashes antes/depois. Manifesto, atlas e métricas físicas não mudaram durante a recaptura; nenhum ajuste foi salvo.

- Manifesto: `fc5560a0478019b1a45ef13cde21c34c177d12ff345ad373993913c5eab2e4da`
- Atlas: `60f5e1b428e851146072fc7e8d5a672b479fb62bb6fbba91308519d4d8efd028`
- Métricas físicas: `7f77453d98681adf5b6b0d8020ca8167dcbb8245ea97363425c5fc9cf309c962`

A inspeção cobre o carregamento e os quadros na orientação direita. Não comprova reprodução contínua ou trajetória física dentro do Studio; a recaptura World/Lab da nova versão é registrada separadamente.

```sh
python3 target/art/go-finalization-studio/jump-v2/reload_atspi_candidate.py
python3 target/art/go-finalization-studio/jump-v2/capture_atspi_jump_v2.py
```

Os [helpers de recarga](reload_atspi_candidate.py) e [captura](capture_atspi_jump_v2.py) acionam o seletor GTK, o clip `jump` e `Next frame`. O ID nativo desta sessão foi `0xa00003`; precisa ser redescoberto se o Studio for reaberto. Nenhum processo de outro agente foi tocado.

Ao concluir, somente o PID próprio `3387235` foi encerrado com SIGINT, após conferir o comando exato em `/proc`. A sessão terminou com código 130; manifesto, atlas e métricas mantiveram os hashes após o encerramento. [Registro](process-close.json).
