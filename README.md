
# SNMP-RP

Uma biblioteca SNMP Python desenvolvida em Rust utilizando PyO3.




## Documentação

O SNMP-RP é uma biblioteca com o objetivo de ser uma interface entre a biblioteca [SNMP](https://docs.rs/snmp/latest/snmp/) pertecente a linguagem Rust, e o Python, assim trazendo agilidade, desempenho e segurança de Rust e a simplicidade e praticidade de Python.






<a href="https://pypi.org/project/snmp-rp/0.3.0/">https://pypi.org/project/snmp-rp/0.3.0/</a>





## Uso/Exemplos



GET
```python
import snmp_rp

snmp_oid       = '1.3.6.1.4.1.367.3.2.1.2.1.4.0'
snmp_host      = '172.16.0.53'
snmp_community = 'public'
snmp_port      = '161'

sys_descr = snmp_rp.get(snmp_oid, snmp_host, snmp_port, snmp_community)

print(sys_descr)

```


## Roadmap

- Implementar GETNEXT
- Implementar GETBULK
- Implementar SET



## Licença

Distribuido sob a licença MIT [![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)


