# RAC CLI -> Method Map (Generated)

Generated: 2026-02-26T00:04:24Z

- Cluster UUID: `1619820a-d36f-4d8a-a716-1516b1dea077`
- Manager UUID: `3985f906-ba9d-484f-aebc-3e1c6f1a8fe8`
- Server UUID: `6aa3a88a-9346-4499-8034-a4a72d7ee8e8`
- Process UUID: `f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5`

| Command | rac_exit | Session | c2s method IDs | s2c method IDs |
|---|---:|---|---|---|
| `cluster list` | 0 | `session_1771103982_389032_127_0_0_1_37378` | `0xb` | `0xc` |
| `cluster info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103983_389077_127_0_0_1_37392` | `0xd` | `0xe` |
| `cluster info --cluster 00000000-0000-0000-0000-000000000001` | 255 | `session_1772064219_1260833_127_0_0_1_52668` | `0xd` | `` |
| `agent version` | 0 | `session_1771103983_389122_127_0_0_1_37406` | `0x87` | `0x88` |
| `manager list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103984_389177_127_0_0_1_37414` | `0x9,0x12` | `0x13` |
| `server list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103985_389222_127_0_0_1_37426` | `0x9,0x16` | `0x17` |
| `process list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103985_389267_127_0_0_1_37442` | `0x9,0x1d,0x87` | `0x1e,0x88` |
| `infobase summary list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103986_389363_127_0_0_1_37454` | `0x9,0x2a` | `0x2b` |
| `connection list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103987_389408_127_0_0_1_37462` | `0x9,0x32` | `0x33` |
| `connection list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1772063927_1257962_127_0_0_1_34284` | `0x9,0x32` | `0x33` |
| `connection info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --connection 00000000-0000-0000-0000-000000000001` | 255 | `session_1772064185_1260371_127_0_0_1_54812` | `0x9,0x36` | `` |
| `session list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103987_389453_127_0_0_1_37466` | `0x9,0x41` | `0x42` |
| `session list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1772063905_1257551_127_0_0_1_43754` | `0x9,0x41` | `0x42` |
| `session list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --cluster-user cadmin --cluster-pwd badpwd` | 255 | `session_1772064183_1260268_127_0_0_1_54790` | `0x9` | `` |
| `session info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --session 00000000-0000-0000-0000-000000000001` | 255 | `session_1772064184_1260327_127_0_0_1_54798` | `0x9,0x45` | `` |
| `session info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --session not-a-uuid` | 255 | `session_1772064184_1260352_127_0_0_1_54810` | `0x9` | `` |
| `lock list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103988_389509_127_0_0_1_37480` | `0x9,0x48` | `0x49` |
| `lock list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --session 4851f0a9-ed90-4359-bc62-c36b926193c5` | 0 | `session_1772063918_1257736_127_0_0_1_38676` | `0x9,0x48` | `0x49` |
| `lock list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --infobase 717bdda7-2f60-4577-b262-f1fc8c0e472c` | 0 | `session_1772063157_1251829_127_0_0_1_45554` | `0x9,0x4a` | `0x4b` |
| `lock list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --connection 8b7739ee-c6c3-4890-b533-32632987433a` | 0 | `session_1772063148_1251710_127_0_0_1_55424` | `0x9,0x4c` | `0x4d` |
| `rule list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 255 | `session_1771103989_389578_127_0_0_1_37492` | `0x9` | `` |
| `profile list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103989_389623_127_0_0_1_37500` | `0x9,0x59` | `0x5a` |
| `service list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1772063081_1251004_127_0_0_1_53412` | `0x9,0x23` | `0x24` |
| `counter list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103990_389678_127_0_0_1_49366` | `0x9,0x76` | `0x77` |
| `limit list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077` | 0 | `session_1771103991_389723_127_0_0_1_49380` | `0x9,0x7c` | `0x7d` |
| `manager info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --manager 3985f906-ba9d-484f-aebc-3e1c6f1a8fe8` | 0 | `session_1771103991_389768_127_0_0_1_49396` | `0x9,0x14` | `0x15` |
| `server info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --server 6aa3a88a-9346-4499-8034-a4a72d7ee8e8` | 0 | `session_1771103992_389824_127_0_0_1_49406` | `0x9,0x18` | `0x19` |
| `process info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --process f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5` | 0 | `session_1771103993_389869_127_0_0_1_49412` | `0x9,0x1f,0x87` | `0x20,0x88` |
| `connection info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --connection 88763766-02aa-4cff-a896-6ddcbbd159eb` | 0 | `session_1771103994_389914_127_0_0_1_49422` | `0x9,0x36` | `0x37` |
| `session info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --session 25510e27-f24a-4586-9ac9-9f7837c0dea1` | 0 | `session_1771103994_389974_127_0_0_1_49430` | `0x9,0x45` | `0x46` |
| `infobase info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --infobase 717bdda7-2f60-4577-b262-f1fc8c0e472c ` | 0 | `session_1771103995_390019_127_0_0_1_49436` | `0x9,0xa,0x30` | `0x31` |
| `infobase summary info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --infobase 717bdda7-2f60-4577-b262-f1fc8c0e472c` | 0 | `session_1771103996_390065_127_0_0_1_49450` | `0x9,0x2e` | `0x2f` |
| `infobase info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --infobase 00000000-0000-0000-0000-000000000001` | 255 | `session_1772064185_1260408_127_0_0_1_54824` | `0x9,0xa,0x30` | `` |
