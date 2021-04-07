use crate::imp::{core::*, prelude::*};
pub(crate) use tokio::sync::{broadcast, broadcast::error::TryRecvError};

pub trait EventEmitter {
    type Event: Clone;

    fn tx(&self) -> Option<broadcast::Sender<Self::Event>>;

    fn set_tx(&self, tx: broadcast::Sender<Self::Event>);

    fn new_tx(
        &self
    ) -> (
        broadcast::Sender<Self::Event>,
        broadcast::Receiver<Self::Event>
    ) {
        broadcast::channel(64)
    }

    fn subscribe_event(&self) -> broadcast::Receiver<Self::Event> {
        if let Some(tx) = self.tx() {
            tx.subscribe()
        } else {
            let (tx, rx) = self.new_tx();
            self.set_tx(tx);
            rx
        }
    }

    fn emit_event<E: Into<Self::Event>>(&self, e: E) { self.tx().map(|tx| tx.send(e.into()).ok()); }
}

pub(crate) trait Event: Clone {
    type EventType: Clone + Copy + PartialEq;

    fn event_type(&self) -> Self::EventType;
}

pub(crate) async fn expect_event<E: Event>(
    mut rx: broadcast::Receiver<E>,
    evt: E::EventType,
    timeout: u32
) -> Result<E, Error> {
    // consume
    loop {
        match rx.try_recv() {
            Err(TryRecvError::Empty) | Err(TryRecvError::Closed) => break,
            _ => {}
        }
    }
    let sleep = sleep(Duration::from_millis(timeout as u64));
    // TODO: Need spawning to prevent lagged?
    let event = async {
        loop {
            match rx.recv().await {
                Ok(x) if x.event_type() == evt => break Ok(x),
                Ok(_) => continue,
                Err(e) => break Err(Error::Event(e))
            }
        }
    };
    tokio::select! {
        _ = sleep => Err(Error::Timeout),
        x = event => x
    }
}

#[cfg(test)]
mod tests {
    crate::runtime_test!(select, {
        use crate::imp::prelude::*;
        let first = sleep(Duration::from_millis(200u64));
        let second = sleep(Duration::from_millis(400u64));
        tokio::select! {
            _ = first => {},
            _ = second => unreachable!()
        }
    });
}
