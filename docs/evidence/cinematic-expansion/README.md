# Expansão cinematográfica — evidências

Implementação de 12/09/2026. As capturas vêm do renderer Rust/Raylib, com os
mesmos assets e dados carregados pelo jogo.

- [Chegada da EP](ep/README.md): câmera alta, queda no plano jogável, impacto,
  poeira e recuperação; **11/11 verificações com comandos de teclado X11**.
- [Caminhada de Rust](gait/rust-walk-native.mp4): saída do quarto e movimento
  nos dois sentidos. [Oito apoios](gait/walk-poses-native.png) e
  [preparação/contato/recuperação do chute](gait/kick-poses-native.png).
- [Prévia das biografias](opening/biographies-native.mp4), [Duke chegando à Paulista](opening/screenshots/duke-paulista.png),
  [na cabeceira](opening/screenshots/duke-boardroom.png),
  [setup de Old C](opening/screenshots/old-c-workshop.png) e
  [fundamentos](opening/screenshots/old-c-foundations.png).
- [Capítulo](chapter/README.md): carga intacta, destruição por contato, caminho
  aberto e dois inimigos. O percurso automático terminou com as duas EPs
  derrotadas, nove caixas destruídas e checkpoint final; 14 contatos na carga.

Os vídeos de caminhada são silenciosos. A prévia da EP usa imagens nativas
com áudio reconstruído pelos marcos da telemetria, conforme seu README.
As biografias usam a trilha original de 64 segundos; o código nunca reproduz
um vídeo pronto como substituto das cenas.

## Verificação

[Registro dos checks](verification.json): fmt e Clippy estrito; testes com
todas as features, aventura/luta isoladas e core; 56 fixtures de fronteiras,
20 testes de pacote e 48 do mixer. Dois testes preexistentes de dispositivo
de áudio permanecem ignorados. Os pacotes seguem as dependências transitivas
dos catálogos, incluindo cada pose e os novos assets narrativos.

Os testes adicionais cobrem: continuidade da queda, pausa/skip/retry, recarga
validada, apoios/stride, janela do chute, contato consumido uma vez, caixas
perdendo suporte, bloqueio de pulos repetidos na pilha intacta, duas barras,
vitória do grupo e restauração do checkpoint.

Não houve teste com controle físico, medição de latência sonora ou validação
humana do conforto/dificuldade. As capturas e a revisão visual automatizada
permitem avaliar os encaixes; o grau final de naturalidade continua subjetivo.

[Escopo e arquivos editáveis](../../34-cinematic-expansion.md) ·
[Diário de recuperação](../../worklogs/cinematic-expansion.md).
