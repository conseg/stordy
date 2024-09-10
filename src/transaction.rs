use prost::Message;
use std::convert::TryInto;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use transaction::transaction_service_server::TransactionService;

use self::btree::BTree;

#[path = "./btree.rs"]
pub mod btree;

pub mod transaction {
    tonic::include_proto!("stordy.transaction"); // The string specified here must match the proto package name
}

pub struct Transaction {
    db_block: Arc<Mutex<BTree>>,
    db_transaction: Arc<Mutex<BTree>>,
}

impl Transaction {
    pub fn new(db_block: Arc<Mutex<BTree>>, db_transaction: Arc<Mutex<BTree>>) -> Self {
        Transaction {
            db_block,
            db_transaction,
        }
    }
}

#[tonic::async_trait]
impl TransactionService for Transaction {
    async fn add_transaction(
        &self,
        request: Request<transaction::AddTransactionRequest>,
    ) -> Result<Response<transaction::Empty>, Status> {
        let request = request.into_inner();
        let mut transaction = request.transaction.unwrap();
        let block_public_key = request.block_public_key;


        let db_transaction = self.db_transaction.lock().unwrap();

        let qtd = match request.qtd {
            Some(x) => x,
            None => 1,
        };
        let db = self.db_block.lock().unwrap();
        let id = db.get(block_public_key.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let exists_transaction = {
            let buf = db_transaction.get(transaction.hash.clone());
            buf.is_some()
        };

        if exists_transaction {
            return Err(Status::already_exists("Transaction already exists"));
        }

        let mut buf = Vec::new();
        
        for i in 0..qtd {
            let mut buf_transaction = vec![];
            transaction.index += 1 as u32;
            transaction.encode(&mut buf_transaction).unwrap();

            if buf_transaction.len() > 2u32.pow(16).try_into().unwrap() {
                return Err(Status::invalid_argument("Transaction too large"));
            }

            let transaction_size_buf = buf_transaction.len().to_be_bytes();
            let two_bytes_of_transaction_size = &transaction_size_buf
                .get(transaction_size_buf.len() - 2..)
                .unwrap();

            buf.extend_from_slice(&two_bytes_of_transaction_size);
            buf.extend_from_slice(&buf_transaction);
            buf.extend_from_slice(&two_bytes_of_transaction_size);
        }
        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());

        let metadata = fs::metadata(filename.clone())?;
        let shift = metadata.len();

        let mut writer = OpenOptions::new()
            .append(true)
            .open(filename.clone())
            .unwrap();

        writer.write_all(&buf).unwrap();

        let mut buf = Vec::new();
        buf.extend_from_slice(&shift.to_be_bytes().to_vec());
        buf.extend_from_slice(&block_public_key.as_bytes().to_vec());

        db_transaction.insert(transaction.hash.clone(), buf);

        Ok(Response::new(transaction::Empty {}))
    }

    async fn find_last_transaction(
        &self,
        request: Request<transaction::FindLastTransactionRequest>,
    ) -> Result<Response<transaction::Transaction>, Status> {
        // let db_transaction = self.db_transaction.lock().unwrap();
        let db_block = self.db_block.lock().unwrap();
        let block_public_key = request.into_inner().block_public_key;
        let block_id = db_block.get(block_public_key.clone());

        if block_id.is_none() {
            return Err(Status::not_found("Block not found"));
        }
        // let block_id = block_id.unwrap();
        // let buf = fs::read(format!("blocks/{}", String::from_utf8(block_id).unwrap())).unwrap();
        // let size_last_transaction = u16::from_be_bytes([buf[buf.len() - 2], buf[buf.len() - 1]]);
        // let shift = buf.len() - 2 - size_last_transaction as usize;

        // let buf = buf.get(shift..buf.len() - 2).unwrap();
        // let transaction = transaction::Transaction::decode(buf).unwrap();
        //Ok(Response::new(transaction))
        let filename = format!(
            "blocks/{}",
            String::from_utf8(block_id.clone().expect("REASON")).unwrap()
        );

        // println!("find last transaction {:}", block_public_key.clone());
        //texto da documentação do FILE
        // An object providing access to an open file on the filesystem.
        // An instance of a File can be read and/or written depending on what options it was opened with.
        //Files also implement Seek to alter the logical cursor that the file contains internally.
        // Files are automatically closed when they go out of scope. Errors detected on closing are ignored by the implementation of Drop.
        //Use the method sync_all if these errors must be manually handled.
        //****File does not buffer reads and writes****.
        //For efficiency, consider wrapping the file in a BufReader or BufWriter when performing many small read or write calls,
        // unless unbuffered reads and writes are required.
        let mut arquivo = fs::File::open(&filename)?;
        //pega o tamanho do arquivo para poder ver quanto devemos voltar para pegar a ultima transacao
        let tamanho_arquivo = arquivo.metadata()?.len();

        arquivo.seek(io::SeekFrom::Start(0))?;
        let mut buffer = [0; 2];
        // le os primeiros 2 bytes do arquivo para o buffer
        arquivo.read_exact(&mut buffer)?;
        let size_header = u16::from_be_bytes(buffer);

        if (size_header + 2) as u64 == tamanho_arquivo {
            return Err(Status::not_found("Block is empty"));
        }

        // move o cursor para 2 bytes antes do final do arquivo
        arquivo.seek(io::SeekFrom::End(-2))?;
        let mut buffer = [0; 2];
        // le os ultimos 2 bytes do arquivo para o buffer
        arquivo.read_exact(&mut buffer)?;
        let size_last_transaction = u16::from_be_bytes(buffer);
        // pega o offset/shift da transacao
        let shift = (tamanho_arquivo - 2 - size_last_transaction as u64).max(0);
        // mcove o ponteiro do arquivo para o inicio da transacao
        arquivo.seek(io::SeekFrom::Start(shift))?;
        // cria um buffer do tamanho da  transação
        let mut buffer = vec![0; size_last_transaction as usize];
        // le a ultima transacao para o buffer
        arquivo.read_exact(&mut buffer)?;

        match transaction::Transaction::decode(&*buffer) {
            Ok(transaction) => Ok(Response::new(transaction)),
            Err(_) => Err(Status::internal("Failed to decode transaction")),
        }
    }

    async fn find_transaction_by_hash(
        &self,
        request: Request<transaction::FindTransactionByHashRequest>,
    ) -> Result<Response<transaction::Transaction>, Status> {
        let request = request.into_inner();
        let db = self.db_transaction.lock().unwrap();

        let buf = db.get(request.hash.clone());
        if buf.is_none() {
            return Err(Status::not_found("Transaction not found"));
        }

        let buf = buf.unwrap();
        let block_hash = String::from_utf8(buf.get(8..).unwrap().to_vec()).unwrap();

        let id = db.get(block_hash.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let shift = u64::from_be_bytes(buf.get(0..8).unwrap().try_into().unwrap());
        let filename = format!("blocks/{}", String::from_utf8(id.unwrap()).unwrap());
        let buf = fs::read(filename).unwrap();

        let transaction_size = u16::from_be_bytes([buf[shift as usize], buf[shift as usize + 1]]);
        // println!("transaction_size: {}", transaction_size);
        let transaction = transaction::Transaction::decode(
            &buf[shift as usize + 2..transaction_size as usize + shift as usize + 2],
        )
        .unwrap();

        Ok(Response::new(transaction))
    }

    async fn find_all_transactions(
        &self,
        request: Request<transaction::FindAllTransactionsRequest>,
    ) -> Result<Response<transaction::FindAllTransactionsReply>, Status> {
        let db = self.db_block.lock().unwrap();
        let request = request.into_inner();
        let block_id = db.get(request.block_public_key.clone());

        if block_id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let filename = format!("blocks/{}", String::from_utf8(block_id.unwrap()).unwrap());
        let buf = fs::read(filename).unwrap();

        let mut transactions: Vec<transaction::Transaction> = Vec::new();

        let mut it = 0;
        let mut is_block = true;
        while it < buf.len() {
            let transaction_size = u16::from_be_bytes([buf[it], buf[it + 1]]);
            if is_block {
                is_block = false;
                it += transaction_size as usize + 2;
                continue;
            }
            let transaction =
                transaction::Transaction::decode(&buf[it + 2..transaction_size as usize + it + 2])
                    .unwrap();
            transactions.push(transaction);
            it += transaction_size as usize + 4;
        }

        let response = transaction::FindAllTransactionsReply { transactions };

        Ok(Response::new(response))
    }

    async fn exists_transaction_on_block(
        &self,
        request: Request<transaction::ExistsTransactionOnBlockRequet>,
    ) -> Result<Response<transaction::ExistsTransactionOnBlockReply>, Status> {
        Ok(Response::new(transaction::ExistsTransactionOnBlockReply {
            exists: false,
        }))
    }
}
