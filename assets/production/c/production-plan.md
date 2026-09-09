# C — preparação das 20 ações

Estado atual: **20 ações e 61 quadros entregues como candidato**, com verificação e ressalvas no [laudo de revisão](review.md). O piloto Rust foi validado tecnicamente antes do início desta produção. As propostas abaixo e os dados compactos em [production-plan.json](production-plan.json) preservam o planejamento inicial; a seleção final está no laudo e no manifesto exportado. A verificação técnica não constitui aprovação artística final. O [contrato de contato](../contact-contract.json) mantém os dados de combate; o [inventário](../../../docs/19-sprite-production-coverage.md#c) documenta o baseline e a entrega.

## Referência e identidade

Inspeção visual realizada no [master preservado](reference/master-existing.png) e nos dois originais [langc-03](../../references/langc-03.png) e [langc-04](../../references/langc-04.png). C é um homem idoso esguio, cabelo branco longo e despenteado, sobrancelhas expressivas, bigode branco, nariz e rugas marcantes. Usa jaqueta jeans azul clara com emblema C, camiseta branca, jeans azul escuro, cinto marrom com fivela dourada e sapatos claros. O livro azul/branco com C permanece em todas as ações, inclusive chutes, dano e derrota. A fonte antiga omite o livro em alguns chutes; esse desaparecimento não deve se repetir na arte final.

Preservar anatomia idosa e pernas longas; não transformar em jovem, musculoso ou chibi. O rosto deve continuar reconhecível em tamanho de jogo. Livro e mãos precisam aparecer separados e íntegros. A vitória original corta as pernas, portanto serve apenas para o gesto de apresentar o livro. O master é recorte reutilizado, sem geração nova.

O idle gerado tornou-se a referência de acabamento, junto do master original fixo em **cada** geração seguinte. Alvo inicial: altura visual em pé de aproximadamente 268 px, corpo físico preservado em 101,333 × 224 px. A escala fonte→runtime foi medida pela referência anatômica, sem normalizar cada pose pela própria altura aparente. Fundo de geração magenta sólido #FF00FF; limpeza pelo helper genérico, **sem `--rust-contour`**.

## Coreografia por ação

| Ação | Poses propostas | Leitura visual |
|---|---:|---|
| idle | 4 | Respiração discreta, livro junto ao corpo, mão livre em guarda, solas estáveis. |
| walk | 4 | Passadas alternadas com apoio visível, livro seguro, ciclo capaz de reproduzir avanço e recuo. |
| jump | 4 | Impulso curto, subida com pernas recolhidas, ápice/queda, preparação da aterrissagem; âncora acompanha a física. |
| crouch | 3 | Flexão, acomodação e sustentação baixa; manter livro e pés. |
| block | 3 | Livro já protegendo no primeiro quadro, compressão do bloqueio e guarda sustentada. |
| crouch_block | 3 | Guarda baixa própria com livro à frente; nenhuma pose em pé no início. |
| hit | 3 | Reação imediata, recoil, estabilização; livro retido pela mão. |
| punch_light | 3 | Preparação curta, golpe curto com borda do livro na altura do abdômen, recolhimento. |
| punch_heavy | 3 | Carregar livro junto ao corpo, estocada pesada mais longa na altura do abdômen, recuperação clara. |
| kick | 3 | Recolher perna, chute baixo na canela, retornar apoio; livro seguro junto ao torso. |
| sweep | 3 | Baixar centro do corpo, estender perna oblíqua com canela/sola cruzando a caixa existente, recolher; não desenhar contato rente ao chão. |
| overhead | 3 | Erguer livro acima da cabeça, golpe descendente com contato à frente na metade do corpo, recuperação. |
| anti_air | 3 | Preparação baixa, braço/livro estendido para cima e à frente, recolhimento; sem deslocar artificialmente o chão. |
| air_punch | 3 | Preparação aérea, golpe descendente de livro até a região indicada pela âncora virtual, retração. |
| air_kick | 3 | Perna recolhida, chute para frente e baixo no ar, recolhimento; livro contínuo. |
| throw | 3 | Aproximação da mão livre, agarrão curto sem vítima desenhada, puxada e retorno; livro permanece na outra mão. |
| special | 3 | Livro já aberto/apresentado no ponto de emissão no primeiro quadro, acomodação e recuperação; bitstream fica separado. |
| spawn | 4 | Entrar com livro, apresentá-lo, apontar como professor, chegar à guarda. Corpo inteiro em todas as poses. |
| victory | 3 | Erguer livro, gesto vitorioso com dedo, sustentar pose inteira. |
| defeat | 4 | Recoil, joelho ao chão, queda lateral controlada, repouso final com livro junto à mão. |

São 65 poses propostas pela necessidade destas sequências, sem cota fixa. Uma chamada de imagegen por ação, seguida de inspeção e correção antes da próxima. Uma revisão pode reduzir ou acrescentar poses se o movimento exigir.

## Contato, timing e revisão

Todos os nove ataques usam a altura física em pé durante o ataque, inclusive sweep. As caixas não acompanham uma escolha estética de agachamento. O contrato define offsets em relação à âncora nos pés e deve ser projetado para a fonte após medir escala e pivô: `ponto_fonte = pivô + offset_mundo / escala`. No ar, manter a mesma interpretação da âncora, que se move com o lutador.

As três poses de cada golpe usam preparação/ativo/recuperação. Os limites são `floor(active_start * 1000 / 60)` e `floor((active_end + 1) * 1000 / 60)`; fim inclusivo preservado. O especial emite no tick 0 e tem 333 ms de duração visual (20 ticks); o cooldown de 56 ticks não prolonga o clip. Origem inicial à direita: `(148,48; -140,746666...)` px relativos à âncora.

Entregar por ação fonte versionada, prompt, folha com alpha e `action.json` com retângulos/pivôs explícitos e durações. Deixar `reviewed=false` até revisão do runtime. Conferir lado direito e espelhado, contato no primeiro/último tick ativo, recuperação, solas, acessório contínuo e recorte. Preservar todas as versões rejeitadas para rastreabilidade.
