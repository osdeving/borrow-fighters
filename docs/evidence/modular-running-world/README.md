# Mundo modular — evidências

Revisão a partir do checkpoint `6740d6d`. As imagens são do renderer nativo
Rust/Raylib; os vídeos são evidências, não assets reproduzidos pelo jogo.

- [Corrida pela rua e chegada rápida](world/run-and-arrival-native.mp4),
  [corrida no cenário](world/running.png), [impacto](world/impact.png) e
  [medidas do percurso](world/summary.json). [11/11 checks X11 no mundo atual](world/native-checks-full.json) incluem pausa, skip, retry e continuidade.
- [Corrida articulada](gait/README.md): oito segundos nos dois sentidos,
  alternância, suspensão e folha de poses; rig e recaptura reproduzíveis.
- [Capítulo completo](chapter/README.md): 6814 frames, nove destroços
  destruídos e duas EPs derrotadas; câmera dentro dos três mapas.
- [Duke e Old C](biographies/README.md): limousine, reunião ampliada,
  secretária e setup atual, com sobreposições conferidas no renderer.
- [Chegada da EP isolada](ep/README.md): verificação anterior dos comandos
  de pausa, skip, retry e continuidade, além da queda sem pausas.

[Verificações finais](verification.json) registram fmt, Clippy, matriz de
features, fronteiras e dependências de distribuição. A captura do capítulo
usa comandos públicos simulados; a rua/EP usa teclas enviadas à janela
pertencente ao processo. Os vídeos da corrida e do capítulo são silenciosos.
Controle físico e avaliação humana de conforto/dificuldade não foram executados.

As correções durante a revisão incluíram proporção do piso, halos das fachadas,
apoio e recolhimento dos pés, wrap dos figurantes nos limites antigos, câmera
ao alterar spawn e máscara de braços sobre a mesa. Cada correção alterou
dados ou componentes próprios; não foi necessário refazer os cenários inteiros.

[Guia de edição](../../35-modular-running-world.md) ·
[Arte e prompts do mundo](../../../assets/adventure/world/README.md) ·
[Diário](../../worklogs/modular-running-world.md).
