# Rust projectile — reaproveitamento com alpha limpo

`source-v1.png` é uma cópia preservada de
`assets/placeholder/rust-gear-projectile.png`, extraído originalmente da referência
`assets/references/sprinte-rust.png`. **Esta entrega é limpeza de asset existente,
não nova geração de imagem.** A autorização do usuário para remoção local de
fundo foi recuperada da sessão anterior.

O PNG tinha uma franja neutra branca/cinza em volta da engrenagem e das linhas
laranjas. `clean_alpha.py` remove somente pixels claros neutros conectados ao
exterior transparente. Os 13.177 pixels não transparentes retidos mantêm RGB e
alpha exatamente iguais à fonte. Foram removidos 3.768 pixels de matte. O R,
contorno escuro, preenchimento laranja e trilhas coloridas não foram repintados.
Os 23 pixels neutros restantes estão encerrados dentro das trilhas, preservados.

O PNG preparado é `cleaned-v1.png`, copiado para
`assets/candidates/rust/rust-projectile.png`. A fonte tem 229 × 108 px e o candidato
tem 237 × 116 px, com margem transparente de 4 px em cada lado. Nenhum desenho
foi ampliado ou reduzido. A fórmula atual do renderer aplica escala 0,6 e mantém
a textura centrada: `(p+4) − (size+8)/2 = p − size/2`. Portanto, a margem altera
apenas o canvas de 137,4 × 64,8 para 142,2 × 69,6 px de runtime; cada pixel retido
mantém a mesma posição, tamanho e centro visual. As bordas do efeito original
não foram reconstruídas artificialmente.

Origem preservada para Right: (+138,667; −117,333) px relativos ao centro inferior
do lutador. ProjectileSpec, origem, colisão, atlas do corpo e metadados de combate
não foram modificados.

## Reprodução

```bash
python3 assets/production/rust/projectile/clean_alpha.py assets/production/rust/projectile/source-v1.png assets/production/rust/projectile/cleaned-v1.png
cp assets/production/rust/projectile/cleaned-v1.png assets/candidates/rust/rust-projectile.png
```

Requer Pillow e NumPy. `cleaned-v1.json` registra os parâmetros exatos;
`production.json` acrescenta hashes, candidato e estado da verificação.

`alpha-review-v1.png` compara original/limpo/espelhado em fundo escuro e claro,
tanto na escala de runtime quanto na resolução da fonte. O halo exterior deixou
de ser visível; a identidade e a energia laranja continuam legíveis. O símbolo R
ainda se inverte ao espelhar, conforme o contrato atual do renderer.

Verificação desta subtask: inspeção estática nas duas orientações, alpha real,
margem simétrica vazia, invariância de RGB/alpha retidos e transformação do
centro. A recaptura nativa do especial após carregar o candidato deve ser feita
pelo responsável pela integração do piloto. Não houve nova geração nem testes
de gameplay nesta limpeza.
