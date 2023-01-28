use pyo3::prelude::*;
use std::time::Duration;
use snmp::{SyncSession};



/* 
Convert a OID &str to Vec<u32>

Input - 1.3.6.367.5.1
Ouput - [1,3,6,367,5,1]

*/
pub fn parse_oid(oid: &str) -> Vec<u32>{

    return oid.split('.').map(|n| n.parse().unwrap()).collect()

}


/* 
Call the host

Input  - OID, Host (IP), SNMP Port, SNMP Community
Output - String containing the OID return type and value

*/
#[pyfunction]
fn get(oid: &str, host: &str, port: &str, community: &str) -> PyResult<String>{

    let oid_converted = &parse_oid(oid);

    let _sys_descr_oid  = oid_converted;
	let community       = community.as_bytes();
	let timeout         = Duration::from_secs(2);
  	      	
	let ip 		   = host.to_string();
	let port: &str = port;

	let c 	 = String::from(ip + ":" + port);  
	let oid  = _sys_descr_oid.clone();
		
	let mut snmp_get = String::new();
	let mut sess 	 = SyncSession::new(c, community, Some(timeout), 0).unwrap();

	let mut response = sess.get(&oid).unwrap();	

	if let Some((_desc,val)) = response.varbinds.next(){
		snmp_get = (format!("{:?}", val)).to_string();
	}

    Ok(snmp_get)

}


#[pymodule]
fn snmp_rp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}



