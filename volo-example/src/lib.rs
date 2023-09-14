#![feature(impl_trait_in_assoc_type)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use anyhow::anyhow;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use volo::FastStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use tokio::io;

// const DEFAULT_PROXY_ADDR: &str = "7777";

pub struct S {
	kv: Mutex<HashMap<String, String>>,
	pub channels: Mutex<HashMap<String, Sender<String>>>,
	pub file_name: String,
	pub aof: Mutex<File>,
	pub port: Mutex<String>,
	pub master: Option<String>,
	pub slaves: Mutex<Vec<String>>,
	pub cluster: Mutex<Vec<String>>
}

impl S {
	pub fn new(fname: String) -> S {
		S {
			kv: Mutex::new(HashMap::new()), 
			channels: Mutex::new(HashMap::new()), 
			file_name: fname.clone(),
			aof: Mutex::new(OpenOptions::new().write(true).create(true).append(true).open(fname.clone()).expect("Failed to open file")), 
			port: Mutex::new(String::new()),
			master: None,
			slaves: Mutex::new(vec![]),
			cluster: Mutex::new(vec![])
		}
	}
	pub fn init(&mut self) -> io::Result<()>{
		//std::mem::drop(self.aof.lock().unwrap());
		//self.aof = Mutex::new(File::open("temp.txt").expect("Failed to open"));
		let file = File::open(self.file_name.clone())?;
		let reader = BufReader::new(file);
		for line in reader.lines() {
			let line = line?;
			let mut parts = line.split('\t');

			let operator = parts.next().unwrap();
			println!("解析成功！{}", operator);
			let operand1 = parts.next().unwrap();
			println!("解析成功！");
			let operand2 = parts.next().unwrap();

			match operator {
				"set" => {
					self.kv.lock().unwrap().insert(operand1.to_string(), operand2.to_string());
				}
				"del" => {
					self.kv.lock().unwrap().remove(&operand1.to_string());
				}
				_ => {}
			}
		}
		//std::mem::drop(self.aof.lock().unwrap());
		//self.aof = Mutex::new(OpenOptions::new().write(true).create(true).append(true).open("temp.txt").expect("Failed to open file"));
		Ok(())
	}
	pub fn set_port(&mut self, port_: &str){
		self.port.lock().unwrap().push_str(port_);
	}
	pub fn set_master(&mut self, master_: String){
		self.master = Some(master_);
	}
	pub fn set_slave(&mut self, slave_: String) {
		self.slaves.lock().unwrap().push(slave_);
	}
	pub fn is_master(&self) -> bool {
		self.master.is_none() && self.cluster.lock().unwrap().is_empty()
	}
	pub fn is_slave(&self) -> bool {
		self.master.is_some()
	}
	pub fn is_proxy(&self) -> bool {
		!self.cluster.lock().unwrap().is_empty()
	}
	pub fn add_cluster(&self, serv: String) {
		self.cluster.lock().unwrap().push(serv);
	}
	pub fn distr_port(&self, mut key: String) -> String {	// distribute
		let sum: &mut u32 = &mut 0;
		let len = key.len();
		for i in 0..key.len() {
			let ch = key.remove(0);
			let sum = *sum + (ch as u32);
			if i == len - 1 {
				let cluster_size = self.cluster.lock().unwrap().len();
				let ind = sum as usize % cluster_size;
				return self.cluster.lock().unwrap()[ind].clone();
			}
		}
		"7777".to_string()
	}
}


#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get_item(&self, _req: volo_gen::volo::example::GetItemRequest) -> core::result::Result<volo_gen::volo::example::GetItemResponse, volo_thrift::AnyhowError> {
		let mut resp = volo_gen::volo::example::GetItemResponse{op: " ".into(), key: " ".into(), val: " ".into(), status: false};
		println!("\n收到指令");
		println!("cur_port: {}", self.port.lock().unwrap());
		let option = format!("{}\t{}\t{}\n", _req.op.to_string(), _req.key.to_string(), _req.val.to_string());
		println!("option is {}", option);
		let k = _req.key.to_string();
		let v = _req.val.to_string();
		//let mut test_file = OpenOptions::new().append(true).create(true).open("temp.txt").expect("Failed to open file");
		//let res = test_file.write_all(option.as_ref());
		//println!("{:?}", res);
		//let res = test_file.flush();
		//println!("{:?}", res);
		match _req.op.as_str() {
			"set" => {
				resp.op = "set".to_string().into();
				//let k = _req.key.to_string();
				//let v = _req.val.to_string();
				let flag = self.is_master();
				match flag {
					true => {
						self.kv.lock().unwrap().insert(k.clone(), v);
						//resp.val = v.clone().into();
						//resp.key = k.clone().into();
						resp.status = true;
						println!("Send to slaves");
						// let addr: SocketAddr = "127.0.0.1:22222".parse().unwrap();
						let slaves = self.slaves.lock().unwrap().clone();
						for port in slaves {
							let mut tmp_req = _req.clone();
							tmp_req.op = "mset".to_string().into();
							let addr = format!("127.0.0.1:{}", port);
							let addr: SocketAddr = addr.parse().unwrap();
							let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
								.address(addr)
								.build();
							let _res = sender.get_item(tmp_req).await;
						}
						self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
						println!("aof has been written!");
						self.aof.lock().unwrap().flush().expect("Err");
						println!("aof has been flushed!");
					}
					false => {
						resp.status = false;
					}
				}
				if self.is_proxy() {
					resp.status = true;
					let port = self.distr_port(k.clone());
					println!("distributed to port {}", port);
					let addr = format!("127.0.0.1:{}", port);
					let addr: SocketAddr = addr.parse().unwrap();
					let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
						.address(addr)
						.build();
					let _res = sender.get_item(_req).await;
				}
			}
			"mset" => {
				resp.op = "mset".to_string().into();
				//let k = _req.key.to_string();
				//let v = _req.val.to_string();
				self.kv.lock().unwrap().insert(k, v);
				//resp.val = v.clone().into();
				//resp.key = k.clone().into();
				resp.status = true;
				println!("sent as slave");
				// let addr: SocketAddr = "127.0.0.1:22222".parse().unwrap();
				// let slaves = self.slaves.lock().unwrap().clone();
				// for port in slaves {
				// 	let tmp_req = _req.clone();
				// 	let addr = format!("127.0.0.1:{}", port);
				// 	let addr: SocketAddr = addr.parse().unwrap();
				// 	let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
				// 		.address(addr)
				// 		.build();
				// 	let _res = sender.get_item(tmp_req).await;
				// }
				// self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
				// println!("aof has been written!");
				// self.aof.lock().unwrap().flush().expect("Err");
				// println!("aof has been flushed!");
			}
			"get" => {
				resp.op = "get".to_string().into();
				//let k = _req.key.to_string();
				if self.is_proxy() {
					let port = self.distr_port(k.clone());
					println!("distributed to port {}", port);
					let addr = format!("127.0.0.1:{}", port);
					let addr: SocketAddr = addr.parse().unwrap();
					let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
						.address(addr)
						.build();
					let res = sender.get_item(_req).await;
					if let Ok(info) = res {
						match info.status {
							true => {
								resp.val = info.val;
								resp.status = true;
							}
							false => {
								resp.status = false;
							}
						}
					}
				} else {
					match self.kv.lock().unwrap().get(&k)  {
						None => {
							resp.status = false;
						}
						Some(t) => {
							resp.val = t.clone().into();
							//resp.key = k.clone().into();
							resp.status = true;
							//self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
							// println!("aof has been written!");
							//self.aof.lock().unwrap().flush().expect("Err");
							// println!("aof has been flushed!");
						}
					}
				}
			}
			"del" => {
				resp.op = "del".to_string().into();
				//let k = _req.key.to_string();
				let flag = self.is_master();
				match flag {
					true => {
						let res = self.kv.lock().unwrap().remove(&k);
						match res.is_some() {
							true => {
								resp.status = true;
								println!("Send to slaves");
								let slaves = self.slaves.lock().unwrap().clone();
								for port in slaves {
									let mut tmp_req = _req.clone();
									tmp_req.op = "mdel".to_string().into();
									let addr = format!("127.0.0.1:{}", port);
									let addr: SocketAddr = addr.parse().unwrap();
									let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
										.address(addr)
										.build();
									let _res = sender.get_item(tmp_req).await;
								}
								self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
							}
							false => {
								resp.status = false;
							}
						}
					}
					false => {
						resp.status = false;
					}
				}
				if self.is_proxy() {
					let port = self.distr_port(k.clone());
					println!("distributed to port {}", port);
					let addr = format!("127.0.0.1:{}", port);
					let addr: SocketAddr = addr.parse().unwrap();
					let sender = volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
						.address(addr)
						.build();
					let res = sender.get_item(_req).await;
					if let Ok(info) = res {
						resp.status = info.status;
					}
				}
			}
			"mdel" => {
				resp.op = "mdel".to_string().into();
				self.kv.lock().unwrap().remove(&k);
				resp.status = true;
				println!("sent as slave");
			}
			"ping" => {
				resp.op = "ping".to_string().into();
				resp.status = true;
			}
			"subscribe" => {
				//let k = _req.key.to_string();
				let ( tx, mut rx) = broadcast::channel(16);
				resp.op = "subscribe".to_string().into();
				let mut is_exist = true;
				if let Some(tx) = self.channels.lock().unwrap().get(&k) {
					rx = tx.subscribe();
				}
				else {
					is_exist = false;
				}
				if !is_exist {
					self.channels.lock().unwrap().insert(k, tx);
				}
				let msg = rx.recv().await;
				match msg {
					Ok(m) => {
						resp.val = m.clone().into();
						resp.status = true;
						self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
					}
					Err(_e) => {
						resp.status = false;
					}
				}
			}
			"publish" => {
				resp.op = "publish".to_string().into();
				//let k = _req.key.to_string();
				match self.channels.lock().unwrap().get(&k) {
					Some(tx) => {
						match tx.send(v) {
							Ok(n) => {
								resp.status = true;
								resp.val = FastStr::from((n as u8).to_string());
								self.aof.lock().unwrap().write_all(option.as_ref()).expect("TODO: panic message");
							}
							Err(_e) => {
								resp.status = false;
							}
						}
					}
					None => {
						resp.status = false;
					}
				}
			}
			_ => {
				panic!("INVALID!");
			}
		}
		println!("处理完毕，送回");
		Ok(resp)
		//Ok(Default::default())
	}
}
pub struct FilterLayer;
impl<S> volo::Layer<S> for FilterLayer {
	type Service = FilterService<S>;

	fn layer(self, inner: S) -> Self::Service {
		FilterService(inner)
	}
}
#[derive(Clone)]
pub struct FilterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
	where
		Req: std::fmt::Debug + Send + 'static,
		S: Send + 'static + volo::Service<Cx, Req> + Sync,
		Cx: Send + 'static,
		anyhow::Error: Into<S::Error>,
{
	async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let info = format!("{req:?}");
		let mut dirty = false;
		if info.contains("原神") || info.contains("傻逼") || info.contains("操你妈") {
			dirty = true;
		}
		match dirty {
			true => {
				Err(anyhow!("你怎么骂人呢？给我刷了牙再来").into())
			}
			false => {
				let resp =self.0.call(cx, req).await;
				resp
			}
		}
	}
}