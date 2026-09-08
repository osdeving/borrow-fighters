# Controles no App real — especiais e arremessos

[Resultado](result.json): **25 verificações aprovadas**, com o binário de debug real em Xephyr, GL por software, atlas candidatos habilitados e ambos os jogadores controlados pelo teclado. A janela pai do Xephyr permaneceu sem mapear; os eventos não foram enviados ao desktop do usuário. O JSON registra o hash do binário e os tempos observados das capturas.

O teste abriu `--showcase --character rust --move throw --repeat --reverse`. Verificou pausa, avanço de frame, replay, `Tab`/`Shift+Tab`, `L`, `X`, `PgDown`/`PgUp` preservando o cenário de defesa e `Esc`. Depois desativou as CPUs em Options e iniciou lutas reais, reiniciando cada caso para preservar vida e distância inicial.

`Q+F` arremessou Java e `U+O` arremessou Rust, com dano, captura, voo, aterrissagem e troca de lado observados. `T` ativou Borrow Fortress e `\` ativou System.out.println!, ambos causando dano. Defesa de jab e rasteira também foram exercitadas. Debug desligado não desenhou caixas durante essas ações; ligar e desligar Options alterou os overlays sem reiniciar. O fechamento nativo da janela encerrou o processo com código zero.

| Evidência selecionada | Observação |
|---|---|
| [Arremesso P1](p1-throw-flight.png) | Java em voo após `Q+F`. |
| [Arremesso P2](p2-throw-flight.png) | Rust em voo após `U+O`. |
| [Debug ligado](debug-on.png) | Hurtboxes e limite lateral visíveis. |
| [Debug desligado](debug-off.png) | Mesma luta, overlays removidos por Options. |

O [resultado inicial do detector](initial-detector-result.json) marcou uma falha ao procurar o limite lateral exclusivamente em `x=42`. A linha estava visível em `x=41`; a análise dos mesmos PNGs em `x=40..43` encontrou 395 pixels com debug ligado e zero desligado. Isso corrigiu a medição do harness, sem alterar o aplicativo nem refazer capturas. O resultado final registra essa revisão.

Não havia gamepad físico: RT é coberto pelos testes de mapeamento, não por este ensaio. As capturas por relógio real comprovam integração e leitura dos estados amostrados; os testes de combate verificam separadamente cada frame, cantos, KO e recuperação. Esta rodada não substitui a revisão final dos pivôs e efeitos; os atlas podem receber ajustes visuais posteriores sem mudar os bindings testados.

## Complemento: vítima junto ao canto

Após o ajuste do limite visual, outra execução isolada do App repetiu `U+O` contra Rust perto da borda direita. A [aterrissagem capturada](rust-corner-land.png) mantém o personagem inteiro visível, inclusive cabeça e pés, sem fazê-lo voltar a uma pose em pé. O [resultado direcionado](corner-result.json) confirma dano, ausência de caixas com debug desligado nas três amostras de voo/queda/pouso e encerramento normal. Essa inspeção complementa os 25 checks acima; não repete a suíte completa.
