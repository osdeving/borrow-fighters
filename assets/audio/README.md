# Audio Assets

Este diretório guarda o manifesto de áudio, clips reais de protótipo e o rastreio de fontes/licenças.

## Manifesto

- Arquivo principal: [`audio_manifest.json`](audio_manifest.json)
- Fontes/licenças: [`ATTRIBUTION.md`](ATTRIBUTION.md)
- Audição de vozes e efeitos: [`audition.html`](audition.html)
- Fontes, recortes, mixagens e hashes: [`production-2026-09-09.json`](production-2026-09-09.json)
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
│   ├── duke/
│   │   ├── voice/
│   │   └── sfx/
│   ├── go/
│   │   └── voice/
│   ├── c/
│   │   └── voice/
│   ├── python/
│   │   └── voice/
│   └── cpp/
│       └── voice/
└── sfx/
    ├── combat/
    ├── match/
    └── super/
```

## Convenções

- `voice`: vozes, esforço, dor, provocação, vitória.
- `sfx`: impactos, defesa, whiff, projéteis, UI, match flow e arena.
- `sfx/match`: anúncios de início, vitória e contagem pré-luta.
- `sfx/super`: entradas com voz e efeitos específicos das sequências autorais; a música fica pausada enquanto esses eventos tocam.
- `review`: reel de audição com os volumes e pitches do manifesto; não é carregado pelo jogo.
- `music`: faixas longas tocadas como stream; menu, Combat Lab e arenas podem apontar para faixas diferentes.
- Clips curtos devem usar `.wav` ou `.ogg`, conforme Raylib carregar melhor no
  ambiente alvo.
- Entradas sem arquivo real podem ser opcionais, mas o manifesto atual aponta
  para arquivos reais de protótipo.
- Quando um clip virar obrigatório para uma build ou showcase, marque
  `"required": true` no manifesto.
- Todo asset baixado de terceiros precisa aparecer em [`ATTRIBUTION.md`](ATTRIBUTION.md)
  antes de ir para PR.
