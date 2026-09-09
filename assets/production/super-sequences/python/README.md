# Python — import devour

Quatro atlas produzidos com `imagegen` integrado em 9 de setembro de 2026 para
o super da [rodada 24](../../../../docs/24-reactions-and-transformations.md).
São 32 desenhos, oito por folha, em PNG RGBA de 1448 × 1086 pixels. O conjunto
serve à animação do protótipo; não declara encerrada a direção de arte do jogo.

A [referência existente](../../python/reference/master-existing.png) orienta a
identidade da mulher adulta: cabelo preto, camisa branca, saia preta, sapatos
de salto e acessório de serpente azul/amarela. A transformação usa essa paleta
numa serpente gigante, inspirada no mascote Python conforme o pedido registrado.

| Atlas | Uso no roteiro | Prompt |
|---|---|---|
| [transform.png](transform.png) | Desenhos intermediários da forma humana até a serpente. | [Transformação](prompt-transform.txt) |
| [serpent.png](serpent.png) | Crescimento, abertura da boca, bote e deglutição. | [Serpente](prompt-serpent.txt) |
| [revert.png](revert.png) | Retorno gradual à forma humana. | [Reversão](prompt-revert.txt) |
| [celebrate.png](celebrate.png) | Antecipação, salto com pernas dobradas, pouso e sinal de paz para o jogador. | [Celebração](prompt-celebrate.txt) |

O [renderer específico](../../../../src/engine/render/python_super.rs) mantém
recortes e pivôs explícitos nas tabelas `TRANSFORM`, `SERPENT`, `REVERT` e
`CELEBRATE`. Os pivôs usam coordenadas absolutas do atlas e referenciam o chão;
os desenhos do salto conservam sua altura relativa ao mesmo apoio. O bote longo
ultrapassa uma célula nominal: as poses envolvidas são desenhadas em partes para
preservar a silhueta e excluir pixels da pose vizinha. Não dividir as folhas numa
grade uniforme nem inferir os pivôs apenas pelo centro de cada imagem.

A posição da boca durante o bote orienta a sucção do sprite real do oponente.
O adversário é ocultado pela sequência e reaparece segundo a reação do World;
não existe vítima desenhada dentro destes atlas. O tamanho da serpente não
modifica hitboxes nem cria contatos extras de dano.

Os PNGs selecionados foram copiados intactos da ferramenta. Extrações de fundo
pela própria ferramenta removeram o checkerboard das tentativas anteriores.
O [registro de geração](generation.json) conserva origens, dimensões, hashes,
intervalos de alpha e componentes analisados: todas as folhas têm alpha de
0 a 255, com aproximadamente 57%–66% dos pixels transparentes.
As instruções de extração também foram preservadas:
[inicial](prompt-alpha.txt), [tentativa seguinte](prompt-alpha-retry.txt),
[versão curta](prompt-alpha-short.txt),
[reversão final](prompt-revert-alpha-final.txt) e
[celebração final](prompt-celebrate-alpha-final.txt).

Capturas, vídeo e limites da verificação estão no
[índice de evidências](../../../../docs/evidence/reactions-transformations/README.md).
