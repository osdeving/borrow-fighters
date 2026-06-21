# ADR 0005 — Eventos de audio data-driven

## Status

Proposto.

## Contexto

O jogo precisa receber vozes, sons de dor, golpes, defesa, whiff, projeteis, UI, vitoria e ambientes.

Se cada sistema chamar diretamente um arquivo de audio, o codigo rapidamente fica acoplado a nomes de assets e dificil de ajustar por artistas. Ao mesmo tempo, integrar um middleware completo de audio agora seria cedo demais para o Prototype 0.1.

## Decisao

Criar um motor leve de audio por eventos e faixas nomeadas:

- `src/audio/mod.rs` define `AudioCue`, `AudioEvent`, schema de manifesto e matching de bindings;
- `src/engine/audio.rs` e a unica camada que conhece Raylib para carregar e tocar `Sound`;
- musicas de fundo usam `Music` streaming do Raylib e `MusicTrack` com chaves estaveis;
- gameplay emite eventos como `fighter.attack.start`, `combat.hit` e `fighter.hurt`;
- UI emite eventos como `ui.navigate`, `ui.confirm` e `ui.back` pela camada de app;
- `assets/audio/audio_manifest.json` associa cue, personagem, golpe e futuramente ambiente aos clips e tambem lista faixas de musica;
- clips ausentes sao opcionais por padrao para permitir planejamento antes dos assets finais.

O binding mais especifico vence. Por exemplo, `fighter.attack.start + rust + rust_borrow_jab` vence um binding generico de `fighter.attack.start`.

## Alternativas consideradas

### Chamar `play_sound` direto no combate

Pros:

- mais rapido para o primeiro som.

Contras:

- mistura regra de jogo com arquivo de asset;
- dificulta variacao por personagem e golpe;
- atrapalha artistas, porque todo ajuste exigiria mexer no codigo.

### Integrar FMOD ou Wwise agora

Pros:

- authoring profissional, bancos, eventos, parametros e mistura avancada.

Contras:

- aumenta setup e complexidade cedo demais;
- nao e necessario antes de termos volume real de assets e requisitos de mixagem.

### Manifesto JSON simples sobre Raylib

Pros:

- combina com o pipeline de sprites ja usado no projeto;
- testavel sem abrir janela;
- permite associar audio a personagem, golpe e ambiente;
- mantem caminho aberto para middleware futuro.

Contras:

- ainda nao oferece authoring visual, snapshots, states ou mixagem por bus em runtime;
- exigira disciplina para manter chaves e docs sincronizadas.

## Consequencias

### Positivas

- Gameplay nao conhece arquivos de audio.
- Artistas/devs podem planejar clips no manifesto.
- Clips placeholders nao quebram o jogo.
- Musica longa fica separada de SFX curto e atualizada por stream.
- Testes validam chaves conhecidas e precedencia de bindings.

### Negativas

- Mais um manifesto para manter.
- Ainda falta controle de volume por bus.
- O manifesto pode crescer e exigir validação extra.

## Criterio de revisao

Revisar se:

- precisarmos de musica/ambiencia longa com streaming;
- houver muitos clips por personagem e variação;
- volume por bus virar preferencia de usuario;
- Raylib nao atender mixagem, estados, bancos ou authoring;
- a equipe tiver workflow maduro para FMOD/Wwise.
