# Rust walk v8 — revisão da sequência

Nova arte raster produzida com a ferramenta integrada `imagegen`, mantendo
`../reference/master-existing.png` e `../idle/reviewed.png` como referências de
identidade. `source-v4.png` a `source-v8.png` e os respectivos prompts foram
preservados. O mapeamento anterior está em `action-before-v8.json`.

V4 e v5 foram rejeitadas: os contatos repetiam a mesma perna à frente. V6 produziu
isoladamente as duas poses da metade oposta. V7 completou o ciclo e v8 corrigiu o
pé da segunda pose, que ainda dobrava atrás do corpo. A folha selecionada contém
quatro desenhos diferentes: contato próximo, passagem distante, contato distante
e passagem próxima. O bolso da perna próxima permite acompanhar a alternância.

A ferramenta voltou a devolver RGB com xadrez, apesar do pedido explícito de
alpha real. O fundo foi removido com `remove_generated_checkerboard.py`, conforme
a autorização do usuário recuperada no histórico. `cleaned-v8.png` é o PNG RGBA
preparado, separado da fonte. Os previews claros/escuros são apenas diagnóstico.

## Integração proposta

- Folha: `walk/cleaned-v8.png`, 2048 × 768.
- Escala uniforme: `268 / 614 = 0.4364820846905538`.
- Duração preservada: quatro poses de 100 ms, ciclo completo de 400 ms.
- Retângulos e pivôs explícitos: `action.json`; nenhum recorte automático por
  ordem da folha ou ajuste independente de escala por quadro.
- `review-plan.json`, `production.json`, `prepared.png`, atlas candidatos,
  documentação compartilhada e runtime não foram alterados nesta subtask.

| Quadro | Retângulo x/y/w/h | Pivô local x/y | Fase |
|---|---|---|---|
| `walk_00` | 0/0/560/768 | 327/706 | contato próxima |
| `walk_01` | 560/0/464/768 | 267/706 | passagem distante |
| `walk_02` | 1024/0/560/768 | 301/714 | contato distante |
| `walk_03` | 1584/0/464/768 | 191/708 | passagem próxima |

Os pivôs seguem o eixo do corpo e o apoio da sola. Na reescala proposta, o topo
dos quatro desenhos fica 268–269 px acima do chão; o último pixel opaco das solas
de apoio fica 1–2 px acima do pivô. A escala é a mesma para toda a ação. Nada foi
alterado em velocidade, colisão, corpo físico ou timing de gameplay.

## Evidência e limites

Inspecionados: fonte inteira, recorte em tamanho gerado sobre fundos claro e
escuro, quatro quadros no tamanho proposto de runtime e suas duas orientações.
Não há partes cortadas ou resíduos claros evidentes nesses previews. Olhos,
goggles, rosto, roupa, mãos e acessórios permanecem reconhecíveis. O tronco em
guarda está ligeiramente mais frontal que na v3, próximo da referência idle.

`review/walk.gif` contém as quatro poses e as duas orientações, com 400 ms de loop;
`review/walk-frames.png` mostra os quadros na escala proposta. O manifesto de
diagnóstico é `review-v8.sprite.json`. A inspeção desta subtask foi estática,
quadro a quadro; a fluidez em tempo real, o movimento World e a transição ao idle
devem ser confirmados na integração do piloto pelo agente responsável.

As poses de contato têm a sola dianteira alguns pixels da fonte acima do apoio
traseiro; isso corresponde a poucos pixels de runtime. A ferramenta de cintura
acompanha a perna e varia de inclinação nas passagens. O espelhamento ainda inverte
o símbolo R, seguindo o contrato atual. Esses pontos não foram ocultados por
mudanças de gameplay. `reviewed` permanece `false` até a revisão funcional final.

Verificados por script: quatro nomes distintos, 400 ms, alpha 0–255, retângulos
dentro da fonte e margens transparentes em todos os lados. `git diff --check`
passou para esta pasta. Nenhum teste Rust foi executado por esta subtask, que
alterou somente a produção da caminhada.

## Integração concluída

A [revisão do piloto](../finalization-review.md) registra a exportação e a conferência posterior no World, Lab e Studio nativo. O texto acima preserva a entrega intermediária da geração; `action.json` agora declara a revisão concluída, com a evidência funcional vinculada.
