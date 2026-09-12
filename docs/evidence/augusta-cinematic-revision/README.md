# Augusta — evidências da revisão cinematográfica

Revisão registrada em 12/09/2026 com o capítulo completo, oito figurinos
pintados, perfis próprios para andar e correr e veículos em escala revisada.
Assista ao [filme do capítulo](film/adventure.mp4) e à
[galeria de movimentos](crowd/crowd-motion.mp4).

| Frente | Evidência disponível | Estado |
| --- | --- | --- |
| Código e empacotador | 610 testes Rust e 35 testes Python do pacote; fmt e Clippy estrito | Aprovado |
| Controles nativos | 12 verificações aprovadas e três capturas inspecionadas | Aprovado na versão com veículos corrigidos |
| Pacote portátil | 15 checks; 394 assets; 32 texturas; quatro folhas e 16 registros | Aprovado na versão com veículos corrigidos |
| Desempenho | 300 frames sem captura em 7,912 s, incluindo abertura/fechamento | Medição anterior à correção dos veículos |
| Filme do capítulo | 137,30 s; 18 planos nomeados; conclusão com 96 HP | Captura completa e auditoria aprovadas |
| Galeria de movimentos | 900 frames; cinco atividades; oito figurinos nos dois sentidos | Captura e revisão de amostras aprovadas |

Os relatórios e capturas nativas foram copiados byte a byte, sem incluir
executáveis ou saves.
[Fontes, hashes e estado da revisão visual](verification-sources.json)
permitem conferir cada cópia. Caminhos temporários e marcas automáticas dos
relatórios originais foram preservados.

SHA-256 do release exercitado pelo roteiro nativo e pacote:
`ea0ef4b3e679f48d8c534f89c014ee026598f5adb42aa23312a70fe609bc7436`.

## Controles e encenação na janela nativa

O [roteiro X11](../../../tools/review/check_augusta_cinema_x11.py) concluiu
**12 verificações em 13,43 segundos**, com código de saída 0 e sem erros X11.
O [relatório original](native/native-checks.json), os
[eventos de entrada](native/events.jsonl), a
[telemetria](native/telemetry.jsonl) e o
[relatório de carregamento](native/load-report.json) estão preservados.

Foram comprovados pausa e retomada do relógio, câmera e elenco na introdução
e na saída dos seguranças; tentativa de Julia antes da conversa; seguranças
inativos durante a tomada; skip entregando três inimigos vivos; impossibilidade
de vencer o combate pelo skip; e permanência no encontro após F5. O teste não
criou o perfil de Rust. As teclas foram enviadas diretamente à janela do
processo, sem mover o ponteiro global ou editar saves.

As três imagens foram inspecionadas depois da geração do relatório:

| Captura nativa | Observação conferida |
| --- | --- |
| [Tentativa de Julia](native/screenshots/01-julia-attempt.png) | Julia atrás do cafetão, com o punho preso, antes do diálogo |
| [Saída dos seguranças](native/screenshots/02-guards-emerge.png) | Aparição pela porta do bar, antes da entrega ao jogador |
| [Entrega ao gameplay](native/screenshots/03-combat-handoff.png) | C++ com os dois braços visíveis e combate legível com o veículo ampliado no primeiro plano |

O campo `visual review pending` permanece intacto no JSON automático; a
revisão posterior da v6 está registrada no manifesto de fontes. O roteiro
termina em `GuardsFight`, sem cobrir interação nativa durante as EPs ou
retomada do save em outro processo. Não houve teste com controle físico.

## Pacote portátil

O release foi montado em `/tmp/augusta-vehicles-portable` com
`tools/release/package.py stage`, para `linux-x86_64`, versão
`0.1.0-prototype.5`. `verify` aprovou arquivos e checksums antes e depois do
smoke. O [registro do build](portable/BUILD-INFO.json) contém **394 assets**.

O launcher foi executado a partir de `/tmp`, sem
`BORROW_FIGHTERS_ASSET_DIR`, com dados de usuário isolados:

```sh
/tmp/augusta-vehicles-portable/borrow-fighters --start augusta --hidden \
  --review /tmp/augusta-vehicles-portable-smoke --frames 90
```

O [relatório de carregamento](portable/load-report.json) registra **32 texturas**
e **65.410.072 bytes RGBA decodificados**. Todos os caminhos pertencem ao
pacote e os arquivos coincidem com as fontes usadas nessa montagem. As quatro
folhas `nightlife-cast-{a,b}.png` e `nightlife-profile-{a,b}.png` foram carregadas;
o catálogo local contém oito registros frontais e oito perfis, com 16 peças
distintas e referências válidas.

As [15 verificações do smoke](portable/portable-checks.json) passaram. O
processo fechou normalmente, com código 0. A
[telemetria](portable/telemetry.jsonl) cobre 90 frames da introdução,
correspondentes a 1,50 segundo e 45 frames de vídeo a 30 fps. Esse teste
comprova abertura e carregamento; não percorre os encontros ou o resgate.
É uma árvore de staging com dependências nativas do host, sem teste de
instaladores ou de outra máquina. A montagem e o smoke incluem a correção
dos veículos; sua escala e circulação também aparecem no filme completo.

## Desempenho sem captura

Os relatórios desta seção antecedem a correção dos veículos; não houve novo
benchmark dessa alteração. O release otimizado medido tem SHA-256
`21e4f093765b567eb4a8f441bc4812abc799086c593fdc82aac7a9c460baa66a`.

A [medição do release otimizado](performance/final-release.json) executou
uma rodada de 300 frames com `--start augusta --hidden --frames 300`, sem
`--capture` ou `--review`. Levou **7,912 segundos de parede** e **3,827 segundos
de CPU**. Os hashes do binário e das fontes permaneceram iguais; não havia
outro jogo, gravador ou compilação durante a medição.

| Revisão medida | Tempo de parede para 300 frames | CPU total por rodada |
| --- | --- | --- |
| [Debug anterior aos perfis](performance/previous-debug.json) | 17,381 s; 19,282 s | 13,568 s; 13,059 s |
| [Release antes da redução de subdivisões](performance/previous-release.json) | 6,464 s; 10,687 s; 10,794 s | 4,285 s; 5,110 s; 5,032 s |
| [Release com redução de subdivisões](performance/final-release.json) | 7,912 s | 3,827 s |

Os tempos incluem inicialização e fechamento. O loop configura limite de
60 fps; o ambiente foi uma janela oculta em WSL/X11, com MSAA 4x. A variação
entre rodadas e a ausência de tempos individuais por frame impedem afirmar
um FPS estável. A primeira estimativa corrigida pelo baseline frio do release
anterior excedeu o limite e foi marcada inválida no próprio relatório.
A comparação debug/release também muda perfis e caches; não isola o ganho
de uma única otimização.

## Filme completo

O [filme com áudio](film/adventure.mp4) percorre o capítulo inteiro em
137,30 segundos, incluindo a permanência na tela final. A
[auditoria narrativa](film/story-audit.json) passou sobre **8.238 frames de
telemetria**, com 18 planos nomeados e conclusão em `Complete`, com C++ em
96 HP.

| Acontecimento | Resultado registrado |
| --- | --- |
| Tentativa de Julia | Ocorre antes da conversa com o cafetão |
| Saída dos seguranças | Os três aparecem pela mesma soleira `[1536, 500]`, inativos durante a tomada |
| Entrega ao gameplay | As posições finais da chegada coincidem com as posições iniciais do combate |
| Fuga do cafetão | Começa no frame 5.099, durante a chegada das EPs, com elas vivas |
| Combate das EPs | Começa no frame 5.495, com o cafetão ausente; ele não retorna no resgate |
| Conclusão | Dois encontros, resgate de Julia e saída acompanhada |

A auditoria usa estado, posições e tempo do runtime. Os vídeos e as capturas
preservam a aparência para inspeção; o resultado automatizado não equivale
à revisão manual de cada frame.

O [registro da mixagem](film/mix-review.json) documenta o áudio reconstruído
pelo [mixer da revisão](../../../tools/review/mix_augusta_review_audio.py) com
as amostras originais do jogo e os eventos observados na telemetria, incluindo
pausa, reinício de sons e redução do tráfego após a chegada das EPs. O vídeo
mantém seus frames; essa faixa não foi capturada do dispositivo Raylib.
Não houve escuta no dispositivo nem verificação de sua latência.

## Galeria dos figurantes

A [galeria de movimentos](crowd/crowd-motion.mp4) contém **900 frames em
30 segundos**, a 30 fps. Cada uma das cinco atividades — caminhada, conversa,
pessoa sentada, telefone e corrida — foi amostrada continuamente nos oito
figurinos e nos dois sentidos.

A [revisão visual e os hashes](crowd/visual-review.json) registram 28 amostras
sequenciais inspecionadas independentemente, além da revisão do responsável
pela integração. Os 900 PNGs da captura v6 são idênticos aos da v5 revisada;
14 frames selecionados acompanham o vídeo nesta pasta. Todos os 900 foram
gerados e comparados por hash, sem afirmar inspeção manual de cada um.

As amostras conferidas mostram tronco, quadril e sapatos de perfil na
locomoção, ombros e mãos conectados, oclusão natural do braço distante e
ausência do recorte estático atrás do ombro do vestido ameixa. A galeria usa
as poses e o renderer do jogo, com posições fixas de exibição para comparar
os figurinos. A circulação pelo bairro está registrada no filme do capítulo.

Direção: [escopo cinematográfico](../../39-augusta-cinematic-direction.md).
Execução e recuperação: [diário](../../worklogs/augusta-cinematic-direction.md).
