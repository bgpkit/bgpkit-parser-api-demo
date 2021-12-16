# BGPKIT Parser REST API Demo

## Queries

End point: `http://localhost:8080/parser`

Parameters:
- `file`: required. path to the file to parse.
- `max`: optional. maximum number of messages to return. default 100.
- `asn`: optional. filter by origin asn.
- `prefix`: optional. filter by origin prefix.
- `msg_type`: optional. filter by message type, values can be "a" or "w".

## Run from Docker

`docker run --rm -p 8080:8080 bgpkit/bgpkit-parser-api-demo:latest`

## Example

Parse an update file and filter by ASN 15169:
``` 
➜  ~ curl --silent "http://localhost:8080/parse?file=https://spaces.bgpkit.org/parser/update-example.gz" |jq 
{
  "data": [
    {
      "aggr_asn": null,
      "aggr_ip": null,
      "as_path": "20912 15169",
      "atomic": "NAG",
      "communities": [
        "20912:65016"
      ],
      "elem_type": "ANNOUNCE",
      "local_pref": 0,
      "med": 0,
      "next_hop": "212.66.96.126",
      "origin": "IGP",
      "origin_asns": [
        15169
      ],
      "peer_asn": 20912,
      "peer_ip": "212.66.96.126",
      "prefix": "142.251.21.0/24",
      "timestamp": 1633046897.08933
    },
    {
      "aggr_asn": null,
      "aggr_ip": null,
      "as_path": "3561 3910 3356 15169",
      "atomic": "NAG",
      "communities": null,
      "elem_type": "ANNOUNCE",
      "local_pref": 0,
      "med": 0,
      "next_hop": "206.24.210.80",
      "origin": "IGP",
      "origin_asns": [
        15169
      ],
      "peer_asn": 3561,
      "peer_ip": "206.24.210.80",
      "prefix": "35.217.0.0/18",
      "timestamp": 1633047093.521516
    }
  ],
  "error": null
}
```
