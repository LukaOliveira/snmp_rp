SNMP-RP
=======

A Python SNMP library developed in Rust with PyO3.

About
------------

SNMP-RP is intended to be an interface between `SNMP Rust libray <https://github.com/hroi/rust-snmp/>`__ and Python.
Bringing compiled language performance and optimization to Python.


Installation
------------
.. code:: python

   pip install snmp-rp

Examples
------------

GET

.. code:: python

   import snmp_rp

   snmp_oid       = '1.3.6.1.4.1.367.3.2.1.2.1.4.0'
   snmp_host      = '172.16.0.53'
   snmp_community = 'public'
   snmp_port      = '161'

   sys_descr = snmp_rp.get(snmp_oid, snmp_host, snmp_port, snmp_community)

   print(sys_descr)

Roadmap
-------

-  Add support for GETNEXT
-  Add support for GETBULK
-  Add support for SET

License
-------
MIT

