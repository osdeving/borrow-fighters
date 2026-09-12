# Revisão nativa — biografias modulares

Capturas em Raylib 1280×720 com `adventure::engine::biography::draw`, os mesmos
catálogos, PNGs, fontes e interpolação usados no jogo. O harness não altera a
composição nem edita pixels: exporta a textura de renderização e envia quadros
RGBA diretamente ao FFmpeg. O vídeo de 24 segundos/30 fps reproduz todos os
720 ticks de Duke e todos os 720 ticks de Old C, amostrados a cada dois ticks.
É evidência visual sem áudio; a abertura real mantém a trilha compartilhada.

- [Aproximação](duke-approaching.png), [estacionamento](duke-parking.png) e
  [chegada](duke-arrival.png): limousine em 3/4 raso, sem inclinar o sprite;
  profundidade, crescimento e parada são keyframes. Duke emerge pelo lado da
  calçada e vai até a entrada.
- [Reunião](duke-boardroom.png): seis diretores sentados, Duke à cabeceira e
  secretária em pé anotando. Câmera mais afastada; pernas atrás do tampo e
  braços desenhados sobre ele usando contornos por pessoa.
- [Escritório](old-c-workshop.png) e [fundamentos](old-c-foundations.png):
  traseira do monitor moderno voltada à câmera, tela voltada a Old C, teclado
  sob suas mãos. Café, pizza, estante e livros sem letras são peças separadas.
- [Vídeo completo](biographies-native.mp4).

Os três fundos e os sprites originais de Duke/Old C não foram regenerados.
Veja [catálogo, instruções e procedência](../../../../assets/adventure/opening/scenes/README.md).

## Verificação

- [Testes focados](tests.log): sete testes das biografias, incluindo
  trajetória/parada, referências de assets, poses/pessoas, ausência de labels,
  validação de máscaras e intervalos de um contorno côncavo.
- [Clippy da biblioteca adventure](clippy.log), com `-D warnings`.
- `rustfmt --edition 2024 src/adventure/biography.rs src/adventure/engine/biography.rs`.
- Importação nativa dos sete PNGs do catálogo, conferindo limites dos recortes.

## Reprodução

[capture.rs](capture.rs) é o harness usado. Na raiz do repositório, crie um
pacote temporário com esse arquivo em `src/main.rs`, dependências
`borrow-fighters = { path = "/caminho/borrow-fighters", default-features = false,
features = ["adventure"] }` e `raylib = "6.0.0"`. Execute o pacote mantendo a
raiz deste repositório como diretório atual; `--video` também grava o MP4.
Requer um display gráfico funcional e FFmpeg disponível. O harness mantém a
janela oculta e usa a renderização nativa, sem automação de teclado ou tempo real.
