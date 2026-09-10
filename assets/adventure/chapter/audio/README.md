# Som do capítulo — Depois do silêncio

Cinco efeitos originais, gerados em 10/09/2026 por
[`generate_audio.py`](generate_audio.py), usando apenas a biblioteca padrão do
Python: ruído determinístico filtrado, senoides breves, envelopes e PCM mono
de 16 bits a 22.050 Hz. Não há gravações, samples externos, vozes, serviços ou
música licenciada. Aplicam-se as licenças do repositório.

| Arquivo | Uso | Duração | Pico | RMS |
|---|---|---:|---:|---:|
| `phone_pocket.wav` | Tecido ao sacar ou guardar o celular | 0,32 s | 0,16 | 0,0330 |
| `phone_tap.wav` | Toque abafado do dedo no vidro | 0,07 s | 0,10 | 0,0155 |
| `phone_send.wav` | Confirmação discreta de envio | 0,18 s | 0,15 | 0,0347 |
| `phone_receive.wav` | Um aviso curto de mensagem recebida | 0,30 s | 0,18 | 0,0458 |
| `footstep.wav` | Contato suave do calçado no passeio | 0,14 s | 0,15 | 0,0272 |

Pico e RMS são normalizados antes do volume de reprodução de **0,3**. Os
arquivos somam aproximadamente 44 KB, com samples nulos nas extremidades;
o timbre e a intensidade devem ser avaliados na cena, por audição humana.

O adaptador [`chapter_audio.rs`](../../../../src/adventure/engine/chapter_audio.rs)
reutiliza o loop `assets/adventure/audio/street_air.wav`, sem carregar trânsito
ou faixa musical. Também reutiliza `strike.wav`, `block.wav`, `hurt.wav`,
`shutter_roll.wav` e `shutter_clack.wav` da aventura; os arquivos originais
continuam intactos.

O telefone compartilha os marcos de `chapter/phone.rs`: tecido nos ticks **0**
e **810**, envio nos **210** e **720**, recebimento no **510**. Três toques
espaçados acompanham cada digitação de Rust (**110/146/184** e **626/660/696**);
a espera e a digitação remota de Python ficam sem teclado local.

A porta usa o rolo de um segundo com pitch `60 / SHOP_SHUTTER_TICKS`, produzindo
**1,5 segundo** nos 90 ticks da abertura e do fechamento. Na volta, a porta
espera os **48 ticks** de `SHOP_EXIT_CLEARANCE_TICKS` para a moradora atravessar;
o rolo começa no **48**, e somente o fechamento termina com contato metálico,
no **138**. Passos seguem dois apoios por ciclo de caminhada
de 28 ticks e exigem chão e deslocamento; bloqueio, salto e imobilidade não
produzem passos. Se um render atravessa vários apoios, toca somente o mais
recente, evitando sons sobrepostos de passos atrasados.

Integração: `ChapterAudio::new(device)`, `update(&chapter, paused)` e
`synchronize(&chapter)` imediatamente após skip, retry ou carregar checkpoint.
Pausa suspende streams e efeitos em curso. Sincronizar descarta efeitos
abandonados, alinha o ar ao relógio do capítulo e preserva a pausa; transições
normais não precisam de sincronização. O observador usa cruzamentos de marcos,
deduplica contatos e ignora eventos anteriores quando o relógio é restaurado.
Ausência de dispositivo ou WAV não impede jogar.

Para regenerar somente estes cinco arquivos:

```sh
python3 assets/adventure/chapter/audio/generate_audio.py
```
