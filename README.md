# itch

Leitor do feed binário Nasdaq TotalView-ITCH em Rust, para reconstruir o order book. Projeto de estudo, com foco em performance, memória e latência.

## Dados

Os arquivos de pregão ficam em `data/`, que não vai para o git. Cada arquivo comprimido tem de 3,5 a 5,6 GB, e descomprimido é bem maior.

Para baixar o menor arquivo disponível (30/12/2019, 3,52 GB), a partir da raiz do repositório:

```bash
mkdir -p data && cd data
BASE='https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH'
curl -fL -C - -O "$BASE/12302019.NASDAQ_ITCH50.gz"
gzip -t 12302019.NASDAQ_ITCH50.gz && echo "arquivo íntegro"
```

O `-C -` retoma o download se a conexão cair. O `gzip -t` confere a integridade sem gravar nada em disco, mas leva alguns minutos. Os arquivos `.md5sum` aparecem na listagem, mas o download deles devolve 404. Outros dias estão em <https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/>.

Para criar uma amostra de 100 MB sem descomprimir o arquivo inteiro:

```bash
gzip -dc 12302019.NASDAQ_ITCH50.gz | head -c 100M > amostra.bin
```

O corte é por bytes, então a última mensagem pode ficar incompleta.

## Especificação

A especificação está em [docs/spec/NQTVITCHspecification.pdf](docs/spec/NQTVITCHspecification.pdf). A versão mais recente fica no [site da Nasdaq](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf).
