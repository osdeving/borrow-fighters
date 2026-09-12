# Biografias — revisão cinematográfica

Quatro pinturas completas substituem as montagens de sprites de Duke e Old C,
conforme a [ADR 0030](../../../adr/0030-painted-biography-shots.md). A curadoria
comparou diretamente as artes de Ada, Python e C++, depois conferiu estes
frames no renderer nativo com legenda, nomes, controles, letterbox e transições.

- [Duke / Paulista](duke-paulista.png): limousine estacionada paralela à guia,
  chauffeur segurando a porta e Duke visível na calçada; contato e perspectiva
  coerentes. O quadro comunica a chegada sem deslizar o carro pela rua.
- [Duke / reunião](duke-boardroom.png): mesa horizontal natural, seis executivos
  sentados, Duke proporcional na cabeceira e secretária anotando. Não há pés
  sobre o tampo nem cortes retangulares em braços ou roupas.
- [Old C / oficina](old-c-workshop.png): monitor moderno com traseira para a
  câmera, teclado sob as mãos, rosto, café e pizza visíveis; livros sem rótulos.
- [Old C / fundamentos](old-c-foundations.png): outro enquadramento e ação de
  estudo, com rosto, lápis e livro em destaque; mesma roupa, identidade e sala.
- [Antes do corte](duke-before-cut.png) e [durante a entrada](duke-crossing-cut.png)
  mostram a transição entre as duas tomadas de Duke.
- [Vídeo nativo completo](biographies-native.mp4): 24 segundos, 30 fps, cobrindo
  as duas biografias e suas transições. Vídeo de revisão sem áudio; o jogo
  continua usando a trilha da abertura.

O nome e a legenda ficam na região inferior. As capturas confirmam que não
ocultam rosto, mãos, pizza ou as pessoas da reunião. A câmera interpola movimentos
pequenos e mantém a pintura cobrindo o viewport. Os quatro arquivos originais
são independentes: uma tomada pode ser substituída sem refazer outra.

## Verificação

- [Cinco testes focados](tests.log): quatro imagens distintas, limites das
  tomadas, 720 ticks por personagem, câmera suave sem bordas vazias e rejeição
  transacional de dados inválidos ou do modelo antigo de camadas.
- [Clippy da biblioteca adventure](clippy.log) com `-D warnings`.
- `rustfmt --edition 2024 src/adventure/biography.rs src/adventure/engine/biography.rs`.
- Catálogo runtime com somente quatro imagens; nenhuma textura da montagem
  anterior é carregada por ele.
- Procedência, prompts e SHA-256 no
  [README dos assets](../../../../assets/adventure/opening/scenes/README.md).

## Reprodução

[capture.rs](capture.rs) é o harness usado. Chama `engine::opening::draw` com
`Story.stage = Opening` e o relógio correspondente a Duke/Old C. Portanto usa
as mesmas imagens, câmera, UI e transições do jogo, sem substituir composição
ou editar os pixels resultantes.

Na raiz do repositório, crie um pacote temporário com esse arquivo em
`src/main.rs`, dependências `borrow-fighters = { path = "/caminho/borrow-fighters",
default-features = false, features = ["adventure"] }` e `raylib = "6.0.0"`.
Execute mantendo a raiz do repositório como diretório atual; `--video` também
grava o MP4 via FFmpeg, enviando 720 quadros RGBA diretamente do render target.
Requer display gráfico funcional. A janela permanece oculta durante a captura.
