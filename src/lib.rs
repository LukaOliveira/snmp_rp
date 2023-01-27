use pyo3::prelude::*;
use std::time::Duration;
use snmp::{SyncSession};


pub fn parse_oid(oid: &str) -> Vec<u32>{

    return oid.split('.').map(|n| n.parse().unwrap()).collect()

}


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

	for (_desc,val) in response.varbinds.next(){
		snmp_get = (format!("{:?}", val)).to_string();
		break;
	}

    Ok(snmp_get)

}

/// A Python module implemented in Rust.
#[pymodule]
fn snmp_rp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}



