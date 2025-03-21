## About
`load-calculator` is a REST API to easily calculate the data storage cost on [Load Network](https://load.network) -- Load's data storage comes along with high throughput DA and data permanence backup -- [learn more about Load Network](https://docs.load.network)

Let me fix the syntax error while maintaining the Markdown format:

## REST API 

##### Base endpoint: [calculator.load.rs](https://calculator.load.rs)

### Load Network BaseLayer data

Storing data on Load Network via the baselayer route means that there are no restriction on the tx's input (OPCODEs, etc), tx's target or more properties, it's simply sending a tx the normal way on Load Network

```bash
GET /v1/baselayer/:data_size_in_bytes
```

### Load Network 0xbabe data

Data sent to Load Network as `0xbabe` transaction format (learn more about Bundler's 0xbabe tx format [here](https://github.com/weaveVM/bundler)) is more cost efficient and have optimizations on data size limit and reconstruction -- 0xbabe txs come with strict data input restrictions (no OPCODEs allowed, data should be strictly for data settlement purpose, following the Bundler data protocol specs)

```bash
GET /v1/babe/:data_size_in_bytes
```

### Celestia DA

```bash
GET /v1/celestia/:data_size_in_bytes
```

### Avail DA

```bash
GET /v1/avail/:data_size_in_bytes
```

## External pricing resources:
* https://docs.availproject.org/docs/learn-about-avail/tx-pricing#fees-calculation-for-da-transactions
* https://docs.celestia.org/how-to-guides/submit-data#fees-and-gas-limits


## License
This repository is licensed under the [MIT License](./LICENSE)