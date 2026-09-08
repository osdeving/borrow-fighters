# Rust — revisão da continuação

## Entrega atual

O piloto mantém **20 clips e 71 quadros**. Walk v8, sweep v6, throw v2 e air_kick v2 foram gerados com a ferramenta integrada de imagem, usando a referência original fixa e as fontes da ação. Air_punch reaproveita a fonte anterior com correção explícita de um pivô virtual. A engrenagem foi reaproveitada e limpa por processamento alpha autorizado, sem nova geração.

| Ação | Correção e verificação |
|---|---|
| walk | Quatro poses distinguem contato/passagem de cada perna. Ciclo de 400 ms, escala uniforme 268/614. World confirma avanço/recuo, duas orientações e retorno ao idle; apoio varia poucos pixels. [Laudo da fonte](walk/review-v8.md). |
| sweep | A fonte v6 coloca a sola ativa na caixa original. A v5 foi rejeitada por baixar demais. Apoios e 533 ms preservados; o matte residual entre braço e perna é removido pelo [preparador](sweep/prepare-v6.py). |
| throw | Braços dobrados e mão avançada dentro do alcance curto do Ownership; 333 ms e pivôs de chão preservados. |
| air_kick | Perna/bota ativas redesenhadas para contato na box aérea. Restaurados os pivôs virtuais originais Y650 da sequência; 433 ms e física preservados. |
| air_punch | Pivô ativo Y650→694 remove queda comum de cabeça/cintura de aproximadamente 22 px, com mão ainda na box. Fonte, outros pivôs e 366 ms preservados. [Medições](air_punch/pivot-calibration-2026-09-07.md). |
| projectile | Removida franja neutra do PNG existente; RGB e alpha dos pixels retidos preservados. Margem simétrica de 4 px mantém centro e tamanho visual do desenho. [Produção e limpeza](projectile/README.md). |

Os outros 15 clips foram examinados nos painéis/PNG em tamanho de runtime e preservados. Guardas, hit, entrada e resultados foram exercitados no Lab. Pequenas variações de cabelo/acessórios não alteram a identidade reconhecível.

## Evidência funcional

Os [arquivos de revisão](../../candidates/rust/review/refinement-2026-09-07/README.md) preservam vídeo, contatos e relatórios. O Lab completo produziu 276 capturas/19 contextos. O World produziu 650 imagens/1.845 estados com os dez golpes em ambas as orientações e retorno ao idle. A recaptura do especial limpo contém 17 imagens e confirma o carregamento do PNG candidato. Uma partida do aplicativo nativo entre CPUs chegou a Rust 9 × 0, mostrando vitória/derrota no chão.

Passaram os 17 testes focados de candidatos, seleção e reprodução após a integração. Fontes, recortes, margens, alpha e escala foram inspecionados; durações e fases continuam conferidas pelos testes contra os ticks reais do combate. O Sprite Studio nativo carregou o candidato pelo seletor de arquivo GTK e mostrou sequencialmente os 24 quadros de idle, walk, sweep, throw, air_punch e air_kick em zoom 1.00x. Manifesto, atlas e métricas mantiveram seus hashes; nenhum ajuste foi salvo. O [painel e relatório](../../candidates/rust/review/refinement-2026-09-07/studio/review-captures.json) registram os botões reais acionados via acessibilidade AT-SPI. O Studio foi conferido quadro a quadro; a evidência temporal vem do World.

## Alcance e ressalvas

Os inputs do World são controlados, não teclado manual. O vídeo respeita o tempo de simulação, mas usa capturas amostradas; não há alegação de cobertura de cada tick por imagem. Ataques próximos do cenário World são whiffs e o alinhamento de contato é conferido no Lab.

Ao sair do salto encolhido para um ataque aéreo, a extensão de tronco altera a altura da silhueta; ela foi distinguida da translação interna indevida corrigida no air_punch. O renderer continua espelhando letras/símbolos, inclusive o `R`. As poses baixas e a geometria de combate baseline mantêm o envelope físico preexistente; esta rodada não altera balanceamento para ajustar o desenho.

O [laudo anterior](review.md) permanece como histórico do primeiro lote. Esta revisão documenta os refinamentos efetivamente produzidos e verificados, sem apagar as fontes anteriores.
