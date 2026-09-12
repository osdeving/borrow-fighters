# Corrida, mundo modular e revisão das apresentações

- Branch: `main`.
- Base preservada: `6740d6d`, criado antes das alterações conforme pedido.
- Estado: implementação e verificação concluídas; alterações desta revisão permanecem no workspace, sem novo commit ou push.

## Entrega

- Mundo do prólogo com 4608 px, spawn x160, trigger x3148 e EP em x3548; capítulo com cenas de 4608, 4096 e 3840 px.
- Seis fachadas em PNGs separados, parallax distante, chão/canteiro da pintura original em peças repetíveis. Catálogo, instâncias e limites externos.
- Corrida a 455 px/s com aceleração/freio, rig de nove peças, apoios por distância e braços/pés interpolados. Caminhada cuidadosa e chute preservados.
- Queda da EP em 58 ticks, câmera ampla antes do contato e impacto que inicia o tumulto. Cartão solo de Rust retirado; abertura de 60 s, logo em 53 s.
- Old C com monitor moderno voltado para ele, teclado, café, pizza, estante e livros sem lettering. Duke com limousine alinhada à rua, seis diretores e secretária; contornos dos braços sobre a mesa editáveis no track.

## Correções encontradas na revisão

- Halo magenta e piso esticado: corrigidos na importação e na escala dos tiles, sem regenerar o cenário.
- Bordas opostas de PNGs aparecendo durante zoom: clamp de textura e RGB preto sob alpha zero.
- Figurantes fazendo wrap no meio do mapa: AmbientState recebe limites locais `[-hub, width-hub]` e só encerra trajetos fora do mundo.
- Spawn editado deslocando o foco da pipa: câmera compartilhada por renderização/telemetria desconta o deslocamento atual e respeita os limites.
- Pernas sobre a mesa e cortes horizontais nas camisas: máscara poligonal normalizada mantém apenas os braços sobre o tampo.
- Uma tentativa de revisão abriu um binário antigo com o novo schema de máscara e falhou na validação. A captura foi repetida com o binário recompilado: 11/11 checks passaram.

## Verificação final

- `cargo fmt --check`: passou.
- `cargo clippy --all-targets --all-features -- -D warnings`: passou.
- `cargo test --all-targets --all-features`: 529 passaram; 2 ignorados preexistentes.
- Aventura isolada: 133 passaram; luta isolada: 394 passaram e 2 ignorados; core: 3 passaram.
- Fronteiras de domínio: passou; 56 fixtures passaram.
- Pacote: 20 testes passaram; inclui mapa, fachadas, camadas e rig com todos os seus PNGs.
- Mixer: 48 testes passaram.
- EP no mundo novo: 11/11 checks X11, mais 3/3 na prévia sem pausas; queda de 0,97 s.
- Capítulo nativo: Complete/Victory em 6814 frames, 9 destroços destruídos, 2 EPs derrotadas e câmera dentro dos limites.
- Corrida: vídeo nativo de 8 s; biografias: vídeo nativo de 24 s. FFmpeg concluiu e decodificou os arquivos.

Os comandos e resultados ficam em [verification.json](../evidence/modular-running-world/verification.json). [Evidências](../evidence/modular-running-world/README.md) e [guia de edição](../35-modular-running-world.md) apontam para assets e dados.

## Recuperação

Inspecionar `git status` e o diff desde `6740d6d` antes de repetir trabalho. Os agentes concluíram seus escopos. Não regenerar as cenas para trocar posições: editar os JSONs indicados no guia; reabrir a sessão após mudanças no mapa. F5 recarrega textos, biografias e animação.
