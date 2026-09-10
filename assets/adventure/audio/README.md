# Áudio original da aventura

Sons procedurais criados para a abertura de Rust em 10 de setembro de 2026.
Não contêm samples externos, música licenciada nem locução. A autoria técnica
está no gerador determinístico [`generate_audio.py`](generate_audio.py), que usa
somente a biblioteca padrão do Python: senoides, envelopes, ruído com seed fixa
e exportação `wave`. A apresentação musical de 48 segundos tem gerador próprio,
[`generate_opening_audio.py`](generate_opening_audio.py), sem alterar o gerador
ou os oito WAVs anteriores. Os três efeitos de trânsito têm gerador separado,
[`generate_traffic_audio.py`](generate_traffic_audio.py), também com síntese
original e ruído determinístico, sem gravações externas. Aplicam-se as licenças
do repositório.

| Arquivo | Uso | Duração |
|---|---|---|
| `ada.wav` | Acordes discretos, pequenos mecanismos e sinos na descoberta de Ada | 12 s, loop |
| `morning.wav` | Notas suaves de timbre semelhante a piano elétrico na manhã de Rust | 12 s, loop |
| `threat.wav` | Pulso grave e tensão durante a ameaça | 12 s, loop |
| `remorse.wav` | Notas descendentes suaves durante o gesto de pesar | 12 s, loop |
| `opening.wav` | Apresentação com notícias, heroínas, elenco e chegada musical do logo | 48 s, sem loop |
| `strike.wav` | Contato de Rust com a criatura | 0,22 s |
| `block.wav` | Defesa frontal bem-sucedida | 0,28 s |
| `hurt.wav` | Rust atingido | 0,36 s |
| `transition.wav` | Mudança de cena ou surgimento da ameaça | 0,80 s |
| `car_horn.wav` | Buzina urgente de dois tons quando o carro percebe a EP | 0,60 s |
| `car_skid.wav` | Pneus freando até o contato com o poste | 0,567 s |
| `car_crash.wav` | Colisão grave, lataria amassando e cauda de metal | 1,35 s |

Os WAVs usam PCM mono de 16 bits a 22.050 Hz, com envelopes nas extremidades e
pico limitado antes da conversão. São áudio original de piloto; a qualidade e o
equilíbrio ainda devem ser julgados por audição humana. O adaptador de aventura
usa volume base de 0,3 e mantém o ponto de reprodução quando o jogo é pausado.

A revisão de `morning.wav` remove o ruído aleatório de vento, os pássaros agudos
e o acorde grave contínuo, após relato de chiado pelo usuário. A nova versão usa
somente notas com ataque suave, harmônicos discretos e pico de 0,28. Foi
substituído o WAV externo; reabrir o jogo carrega a mudança sem recompilar.

`opening.wav` é uma composição original a 120 BPM, com baixo pulsado,
percussão sintetizada, arpejos, melodias próprias e crescimento de intensidade.
Os trechos acompanham o relógio de `Stage::Opening`: notícias de 0–9 s, C++ de
9–19 s, Python de 19–29 s e os quatro cartões do elenco de 29–41 s. Aos **41 s**,
um ataque forte e a chegada em ré maior conduzem ao logo e à resolução até
48 s. A faixa toca uma vez, exclusivamente nessa etapa; pausar preserva sua
fase, inclusive o alinhamento com o logo. A intensidade e a resolução musical
devem ser avaliadas por audição humana junto à montagem.

O acidente usa o relógio fixo de `adventure/ambient.rs`: buzina no tick **38**,
frenagem no **78** e impacto no **112**, contados desde o despertar da EP.
O efeito de pneus dura 34/60 s e termina no impacto. Os picos dos três WAVs
são 0,82, 0,73 e 0,94, antes do volume de reprodução de 0,3. Pausa suspende
os sons em execução; retry rearma os marcos e descarta a cauda do acidente
anterior. O avanço por trecho descarta os efeitos abandonados. O adaptador
detecta marcos cruzados entre renders, para não perder ou repetir a buzina,
frenagem ou batida se o número de updates por imagem variar.

Para regenerar exatamente os arquivos desta pasta:

```sh
python3 assets/adventure/audio/generate_audio.py
python3 assets/adventure/audio/generate_opening_audio.py
python3 assets/adventure/audio/generate_traffic_audio.py
```

Dispositivo de áudio ou WAV ausente não impede a aventura. Este conjunto não
depende dos manifestos, sons ou músicas do protótipo de luta.
