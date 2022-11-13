use std::{
    collections::HashMap,
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use rand::{thread_rng, Rng as _};
use tokio::sync::{mpsc, oneshot};

pub type ConnId = usize;
pub type Msg = String;

#[derive(Debug)]
enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
    },

    Disconnect {
        conn: ConnId,
    },

    Message {
        msg: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },
}

#[derive(Debug)]
pub struct WsServer {
    sessions: HashMap<usize, mpsc::UnboundedSender<Msg>>,
    session_count: Arc<AtomicUsize>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl WsServer {
    pub fn new() -> (Self, WsServerHandle) {
        
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                session_count: Arc::new(AtomicUsize::new(0)),
                cmd_rx,
            },
            WsServerHandle { cmd_tx },
        )
    }

    async fn connect(&mut self, tx: mpsc::UnboundedSender<String>) -> ConnId {
        println!("user connected");

        let id = thread_rng().gen::<usize>();
        self.sessions.insert(id, tx);

        let count = self.session_count.fetch_add(1, Ordering::SeqCst);

        id
    }

    async fn disconnect(&mut self, conn_id: ConnId) {
        println!("user disconnected");
        if self.sessions.remove(&conn_id).is_some() {
            self.sessions.remove(&conn_id);
        }
        self.session_count.fetch_sub(1, Ordering::SeqCst);
    }

    async fn send_message(&self, conn: ConnId, msg: impl Into<String>) {

    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::Message { msg, conn, res_tx } => {
                    self.send_message(conn, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }

}

#[derive(Debug, Clone)]
pub struct WsServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>
}

impl WsServerHandle {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<String>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::Connect { conn_tx, res_tx }).unwrap();
        res_rx.await.unwrap()
    }

    pub fn disconnect(&self, conn: ConnId) {
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
}

