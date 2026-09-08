# Novo Go — revisão independente do conjunto integrado

**Os 20 clips / 60 quadros estão aprovados na revisão estática, com o salto corrigido e ressalvas menores explícitas.** O achado de proporção do jump foi resolvido pela fonte real v2 e pela comparação visual conjunta com o coordenador. A recaptura nativa da versão promovida está em andamento com verification_pipeline; este laudo não afirma sua conclusão.

O [registro estruturado](integration-review.json) guarda os hashes atuais, o histórico do achado e a comparação por quadro. Foram vistos os 20 GIFs em ordem, todos os 60 quadros nas duas direções e em fundos claro/escuro, as caixas dos nove ataques e a origem do special. Após a promoção, os três quadros de jump foram inspecionados novamente.

**A exportação alterou somente jump.** Os outros **57 quadros são idênticos pixel a pixel**, incluindo todos os bytes RGBA de pixels transparentes, aos [assets preservados antes do refinamento](../../candidates/go/review/refinement-2026-09-07/world-before-jump-refinement/runtime-assets/go-fighter.sprite.json). Dimensões e metadados completos desses quadros também são idênticos. Nomes, ordem e loops dos 20 clips, além da escala do manifesto, permanecem iguais. O JSON do laudo inclui os hashes antes/depois de cada recorte.

**Jump resolvido.** A [fonte v2](jump/source-v2.png) mantém volume e proporções coerentes com idle e os aéreos, usando escala única **0,37**. A faixa superior de 30 px da cabeça mede **55/54/53 px**, contra 55 px no idle; a flexão natural do ápice é preservada. Apenas o registro virtual da descida recebeu o ajuste final de cerca de 8 px, colocando a sola sólida em **Y−1** no atlas promovido. Os tempos continuam **60/340/400 ms**. Veja a [comparação selecionada](jump/review/transition-v2.png), as [duas direções](jump/review/both-facings-v2.png) e o [laudo da correção](jump/refinement-review-v2.md). V1, sua calibração e a tentativa v3 permanecem preservadas.

**Ressalvas menores.** Em sweep_01 a palma sólida termina em Y−6 e a pata de apoio em Y+1 relativos à âncora (alpha ≥128): diferença de 7 px. O desenho admite profundidade, mas o apoio não é perfeitamente coplanar; o contato da sola continua dentro da caixa. Em air_kick_01 apenas a ponta fina da garra fica junto à borda frontal (~1 px além segundo o lote C), com o volume útil da pata dentro da caixa. O último spawn tem azul um pouco mais saturado que idle, mantendo identidade e roupa.

| Ação | Quadros / total | Resultado |
| --- | ---: | --- |
| idle | 3 / 540 ms | Respiração discreta, cabeça/faixa/apoios estáveis. |
| walk | 4 / 240 ms | Quatro apoios distintos em avanço de guarda; 240 ms deixam o ciclo rápido. Mantém a ordem das pernas e fecha a base antes de avançar a pata frontal. |
| jump | 3 / 800 ms | V2 selecionada restaura volume de cabeça/tronco coerente com idle/aéreos; pernas recolhem e a sola da descida fica 1 px acima da âncora. Tempos 60/340/400 ms preservados. |
| crouch | 2 / 360 ms | Entrada de 80 ms e hold de 280 ms compactos; a pose descartada não foi exportada. |
| block | 3 / 300 ms | Guarda alta, absorção e retorno legíveis; sem deslocamento anormal dos apoios. |
| crouch_block | 3 / 300 ms | Proteção compacta compatível com crouch hold, sem mudança evidente de escala da cabeça. |
| hit | 3 / 260 ms | Impacto, dobra do tronco e recuperação identificáveis; expressão adulta. |
| punch_light | 3 / 233 ms | Punho curto dentro da caixa; inclinação para golpe no corpo e retorno à guarda. |
| punch_heavy | 3 / 583 ms | Punho e alcance longo dentro da caixa; inclinação maior que no jab sem reescala entre chaves. |
| kick | 3 / 383 ms | Joelho arma, sola baixa entra na caixa e perna recolhe; pata de apoio permanece legível. |
| sweep | 3 / 533 ms | Sola dentro da caixa em altura; mão de apoio sólida fica 6 px acima da âncora e pata 1 px abaixo (ressalva de profundidade). |
| overhead | 3 / 466 ms | Braço arma alto e punho desce até a caixa; recuperação retorna à guarda. |
| throw | 3 / 366 ms | Duas mãos distintas com cotovelos dobrados; a mão frontal entra na caixa curta. |
| anti_air | 3 / 500 ms | Punho alto integralmente na caixa; extensão de pernas/tronco explica postura mais alta. |
| air_punch | 3 / 366 ms | Punho diagonal para baixo na caixa; perna traseira e mãos completas. Continuidade com jump v2 selecionada após comparação visual. |
| air_kick | 3 / 333 ms | Sola/pata na caixa; ponta da garra junto à borda frontal (~1 px). Continuidade com jump v2 selecionada após comparação visual. |
| special | 3 / 267 ms | Origem baseline de emissão toca a base externa da palma no primeiro quadro; projétil segue separado. |
| spawn | 3 / 2700 ms | Ajuste de faixa, reverência e guarda; pernas estendidas explicam altura inicial. O azul do último quadro é um pouco mais saturado que idle. |
| victory | 3 / 860 ms | Relaxamento, celebração e final contido; mãos/faces e apoio legíveis. |
| defeat | 3 / 1040 ms | Descida em duas etapas para apoio no chão, sem ampliar a pose final. |

As transições idle→walk→idle, crouch↔crouch_block e idle↔block preservam escala e leitura dos apoios. O walk usa quatro poses distintas em avanço rápido de guarda. Os golpes apresentam preparação, contato e recuperação legíveis; os nove limites ativos e os 20 totais coincidem com o contrato. Todos os 60 recortes têm alpha real e margem transparente, sem corte de membro ou fusão de rosto/mãos que comprometa a leitura identificados.

Os GIFs de revisão acrescentam 400 ms ao último quadro não cíclico e arredondam tempos para 10 ms; não são prova da cadência nativa. Capturas Lab/World e partida nativa da versão final pertencem à verificação do coordenador/verification_pipeline. Nenhuma fonte, metadado de produção, manifesto ou atlas global foi alterado nesta atualização do laudo.
