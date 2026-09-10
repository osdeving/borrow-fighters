# Áudio original da aventura

Sons procedurais criados para a abertura de Rust em 10 de setembro de 2026.
Não contêm samples externos, música licenciada nem locução. A autoria técnica
está no gerador determinístico [`generate_audio.py`](generate_audio.py), que usa
somente a biblioteca padrão do Python: senoides, envelopes, ruído com seed fixa
e exportação `wave`. Aplicam-se as licenças do repositório.

| Arquivo | Uso | Duração |
|---|---|---|
| `ada.wav` | Acordes discretos, pequenos mecanismos e sinos na descoberta de Ada | 12 s, loop |
| `morning.wav` | Acorde leve, ar e pássaros sintetizados na manhã de Rust | 12 s, loop |
| `threat.wav` | Pulso grave e tensão durante a ameaça | 12 s, loop |
| `remorse.wav` | Notas descendentes suaves durante o gesto de pesar | 12 s, loop |
| `strike.wav` | Contato de Rust com a criatura | 0,22 s |
| `block.wav` | Defesa frontal bem-sucedida | 0,28 s |
| `hurt.wav` | Rust atingido | 0,36 s |
| `transition.wav` | Mudança de cena ou surgimento da ameaça | 0,80 s |

Os WAVs usam PCM mono de 16 bits a 22.050 Hz, com envelopes nas extremidades e
pico limitado antes da conversão. São áudio original de piloto; a qualidade e o
equilíbrio ainda devem ser julgados por audição humana. O adaptador de aventura
usa volume base de 0,3 e mantém o ponto de reprodução quando o jogo é pausado.

Para regenerar exatamente os arquivos desta pasta:

```sh
python3 assets/adventure/audio/generate_audio.py
```

Dispositivo de áudio ou WAV ausente não impede a aventura. Este conjunto não
depende dos manifestos, sons ou músicas do protótipo de luta.
