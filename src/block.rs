use crate::transaction::btree::BTree;
use block::block_service_server::BlockService;
use itertools::Itertools;
use prost::Message;
use std::io::{self, Read, Seek};
use std::sync::{Arc, Mutex};
use std::{fs, vec};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod block {
    tonic::include_proto!("stordy.block"); // The string specified here must match the proto package name
}

pub struct Block {
    db_block: Arc<Mutex<BTree>>,
    db_transaction: Arc<Mutex<BTree>>,
}

impl Block {
    pub fn new(db_block: Arc<Mutex<BTree>>, db_transaction: Arc<Mutex<BTree>>) -> Self {
        Block {
            db_block,
            db_transaction,
        }
    }
}

#[tonic::async_trait]
impl BlockService for Block {
    async fn add_block(
        &self,
        request: Request<block::Block>,
    ) -> Result<Response<block::Empty>, Status> {
        let block = request.into_inner();

        // println!("Received block: {:?}", block);

        if block.public_key.is_empty() {
            return Err(Status::invalid_argument("Public key is empty"));
        }

        let db = self.db_block.lock().unwrap();
        let id = db.get(block.public_key.clone());
        if id.is_some() {
            return Err(Status::already_exists("Block already exists"));
        }

        let uuid = Uuid::now_v7();
        // println!("UUID: {:?}", uuid.clone().to_string());
        // println!("UUID: {:?}", uuid.clone().to_string().as_bytes().to_vec());
        db.insert(
            block.public_key.clone(),
            uuid.to_string().as_bytes().to_vec(),
        );

        let mut buf_block = vec![];
        block.encode(&mut buf_block).unwrap();

        if buf_block.len() > 2u32.pow(16).try_into().unwrap() {
            return Err(Status::invalid_argument("Block too large"));
        }

        let block_size_buf = buf_block.len().to_be_bytes();
        let two_bytes_of_block_size = &block_size_buf.get(block_size_buf.len() - 2..).unwrap();
        let mut buf = Vec::new();

        buf.extend_from_slice(&two_bytes_of_block_size);
        buf.extend_from_slice(&buf_block);
        // buf.extend_from_slice(&two_bytes_of_block_size);

        let res = fs::write(format!("blocks/{}", uuid), buf);

        if res.is_err() {
            return Err(Status::internal("Error writing block to disk"));
        }

        Ok(Response::new(block::Empty {}))
    }

    async fn find_block(
        &self,
        request: Request<block::FindBlockRequest>,
    ) -> Result<Response<block::Block>, Status> {
        let request = request.into_inner();
        let db = self.db_block.lock().unwrap();
        let id = db.get(request.public_key.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());

        let mut file = fs::File::open(&filename).unwrap();
        file.seek(io::SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 2];
        file.read_exact(&mut buffer).unwrap();
        let block_size = u16::from_be_bytes(buffer);
        let mut buf = vec![0; block_size as usize];
        file.seek(io::SeekFrom::Start(2)).unwrap();
        file.read_exact(&mut buf).unwrap();
        let block = block::Block::decode(&*buf).unwrap();

        // println!("ID: {}", String::from_utf8(id.clone()).unwrap());
        // let buf = fs::read(format!("blocks/{}", String::from_utf8(id).unwrap())).unwrap();
        // let block_size = u16::from_be_bytes([buf[0], buf[1]]);
        // let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();
        //

        Ok(Response::new(block))
    }

    async fn length_block(
        &self,
        request: Request<block::LengthRequest>,
    ) -> Result<Response<block::LengthReply>, Status> {
        let request = request.into_inner();
        let public_key = request.public_key;

        let db = self.db_block.lock().unwrap();
        let id = db.get(public_key.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());

        let mut length = 0;

        let mut file = fs::File::open(&filename).unwrap();
        let size_of_file = file.metadata()?.len();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut size_of_header_buf = [0; 2];
        file.read_exact(&mut size_of_header_buf).unwrap();
        let size_of_header = u16::from_be_bytes(size_of_header_buf);
        let mut pointer = u64::from(size_of_header + 2);
        while pointer < size_of_file {
            file.seek(std::io::SeekFrom::Start(pointer as u64)).unwrap();
            let mut size_of_transaction_buf = [0; 2];
            file.read_exact(&mut size_of_transaction_buf).unwrap();
            let size_of_transaction = u16::from_be_bytes(size_of_transaction_buf);
            pointer += u64::from(size_of_transaction + 4);
            length += 1;
        }

        let response = block::LengthReply { length };

        Ok(Response::new(response))
    }

    async fn get_last_block(
        &self,
        _: Request<block::Empty>,
    ) -> Result<Response<block::Block>, Status> {
        let last_block = {
            let files = fs::read_dir("blocks").unwrap();

            files
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    file_name_str.split('-').count() == 5
                })
                .collect::<Vec<_>>() // Coletar os itens em um vetor para usar as funções do itertools
                .into_iter()
                .sorted_by(|a, b| a.file_name().cmp(&b.file_name()))
                .last()
                .unwrap()
        };

        if last_block.file_name().is_empty() {
            return Err(Status::not_found("Block not found"));
        }

        let buf = fs::read(format!(
            "blocks/{}",
            last_block.file_name().to_string_lossy()
        ))
        .unwrap();

        let block_size = u16::from_be_bytes([buf[0], buf[1]]);
        let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();

        Ok(Response::new(block))
    }

    async fn length(
        &self,
        _: Request<block::Empty>,
    ) -> Result<Response<block::LengthReply>, Status> {
        let db = self.db_block.lock().unwrap();
        let length = db.len();
        let response = block::LengthReply {
            length: length as u32,
        };

        Ok(Response::new(response))
    }

    async fn get_full_chain(
        &self,
        _: Request<block::Empty>,
    ) -> Result<Response<block::GetFullChainReply>, Status> {
        let db = self.db_block.lock().unwrap();
        let mut blocks = vec![];

        let iter = db.iter();
        for block in iter {
            let block = block.unwrap();
            // println!("Block: {:?}", block);
            let id = String::from_utf8(block.1.to_vec()).unwrap();
            // println!("ID: {}", id);
            let buf = fs::read(format!("blocks/{}", id)).unwrap();
            let block_size = u16::from_be_bytes([buf[0], buf[1]]);
            let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();
            blocks.push(block)
        }

        let response = block::GetFullChainReply { blocks };

        Ok(Response::new(response))
    }
}
