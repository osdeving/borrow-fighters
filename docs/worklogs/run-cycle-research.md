# Pesquisa de corrida: autoria, deformação e contato

2026-09-12 · branch `main` · tarefa limitada a pesquisa; sem código, assets ou commit.

As cinco fontes primárias abaixo foram lidas diretamente. A conclusão de aplicação
ao projeto é nossa: a qualidade da corrida depende de poses e desenhos preparados
para movimento, além da sincronização dos pés. Correções procedurais trabalham
sobre essa base. Os exemplos não estabelecem uma única técnica obrigatória para
todos os grandes jogos.

| Fonte primária | Evidência específica | O que resolve |
| --- | --- | --- |
| [Epic — Distance Matching](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine), seções Overview e Curve Generation | Uma curva relaciona distância a poses de uma sequência existente; o exemplo escolhe a pose de aterrissagem pela distância restante até o chão. A documentação também indica curvas horizontais para partida, parada e mudança de direção. | Seleção e cadência das poses conforme o deslocamento. Não cria o desenho ou a atuação do corpo. |
| [Epic — Pose Warping](https://dev.epicgames.com/documentation/en-us/unreal-engine/pose-warping-in-unreal-engine), Stride Warping e Adding Necessary AnimBP Nodes | Ajusta a separação dos pés à velocidade usando animação, ossos e IK. Oferece limites de escala, compensação da coxa e proteção contra extensão excessiva; Leg IK aplica os alvos ajustados ao esqueleto. | Adaptação limitada de uma passada existente à locomoção efetiva. |
| [Thomas Vasseur, Motion Twin — pipeline de Dead Cells](https://www.gamedeveloper.com/production/art-design-deep-dive-using-a-3d-pipeline-for-2d-animation-in-i-dead-cells-i-), What e Result, 25/01/2018 | O próprio artista descreve construir personagem/esqueleto 3D a partir de model sheet, validar poses-chave e timing e exportar quadros PNG com normal maps. As revisões acontecem no rig e na timeline. O trecho sobre interpolação antes/depois das poses refere-se especialmente aos ataques; não fornece uma receita específica de corrida. | Fonte editável e reaproveitável para produzir sprites; facilita retakes sem redesenhar toda a sequência. |
| [Esoteric Software — Spine Weights](https://esotericsoftware.com/spine-weights), Weights, Triangle order e Testing weights | Vértices recebem pesos de ossos e deformam a imagem. A ordem dos ossos pode determinar a sobreposição dos triângulos. O guia recomenda testar a deformação em toda a amplitude da animação. | Deformação localizada e controle de auto-oclusão em arte 2D. |
| [Esoteric Software — Spine IK constraints](https://esotericsoftware.com/spine-ik-constraints), introdução, Stretch, Softness e Mix | IK resolve rotações até um alvo e pode manter pés acima do piso. Alongamento é uma opção explícita; softness reduz a mudança brusca perto do alcance máximo. A influência de IK pode variar ao longo da animação. | Controle de contato e alcance; transição entre movimento autorado e restrições. |

Estas são três responsabilidades diferentes. A animação-base define poses,
transferência de peso, ritmo e silhueta. A malha ou o conjunto de sprites define
como o desenho responde às poses. Distance matching, warping e IK adaptam a
reprodução e o contato ao movimento controlado pelo jogo. A documentação da Epic
pressupõe sequências de animação; o relato de Vasseur descreve trabalho autorado,
mesmo com interpolação e renderização automatizadas.

Para o Rust 2D, recomendamos a seguinte aplicação, sujeita à revisão visual:

1. Criar um ciclo lateral de corrida separado da caminhada, com contato,
   compressão, impulso, voo e recuperação reconhecíveis. Aprovar as poses
   extremas e a silhueta no tamanho usado pelo jogo antes de adicionar detalhe.
2. No rig modular atual, preservar as botas como peças rígidas e dar à calça
   deformação localizada na articulação. Pesos, pivôs e ordem das partes devem
   permanecer em dados. Avaliar joelhos completamente dobrados: multiplicar
   pregas ou sobrepor retângulos não garante uma articulação legível. Uma malha
   também precisa de revisão; não corrige automaticamente um desenho inadequado.
3. Manter a fase ligada ao deslocamento e registrar os intervalos de apoio.
   Aplicar correções pequenas de contato durante o apoio e liberar o pé no voo.
   Limitar mudanças de comprimento e impedir que a correção apague a pose
   principal. Compartilhar esse ciclo entre gameplay e cenas.
4. Validar em vídeo a velocidade real: bota apoiada sem deslizar, ausência de
   esticamento nos joelhos, leitura das duas pernas em flexão, entrada/saída da
   corrida e inversão de direção. Uma imagem isolada não comprova esses pontos.

Um pipeline 3D→sprites como o de Dead Cells é uma alternativa de produção futura,
com investimento em modelo, rig e render que preserve nosso estilo. A lição
imediata é manter uma fonte de animação editável e retakes localizados. Estas
fontes não justificam trocar a engine nem introduzir ECS para resolver a corrida.

Verificação: leitura das cinco páginas canônicas com acesso web; links diretos
na tabela. Nenhum teste Rust foi executado por esta subtarefa de pesquisa.
