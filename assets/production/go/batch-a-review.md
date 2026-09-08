# Novo Go — movimento, guarda e reação

Atualização: o salto inicial v1 foi substituído após detectar anatomia reduzida. A revisão atual usa [jump v2](jump/refinement-review-v2.md), escala uniforme 0,37 e pivô virtual de descida com a sola cerca de1px acima da âncora. A comparação com idle/aéreos foi aceita; a nova captura nativa está em andamento. As outras seis ações mantêm a seleção descrita abaixo.

Sete ações realmente geradas com o novo [master](reference/master.png), uma folha por ação. A referência antiga de Go não foi usada. Fontes RGB e prompts exatos permanecem em cada pasta; `keyed-v1.png` contém a extração de alpha autorizada. A revisão de fontes e dos painéis em escala de jogo foi concluída; a validação nativa do conjunto completo vem depois da integração dos lotes B e C.

| Ação | Chaves selecionadas | Registro e leitura |
| --- | --- | --- |
| idle | 3, 540 ms | Respiração discreta; cintura e sola fixas; altura visual de 264 px. |
| walk | 4, 240 ms | Deslocamento marcial curto: contato, transferência de peso, aproximação da perna traseira e avanço da dianteira. As pernas mantêm sua ordem de guarda. Apoio medido por pose; uma escala anatômica para toda a folha. |
| jump | 3, 800 ms | Saída, recolhimento e descida. Pivôs virtuais pela cintura, com deslocamento de 447 px de fonte até o apoio do idle; a trajetória é fornecida pelo World. |
| crouch | 2, 360 ms | Entrada de 80 ms e pose baixa de 280 ms. A célula central da fonte foi rejeitada por um detalhe escuro inventado abaixo do focinho; não é exportada. |
| block | 3, 300 ms | Guarda alta, absorção e assentamento, mantendo apoio e compressão natural dos joelhos. |
| crouch_block | 3, 300 ms | Guarda baixa da mesma referência do crouch selecionado; alturas aproximadas de 124/133/123 px. |
| hit | 3, 260 ms | Impacto recebido, recuo do tronco e recuperação da guarda. Sem atacante ou efeito pintado. |

As folhas de três poses têm 1881×836 px reais; walk tem 2172×724. Escalas: 264/783 nas seis ações e 264/620 em walk. Retângulos e pivôs foram medidos na imagem retornada, sem assumir a resolução pedida ao gerador. O [registro numérico](batch-a-review.json), os `action.json`, painéis e GIFs de cada pasta preservam a seleção.

Os painéis usam chão fixo, fundo claro/escuro e espelhamento. Não foi identificado recorte de mão/pé ou resíduo magenta no corpo exportável. O maior recolhimento das pernas no salto é intencional; não se prende cada sola aérea ao chão. A escala uniforme preserva a flexão de tronco e joelhos entre poses. Os GIFs acrescentam 400 ms de pausa final somente para inspeção nas ações sem loop, conforme o renderizador; o JSON conserva o timing do jogo.
