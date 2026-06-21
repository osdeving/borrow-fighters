# Audio Assets

Este diretório guarda o manifesto de áudio, clips reais de protótipo e o rastreio de fontes/licenças.

## Manifesto

- Arquivo principal: [`audio_manifest.json`](audio_manifest.json)
- Fontes/licenças: [`ATTRIBUTION.md`](ATTRIBUTION.md)
- Código que carrega: `src/audio/mod.rs`
- Código que toca via Raylib: `src/engine/audio.rs`

O jogo usa eventos de gameplay em vez de chamadas soltas de `play_sound`.
Exemplo: o combate emite `fighter.attack.start` com personagem e golpe; o
manifesto decide qual clip tocar.

## Pastas sugeridas

```text
assets/audio/
├── audio_manifest.json
├── ATTRIBUTION.md
├── music/
├── characters/
│   ├── rust/
│   │   ├── voice/
│   │   └── sfx/
│   └── duke/
│       ├── voice/
│       └── sfx/
└── sfx/
    ├── combat/
    └── match/
```

## Convenções

- `voice`: vozes, esforço, dor, provocação, vitória.
- `sfx`: impactos, defesa, whiff, projéteis, UI, match flow e arena.
- `sfx/match`: anúncios de início, vitória e contagem pré-luta.
- `music`: faixas longas tocadas como stream.
- Clips curtos devem usar `.wav` ou `.ogg`, conforme Raylib carregar melhor no
  ambiente alvo.
- Entradas sem arquivo real podem ser opcionais, mas o manifesto atual aponta
  para arquivos reais de protótipo.
- Quando um clip virar obrigatório para uma build ou showcase, marque
  `"required": true` no manifesto.
- Todo asset baixado de terceiros precisa aparecer em [`ATTRIBUTION.md`](ATTRIBUTION.md)
  antes de ir para PR.
