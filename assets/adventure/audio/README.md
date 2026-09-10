# Áudio original da aventura

Sons procedurais criados para a abertura de Rust em 10 de setembro de 2026.
Não contêm samples externos, música licenciada nem locução. A autoria técnica
está no gerador determinístico [`generate_audio.py`](generate_audio.py), que usa
somente a biblioteca padrão do Python: senoides, envelopes, ruído com seed fixa
e exportação `wave`. A apresentação musical de 48 segundos tem gerador próprio,
[`generate_opening_audio.py`](generate_opening_audio.py), sem alterar o gerador
ou os oito WAVs anteriores. Os três efeitos de trânsito têm gerador separado,
[`generate_traffic_audio.py`](generate_traffic_audio.py), também com síntese
original e ruído determinístico, sem gravações externas. A fuga coletiva usa
[`generate_evacuation_audio.py`](generate_evacuation_audio.py) para criar motores
acelerando e bicicletas caindo, com o mesmo formato e sem alterar os sons do
acidente. Os ambientes naturais e os efeitos da vizinhança vêm de
[`generate_neighbourhood_audio.py`](generate_neighbourhood_audio.py), também
inteiramente original e determinístico, usando somente a biblioteca padrão.
Aplicam-se as licenças
do repositório.

| Arquivo | Uso | Duração |
|---|---|---|
| `ada.wav` | Acordes discretos, pequenos mecanismos e sinos na descoberta de Ada | 12 s, loop |
| `morning.wav` | Versão musical anterior preservada; fora do roteamento atual | 12 s |
| `threat.wav` | Versão anterior da ameaça preservada; fora do roteamento atual | 12 s |
| `morning_ambience.wav` | Ar discreto e poucos pássaros distantes no quarto | 20 s, loop |
| `street_air.wav` | Ar contínuo da rua, inclusive durante o combate; sem motores | 20 s, loop |
| `street_traffic.wav` | Motores distantes e pneus em passagens espaçadas, sem buzinas | 20 s, loop independente |
| `remorse.wav` | Notas descendentes suaves durante o gesto de pesar | 12 s, loop |
| `opening.wav` | Apresentação com notícias, heroínas, elenco e chegada musical do logo | 48 s, sem loop |
| `strike.wav` | Contato de Rust com a criatura | 0,22 s |
| `block.wav` | Defesa frontal bem-sucedida | 0,28 s |
| `hurt.wav` | Rust atingido | 0,36 s |
| `transition.wav` | Mudança de cena ou surgimento da ameaça | 0,80 s |
| `traffic_escape.wav` | Motores aceleram e se afastam ao surgir a EP | 1,40 s |
| `bicycle_fall.wav` | Tubos leves e peças das bicicletas batem no piso | 0,90 s |
| `car_horn.wav` | Buzina urgente de dois tons quando o carro percebe a EP | 0,60 s |
| `car_skid.wav` | Pneus freando até o contato com o poste | 0,567 s |
| `car_crash.wav` | Colisão grave, lataria amassando e cauda de metal | 1,35 s |
| `dog_alert.wav` | Um latido curto do caramelo ao perceber a EP | 0,30 s |
| `shutter_roll.wav` | Chapa corrugada descendo pelos trilhos | 1,00 s |
| `shutter_clack.wav` | Contato final da porta com o piso | 0,36 s |

Os WAVs usam PCM mono de 16 bits a 22.050 Hz, com envelopes nas extremidades e
pico limitado antes da conversão. São áudio original de piloto; a qualidade e o
equilíbrio ainda devem ser julgados por audição humana. O adaptador de aventura
usa volume base de 0,3 e mantém o ponto de reprodução quando o jogo é pausado.

A versão histórica de `morning.wav` havia substituído vento e pássaros agudos
por notas suaves, após relato de chiado. A nova direção usa outro arquivo,
`morning_ambience.wav`: ar filtrado de baixo volume e seis pequenos cantos
espaçados em vinte segundos, sem melodia ou drone. O arquivo anterior permanece
preservado. A rua recebe apenas ar e trânsito cotidiano durante a chegada da
câmera e a exploração, sem depender do relógio do combate. Despertar a EP ou
perder a luta não seleciona música leve nem reinicia o ambiente de ar.

O volume relativo do trânsito é `(1 - idade_da_reação / 360)²`, limitado a
0–1. Aos seis segundos de evacuação, seu stream é parado; não volta durante
o combate, derrota ou pesar. Retry inicia uma nova evacuação. O ar segue em
stream separado; no pesar, a faixa `remorse.wav` conserva sua função anterior.
Pausa congela as duas camadas, e avançar a câmera busca o ponto do relógio de
`AmbientState`, sem inventar fuga ou reiniciar sons pendentes.

| Novo WAV | Pico antes do volume 0,3 | RMS normalizado |
|---|---:|---:|
| `morning_ambience.wav` | 0,11 | 0,0222 |
| `street_air.wav` | 0,07 | 0,0154 |
| `street_traffic.wav` | 0,29 | 0,0376 |
| `dog_alert.wav` | 0,29 | 0,0760 |
| `shutter_roll.wav` | 0,39 | 0,0842 |
| `shutter_clack.wav` | 0,48 | 0,0480 |

Os novos loops usam extremos suaves de 350 ms e primeiro/último samples nulos.
São aproximações sintetizadas de fontes naturais e mecânicas; os valores acima
documentam o sinal, sem substituir julgamento humano do timbre e do equilíbrio.

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

A fuga coletiva começa com `traffic_escape.wav` no tick **0** da reação;
`bicycle_fall.wav` toca uma única vez no **70**, quando as bicicletas terminam
de tombar no piso. O abandono começa no **24**; os ciclistas passam a correr no
**54**, e as bicicletas tombam durante os 16 ticks seguintes. Os picos são
0,58 e 0,50, abaixo da colisão no
poste. Ambos seguem as mesmas regras de pausa, retry e descarte por avanço de
cena. São efeitos curtos, sem repetição depois que a rua esvazia.

O caramelo emite um único `dog_alert.wav` no tick **8**. A porta começa a
descer com `shutter_roll.wav` no **270** e encosta com `shutter_clack.wav` no
**330**. Os marcos são importados de `adventure/neighborhood.rs`, compartilhados
com a animação; um segundo de rolo corresponde exatamente aos sessenta ticks
de fechamento. Os três efeitos seguem o mesmo observador de cruzamento,
pausa, retry e descarte por avanço dos efeitos do acidente, sem loops.

Para regenerar exatamente os arquivos desta pasta:

```sh
python3 assets/adventure/audio/generate_audio.py
python3 assets/adventure/audio/generate_opening_audio.py
python3 assets/adventure/audio/generate_traffic_audio.py
python3 assets/adventure/audio/generate_evacuation_audio.py
python3 assets/adventure/audio/generate_neighbourhood_audio.py
```

Dispositivo de áudio ou WAV ausente não impede a aventura. Este conjunto não
depende dos manifestos, sons ou músicas do protótipo de luta.
